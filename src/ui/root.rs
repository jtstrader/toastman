use ratatui::{
    layout::Offset,
    prelude::*,
    symbols::border,
    widgets::{Block, Tabs},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main window
        let title = Line::from("Toastman".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);
        block.render(area, buf);

        // Tabs
        let tabs = Tabs::new(self.workspaces.iter().map(|w| w.root.name.clone()))
            .style(Color::White)
            .highlight_style(Style::default().green().bold())
            .select(self.selected_workspace)
            .divider(symbols::DOT)
            .padding(" ", " ");
        tabs.render(area + Offset::new(1, 0), buf);
    }
}
