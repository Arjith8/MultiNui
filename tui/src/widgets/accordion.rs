use ratatui::{text::Line, widgets::Widget};

use crate::utils::padding;

pub enum AccordionItem {
    Text(String),
    Section(Accordion),
}

pub enum Status {
    Complete,
    Pending,
    WIP
}

pub struct Accordion{
    title: String,
    content: Option<Vec<AccordionItem>>,
    is_open: bool,
    level: u8,
    status: Status
}

impl Accordion {
    fn new(self, title: String, level: u8 ) -> Self{
        return Self { title, content: None, is_open: false, level: level, status: Status::Pending }
    }
}

impl Widget for Accordion {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        let title_string = padding::add_padding((self.level as usize) * 4, vec![self.title]);
        let title = Line::from(title_string.first());
    }
    
}
