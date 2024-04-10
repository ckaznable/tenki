use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::Widget,
};

pub struct FpsWidget(pub usize);

impl Widget for FpsWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [_, fps_area, _] = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(2),
            ],
        )
        .areas(area);

        Line::styled(self.0.to_string(), Style::new().green()).render(fps_area, buf);
    }
}
