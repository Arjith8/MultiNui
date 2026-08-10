use std::time::Instant;

use ratatui::{layout::{Constraint, Layout}, style::Stylize, widgets::{Block, Widget}};

use crate::colors::{IRIS, ROSE};

pub struct BottomBar<'a> {
    leader_until: &'a Option<Instant>,
}

impl <'a> BottomBar<'a> {
    pub fn new(leader_until: &'a Option<Instant>) -> Self{
        Self {
            leader_until
        }
    }
}

impl <'a> Widget for BottomBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        let chunks = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Fill(1)
        ]).split(area);
        let mut color = IRIS;
        if self.leader_until.is_some_and(|until| until >= Instant::now() ){
            color = ROSE;
        }
        Block::new()
            .bg(color)
            .render(chunks[0], buf);
    }
}
