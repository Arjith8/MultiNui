use ratatui::{text::{Line, Text}, widgets::{List, Widget}};
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
}

impl Widget for Goal{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        Text::from(self.name).render(area, buf);
    }
}
