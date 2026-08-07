use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::{Tabs, Widget}};
use uuid::Uuid;

use crate::{App, colors::ROSE, common::goal::GoalSheet, utils::padding::add_padding};

pub struct GoalTab {
    current_goal_tab: usize,
    goal_tabs: Vec<String>
}

impl GoalTab {
    pub fn new(current_goal_tab: usize, goal_tabs: Vec<String>) -> Self{
        return Self {
            current_goal_tab: current_goal_tab,
            goal_tabs: goal_tabs
        };
    }
}

impl Widget for GoalTab {
    fn render(self, area: Rect, buf: &mut Buffer){
        //let goal_tab_names: Vec<String> = self.goal_tabs.iter().map(|tab| tab.name.clone()).collect();
        let goal_tab_names: Vec<String> = self.goal_tabs;

        Tabs::new(goal_tab_names)
            .style(Style::default())
            .highlight_style(Style::default().bg(ROSE).fg(ratatui::style::Color::Black))
            .select(self.current_goal_tab)
            .divider(" ")
            .padding_left("")
            .render(area, buf);
    }
}
