use ratatui::{style::{Style, palette::material::PINK}, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Widget}};

use crate::colors::IRIS;

struct Page{
    id: u8,
    name: String,
    abbr: String
}

impl Page {
    pub fn new(id: u8, name: impl Into<String>, abbr: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            abbr: abbr.into(),
        }
    }
}

pub struct PageIndicator{
    pub current: u8,
    pages: Vec<Page>,
}

impl Default for PageIndicator {
    fn default() -> Self {
        Self {
            current: 1,
            pages: vec![
                Page::new(1, "Home", "H"),
                Page::new(2, "Logs", " L"),
                Page::new(3, "Settings", " S"),
            ],
        }
    }
}

impl Widget for &PageIndicator {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        let mut page_span_vec = vec![];
        for page in &self.pages{
            let span = if self.current == page.id {
                Span::styled(&page.abbr, Style::new().fg(IRIS))
            } else {
                Span::raw(&page.abbr)
            };
            page_span_vec.push(span);
        }
        Paragraph::new(Line::from(page_span_vec)).render(area, buf);
    }
}
