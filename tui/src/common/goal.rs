use ratatui::{layout::{Constraint, Layout}, text::{Line, Span, Text}, widgets::{Block, List, Paragraph, Widget}};
use uuid::Uuid;

use crate::widgets::status::Status;

pub struct Goal {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sub_goals: Vec<Goal>,
    pub status: Status
}

impl Goal {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            sub_goals: vec![],
            status: Status::default()
        }
    }
    pub fn widget(&self) -> Line<'static>{
        Line::from(vec![
            Span::raw(" "),
            self.status.widget(),
            Span::raw(" "),
            Span::raw(self.name.clone())
        ])
    }
}

impl Default for Goal{
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4(), 
            name: "You have not added any goals yet, to add press \"n\"".into(), 
            description: None,
            sub_goals: vec![],
            status: Status::WIP
        }
    }
}

impl Widget for Goal{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        Line::from(vec![
            Span::raw(" "),
            self.status.widget(),
            Span::raw(" "),
            Span::raw(self.name)
        ]).render(area, buf);
    }
}
