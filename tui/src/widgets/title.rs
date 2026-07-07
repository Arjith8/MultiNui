use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::{Buffer, Rect, Style},
    style::Stylize,
    widgets::{Block, Widget},
};

use crate::colors::{IRIS, LOVE, ROSE};

#[derive(Debug)]
pub struct TitleBar {
    text: String,
}

impl TitleBar {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
    pub fn default() -> Self {
        Self {
            text: " MultiNui Goals Manager ".into(),
        }
    }
}

impl Widget for &TitleBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
        ])
        .split(area);

        Block::new()
            .title(self.text.as_str().fg(IRIS))
            .title_alignment(Alignment::Right)
            .render(chunks[0], buf);

        Block::new()
            .style(Style::new().bg(LOVE))
            .render(chunks[1], buf);
        Block::new()
            .style(Style::new().bg(IRIS))
            .render(chunks[2], buf);
        Block::new()
            .style(Style::new().bg(ROSE))
            .render(chunks[3], buf);
    }
}
