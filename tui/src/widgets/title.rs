use ratatui::{
    layout::{Alignment, Constraint, Layout}, prelude::{Buffer, Rect, Style}, style::Stylize, widgets::{Block, Widget},
};

use crate::{colors::{IRIS, LOVE, ROSE}, utils::padding::add_padding, widgets::page::PageIndicator};

pub struct TitleBar<'a> {
    text: String,
    page_indicator: &'a PageIndicator
}

impl <'a> TitleBar <'a> {
    pub fn new(text: impl Into<String>, page_indicator: &'a PageIndicator) -> Self {
        Self { 
            text: text.into(),
            page_indicator
        }
    }
}

impl <'a> Widget for &TitleBar <'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
        ])
        .split(area);

        self.page_indicator
            .render(chunks[0], buf);
        
        let padded_title = add_padding(1, vec![self.text.to_string()])
                            .pop()
                            .unwrap();


        Block::new()
            .title(padded_title.fg(IRIS))
            .title_alignment(Alignment::Right)
            .render(chunks[1], buf);

        Block::new()
            .style(Style::new().bg(LOVE))
            .render(chunks[2], buf);
        Block::new()
            .style(Style::new().bg(IRIS))
            .render(chunks[3], buf);
        Block::new()
            .style(Style::new().bg(ROSE))
            .render(chunks[4], buf);
    }
}
