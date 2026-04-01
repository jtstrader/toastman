use ratatui::{
    prelude::{Buffer, Rect, Widget},
    style::Style,
    symbols::border,
    widgets::{Block, Paragraph},
};

use crate::app::Request;

impl Widget for &Request {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let panel = Block::bordered().title("Request").border_set(border::THICK);
        let inner_area = panel.inner(area);
        panel.render(area, buf);

        let p = Paragraph::new(self.endpoint.clone()).style(Style::new().bold());
        p.render(inner_area, buf);
    }
}
