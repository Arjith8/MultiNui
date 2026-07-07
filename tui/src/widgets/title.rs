use ratatui::{
    layout::Alignment, prelude::{Buffer, Rect, Style}, style::Stylize, widgets::{Block, Widget},
};

use crate::colours::IRIS;

#[derive(Debug)]
pub struct TitleBar {
    text: String,
}

impl TitleBar {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
    pub fn default() -> Self {
        Self { text: " MultiNui Goals Manager ".into() }
    }
}

impl Widget for &TitleBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .title(self.text.as_str().fg(IRIS))
            .title_alignment(Alignment::Right)
            .render(area, buf);
    }
}
