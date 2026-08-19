use ratatui::{style::{Color::{self, Gray, Green, Yellow}, Style}, text::Span, widgets::Widget};

#[derive(Clone)]
pub enum Status {
    Complete,
    Pending,
    WIP
}

impl Default for Status{
    fn default() -> Self {
        Self::Pending
    }
}

impl Status {
    pub fn widget(&self) -> Span<'static> {
        match self {
            Self::Complete => Span::styled("●", Style::default().fg(Green)),
            Self::Pending => Span::styled("●", Style::default().fg(Yellow)),
            Self::WIP => Span::styled("●", Style::default().fg(Gray)),
        }
    }
    
}

impl Widget for Status {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        self.widget().render(area, buf);
    }
}
