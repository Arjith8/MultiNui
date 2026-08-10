use ratatui::widgets::Widget;
use uuid::Uuid;

use crate::widgets::accordion::Accordion;

pub struct Goal {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub struct GoalSheet {
    pub id: Uuid,
    pub name: String,
    pub goals: Vec<Goal>
}

impl GoalSheet {
    pub fn get_all() -> Vec<GoalSheet> {
        vec![
            GoalSheet {
                id: Uuid::new_v4(),
                name: "ML".into(),
                goals: vec![
                    Goal {
                        id: Uuid::new_v4(),
                        name: "Implement AdamW".into(),
                        description: "Implement AdamW from scratch".into(),
                    },
                    Goal {
                        id: Uuid::new_v4(),
                        name: "Build Sequential".into(),
                        description: "Implement a Sequential container".into(),
                    },
                ],
            },
            GoalSheet {
                id: Uuid::new_v4(),
                name: "Rust".into(),
                goals: vec![
                    Goal {
                        id: Uuid::new_v4(),
                        name: "Learn Ratatui".into(),
                        description: "Build a TUI with Ratatui".into(),
                    },
                ],
            },
        ]
    }
    fn new(name: String) -> Self {
        let id: Uuid = Uuid::new_v4();
        Self { id,  name, goals: vec![] }
    }
}

impl Widget for GoalSheet {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    {
        Accordion::new(self.name, 0)
            .render(area, buf);
    }
    
}
