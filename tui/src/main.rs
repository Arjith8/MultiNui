use std::{io, time::{Duration, Instant}};

use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, layout::{Constraint, Layout}, widgets::{Block, Borders, List, Padding, Widget},
};
use ratatui_comfy_toaster::{ToastEngine, ToastEngineBuilder, ToastMessage};

use crate::{common::{db::{self, DB}, goal::GoalSheet, types::Error}, utils::padding::add_padding, widgets::{bottom_bar::BottomBar, goals_tab::GoalTab, page::PageIndicator, title::TitleBar}};

mod colors;
mod widgets;
mod utils;
mod common;

struct App {
    message: String,
    leader_until: Option<Instant>,
    conn: DB,
    error: Option<Error>,
    exit: bool,
    toast_engine: Option<ToastEngine<ToastMessage>>,
    page_indicator: PageIndicator,
    goal_sheets: Vec<GoalSheet>,
    current_tab: usize
}

impl App {
    pub fn new() -> Self{
        let conn = db::DB::open().unwrap();
        Self { 
            error: None,
            message: "".to_string(),
            goal_sheets: Vec::new(),
            conn,
            exit: false,
            leader_until: None,
            toast_engine: None,
            page_indicator: PageIndicator::default(),
            current_tab: 0
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
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('h') => self.page_indicator.current = 1,
                KeyCode::Char('s') => self.page_indicator.current = 2,
                KeyCode::Char(c @ '1'..='9') => {
                    let page: usize = c.to_digit(10).unwrap() as usize - 1;
                    self.current_tab = page
                }
                _ => {}
            }
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
        ])
        .split(area);
        
        let title_block = Block::new();
        let title_block_area = title_block.inner(chunks[0]);
        let title_bar = TitleBar::new("MultiNui Goals Manager", &self.page_indicator);
        title_bar.render(title_block_area, buf);
        title_block.render(chunks[0], buf);

        let block = Block::new()
            .padding(Padding::from(0));
        let inner = block.inner(chunks[1]);
        let block_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1)
        ]).split(inner);

        GoalTab::new(self.current_tab, add_padding(2, vec!["Tab 1".into(), "Tab2".into()]))
            .render(block_chunks[0], buf);

        let main_block = Block::new()
            .borders(Borders::ALL);

        let main_block_inner = main_block.inner(block_chunks[1]);
        let all_goals = GoalSheet::get_all();
        List::new(all_goals.iter().map(|goal| goal.name.clone()))
            .render(main_block_inner, buf);

        main_block.render(block_chunks[1], buf);

        block.render(chunks[1], buf);

        BottomBar::new(&self.leader_until)
            .render(chunks[2], buf);
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
