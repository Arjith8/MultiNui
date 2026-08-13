use ratatui::{style::{Color, Style, Stylize}, text::Line, widgets::{List, ListDirection, ListState, StatefulWidget}};
use uuid::Uuid;

use crate::{common::goal::Goal, widgets::status::Status};

pub struct GoalList {
    goals: Option<Vec<Goal>>,
    is_active: bool
}

impl Default for GoalList{
    fn default() -> Self {
        return Self { goals: Some(vec![Goal::default()]), is_active: false }
    }
}

impl GoalList {
    pub fn new(is_active: bool) -> Self {
        Self { goals: None, is_active: is_active }
    }
    pub fn fetch_goals() -> Option<Vec<Goal>>{
        return Some(vec![
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
    }
    
}

impl StatefulWidget for GoalList{
    type State = ListState;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let goals: Vec<Line> = GoalList::fetch_goals()
            .unwrap_or_default()
            .into_iter()
            .map(|goal| goal.widget())
            .collect();
        if self.is_active {
            state.select_first();
        } else {
            state.select(None);
        }
        List::new(goals)
            .style(Color::White)
            .highlight_style(Style::new().yellow().italic())
            .highlight_symbol("> ".red())
            .scroll_padding(1)
            .repeat_highlight_symbol(true)
            .render(area, buf, state);
    }
}
