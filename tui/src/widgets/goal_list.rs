use ratatui::{text::Line, widgets::{List, ListState, StatefulWidget}};
use uuid::Uuid;

use crate::{common::goal::Goal, widgets::status::Status};

pub struct GoalList {
    goals: Vec<Goal>
}

impl Default for GoalList {
    fn default() -> Self {
        Self { 
            goals: vec![
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
            ]
        }
    }
}

impl StatefulWidget for GoalList{
    type State = ListState;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let all_goals: Vec<Line> = self.goals
            .into_iter()
            .map(|goal| goal.widget())
            .collect();
        List::new(all_goals).render(area, buf, state);
    }
}
