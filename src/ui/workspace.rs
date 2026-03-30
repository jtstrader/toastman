use ratatui::{prelude::*, widgets::Paragraph};

use crate::app::Workspace;

impl Widget for &Workspace {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new(self.root.name.clone());
        p.render(area, buf);
    }
}
