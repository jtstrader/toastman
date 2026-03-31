use ratatui::{
    layout::Offset,
    prelude::*,
    symbols::border,
    widgets::{Block, ListState, Tabs},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        assert!(self.selected_workspace < self.workspaces.len());

        // Main window
        let title = Line::from("Toastman".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Tabs
        let tabs = Tabs::new(self.workspaces.iter().map(|w| w.name.clone()))
            .style(Color::White)
            .highlight_style(Style::default().green().bold())
            .select(self.selected_workspace)
            .divider(symbols::DOT)
            .padding(" ", " ");
        tabs.render(area + Offset::new(1, 0), buf);

        // Render workspace window
        let mut state = ListState::default().with_selected(Some(0));
        self.workspaces[self.selected_workspace].render(inner_area, buf, &mut state);
    }
}
