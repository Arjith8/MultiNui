use ratatui::{style::{Color, Style, Stylize}, text::Line, widgets::{List, ListState, StatefulWidget}};
use uuid::Uuid;

use crate::{common::goal::Goal, widgets::status::Status};

#[derive(Clone)]
pub struct GoalList {
    pub goals: Option<Vec<Goal>>,
    pub current_idx: usize
}

impl Default for GoalList{
    fn default() -> Self {
        Self { goals: Some(vec![Goal::default()]), current_idx:0 }
    }
}

impl GoalList {
    pub fn fetch_goals() -> GoalList{
        GoalList{ current_idx:0, goals: Some(vec![
            Goal {
                id: Uuid::new_v4(),
                name: "ML".into(),
                description: None,
                status: Status::WIP,
                sub_goals: vec![
                    Goal {
                        id: Uuid::new_v4(),
                        name: "Implement AdamW".into(),
                        description: Some("Implement AdamW from scratch".into()),
                        status: Status::Pending,
                        sub_goals: vec![],
                    },
                    Goal {
                        id: Uuid::new_v4(),
                        name: "Build Sequential".into(),
                        description: Some("Implement a Sequential container".into()),
                        status: Status::Complete,
                        sub_goals: vec![],
                    },
                ],
            },
            Goal {
                id: Uuid::new_v4(),
                name: "Rust".into(),
                description: None,
                status: Status::WIP,
                sub_goals: vec![
                    Goal {
                        id: Uuid::new_v4(),
                        name: "Learn Ratatui".into(),
                        description: Some("Build a TUI with Ratatui".into()),
                        status: Status::WIP,
                        sub_goals: vec![],
                    },
                ],
            },
        ])
    }}
}

pub struct GoalListWidget<'a>{
    goals: &'a GoalList
}

impl<'a> GoalListWidget<'a> {
    pub fn new(goals: &'a GoalList) -> Self {
        Self { goals }
    }
}


impl <'a> StatefulWidget for GoalListWidget<'a>{
    type State = ListState;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let goals: Vec<Line> = self.goals
            .goals
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|goal| goal.widget())
            .collect();

        List::new(goals)
            .style(Color::White)
            .highlight_style(Style::new().yellow().italic())
            .scroll_padding(1)
            .repeat_highlight_symbol(true)
            .render(area, buf, state);
    }
}
