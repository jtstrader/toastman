use std::{collections::HashMap, io};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

/// The main application state.
#[derive(Debug)]
pub struct App {
    /// A series of top-level folders (workspaces) that are isolated and have their own
    /// environments.
    pub workspaces: Vec<Workspace>,

    exit: bool,

    /// The workspace that is currently in view.
    pub selected_workspace: usize,
}

impl App {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            workspaces: vec![Workspace {
                environment: HashMap::new(),
                root: Folder {
                    name: "Folder1".into(),
                    folders: HashMap::new(),
                    requests: Vec::new(),
                },
            }],
            exit: false,
            selected_workspace: 0,
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
    /// The environment that applies to all subfolders/requests of the workspace.
    pub environment: HashMap<String, String>,

    /// The entry point of the workspace.
    pub root: Folder,
}

/// A folder is a named container that can have requests or more folders within.
#[derive(Debug)]
pub struct Folder {
    /// The name of the folder.
    pub name: String,

    /// Subfolders that can contain additional subfolders/requests.
    pub folders: HashMap<u16, Folder>,

    /// A series of requests associated with this folder.
    pub requests: Vec<Request>,
}

/// A request object.
#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    base_url: String,
    endpoint: String,
    query_params: HashMap<String, String>,
    path_params: HashMap<String, String>,
}

/// An HTTP request method.
#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}
