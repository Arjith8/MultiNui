use ratatui::{style::{Color, Style}, text::Span, widgets::{Block, BorderType, Borders, Widget}};

pub enum Status {
    Complete,
    Pending,
    WIP
}

impl Widget for Status {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        let color = match self {
            Self::Complete => Color::Green,
            Self::Pending => Color::Yellow,
            Self::WIP => Color::Gray,
        };

        Span::styled("⬤ ", Style::default().fg(color))
            .render(area, buf);
    }
}
