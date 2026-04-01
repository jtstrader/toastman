use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, StatefulWidget, Widget},
    symbols::border,
    widgets::{Block, List, ListDirection, ListState},
};

use crate::app::{Request, Workspace};

impl StatefulWidget for &Workspace {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Sidebar layout
        let [sidebar_layout, request_layout] = area.layout(&Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]));

        // Sidebar container
        let sidebar_title = "APIs";
        let sidebar = Block::bordered()
            .title(sidebar_title)
            .border_set(border::THICK);
        let inner_area = sidebar.inner(area);

        // Sidebar items
        let items = self.requests.iter().map(Request::to_sidebar_display);
        let sidebar_items = List::new(items)
            .scroll_padding(1)
            .direction(ListDirection::TopToBottom);

        // Render sidebar
        sidebar.render(sidebar_layout, buf);
        <List as StatefulWidget>::render(sidebar_items, inner_area, buf, state);

        // Render request in view
        if let Some(request_idx) = state.selected() {
            self.requests[request_idx].render(request_layout, buf);
        }
    }
}
