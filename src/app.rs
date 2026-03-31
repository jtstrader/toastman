use std::{collections::HashMap, fmt::Display, io};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::Color,
    text::{Line, Span},
    widgets::ListState,
};

/// A unique ID for a workspace.
type WorkspaceId = usize;

/// The main application state.
#[derive(Debug)]
pub struct App {
    /// A series of top-level folders (workspaces) that are isolated and have their own
    /// environments and requests.
    pub workspaces: Vec<Workspace>,

    /// The workspace that is currently in view.
    pub selected_workspace: WorkspaceId,

    /// The currently selected request for the given workspace.
    pub selected_request_state: HashMap<WorkspaceId, ListState>,

    exit: bool,
}

impl App {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            workspaces: vec![Workspace {
                name: "Default".to_owned(),
                environment: HashMap::new(),
                requests: vec![
                    Request {
                        method: RequestMethod::Get,
                        base_url: "http://localhost:5000".to_owned(),
                        endpoint: "api/v1/healthy".to_owned(),
                        ..Default::default()
                    },
                    Request {
                        method: RequestMethod::Post,
                        base_url: "http://localhost:5000".to_owned(),
                        endpoint: "api/v1/create".to_owned(),
                        ..Default::default()
                    },
                    Request {
                        method: RequestMethod::Put,
                        base_url: "http://localhost:5000".to_owned(),
                        endpoint: "api/v1/put".to_owned(),
                        ..Default::default()
                    },
                    Request {
                        method: RequestMethod::Patch,
                        base_url: "http://localhost:5000".to_owned(),
                        endpoint: "api/v1/patch".to_owned(),
                        ..Default::default()
                    },
                    Request {
                        method: RequestMethod::Delete,
                        base_url: "http://localhost:5000".to_owned(),
                        endpoint: "api/v1/delete".to_owned(),
                        ..Default::default()
                    },
                ],
            }],
            exit: false,
            selected_workspace: 0,
            selected_request_state: HashMap::new(),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<(), io::Error> {
        if let KeyCode::Char('q') = key_event.code {
            self.exit = true;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Workspace {
    /// The name of the workspace.
    pub name: String,

    /// The environment that applies to all subfolders/requests of the workspace.
    pub environment: HashMap<String, String>,

    /// A series of requests associated with this folder.
    pub requests: Vec<Request>,
}

/// A request object.
#[derive(Debug, Default)]
pub struct Request {
    pub method: RequestMethod,
    pub base_url: String,
    pub endpoint: String,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
}

impl Request {
    /// Emit the request as a user-facing display item.
    pub fn to_sidebar_display(&self) -> Line<'_> {
        let span: Span = self.method.into();

        Line::from_iter([
            span,
            " ".repeat(self.method.get_required_padding()).into(),
            self.endpoint.clone().into(),
        ])
    }
}

/// An HTTP request method.
#[derive(Debug, Default, Copy, Clone)]
pub enum RequestMethod {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl RequestMethod {
    /// Get the padding for the method. For the longest method, padding will be 0.
    pub fn get_required_padding(&self) -> usize {
        match self {
            RequestMethod::Get => 4,
            RequestMethod::Post => 3,
            RequestMethod::Put => 4,
            RequestMethod::Patch => 2,
            RequestMethod::Delete => 1,
        }
    }
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Patch => "PATCH",
                Self::Delete => "DELETE",
            }
        )
    }
}

impl<'a> From<RequestMethod> for Span<'a> {
    fn from(value: RequestMethod) -> Self {
        let color = match value {
            RequestMethod::Get => Color::Green,
            RequestMethod::Post => Color::Blue,
            RequestMethod::Put => Color::Yellow,
            RequestMethod::Patch => Color::Cyan,
            RequestMethod::Delete => Color::Red,
        };

        Span::styled(value.to_string(), color)
    }
}
