use std::{io, time::{Duration, Instant}};

use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, layout::{Constraint, Layout}, widgets::{Block, Borders, ListState, StatefulWidget, Widget},
};
use ratatui_comfy_toaster::{ToastEngine, ToastEngineBuilder, ToastMessage};

use crate::{utils::padding::add_padding, widgets::{bottom_bar::BottomBar, goal_list::{GoalList, GoalListWidget}, goals_tab::GoalTab, page::PageIndicator, title::TitleBar}};

mod colors;
mod widgets;
mod utils;
mod common;

struct App {
    leader_until: Option<Instant>,
    exit: bool,
    toast_engine: Option<ToastEngine<ToastMessage>>,
    page_indicator: PageIndicator,
    current_tab: usize,
    goal_view_active: bool,
    goals: GoalList
}

impl App {
    pub fn new() -> Self{
        let goals = GoalList::fetch_goals();
        Self { 
            exit: false,
            leader_until: None,
            toast_engine: None,
            page_indicator: PageIndicator::default(),
            current_tab: 0,
            goal_view_active: true,
            goals
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        if self.toast_engine.is_none(){
            self.toast_engine = Some(ToastEngineBuilder::new(frame.area()).build());
        }
        frame.render_widget(&*self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn is_leader_active(&self) -> bool{
        self.leader_until.is_some_and(|until| until >= Instant::now())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let (KeyCode::Char('a'), KeyModifiers::CONTROL) = (key_event.code, key_event.modifiers) {
            self.leader_until = Some(Instant::now() + Duration::from_secs(2));
        }
        if self.is_leader_active(){
            println!("{:?}", key_event);
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('h') => self.page_indicator.current = 1,
                KeyCode::Char('s') => self.page_indicator.current = 2,
                KeyCode::Char('g') => {
                    self.goal_view_active = !self.goal_view_active
                },
                KeyCode::Char(c @ '1'..='9') => {
                    let page: usize = c.to_digit(10).unwrap() as usize - 1;
                    self.current_tab = page
                }
                _ => {}
            }
        }
        match key_event.code {
            KeyCode::Up => {
                if self.goal_view_active{
                    self.goals.current_idx = self.goals.current_idx.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if self.goal_view_active && self.goals.goals.clone().unwrap_or(vec![]).len() - 1 > self.goals.current_idx {
                    self.goals.current_idx += 1;
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let chunks = ratatui::prelude::Layout::vertical([
            ratatui::prelude::Constraint::Length(1),
            ratatui::prelude::Constraint::Fill(1),
            ratatui::prelude::Constraint::Length(1),
        ]).split(area);
        
        TitleBar::new("MultiNui Goals Manager", &self.page_indicator)
            .render(chunks[0], buf);

        let goal_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1)
        ]).split(chunks[1]);

        GoalTab::new(self.current_tab, add_padding(2, vec!["Tab 1".into(), "Tab2".into()]))
            .render(goal_chunks[0], buf);

        let block = Block::new()
            .borders(Borders::ALL);

        let inner = block.inner(goal_chunks[1]);
        let mut state = ListState::default();
        state.select(Some(self.goals.current_idx));
        let goal_widget = GoalListWidget::new(&self.goals);
        goal_widget.render(inner, buf, &mut state);

        block.render(goal_chunks[1], buf);
        
        BottomBar::new(&self.leader_until)
            .render(chunks[2], buf);
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
