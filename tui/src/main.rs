use std::{io, time::{Duration, Instant}};

use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, style::Style, widgets::{Block, Borders, Tabs, Widget},
};
use ratatui_comfy_toaster::{ToastBuilder, ToastEngine, ToastEngineBuilder, ToastMessage};

use crate::{colors::ROSE, common::{db::{self, DB}, goal::GoalSheet, types::{Error, ErrorLevel::FATAL}}, utils::padding::add_padding, widgets::{bottom_bar::BottomBar, page::PageIndicator, title::TitleBar}};

mod colors;
mod widgets;
mod utils;
mod common;

struct App {
    message: String,
    leader_until: Option<Instant>,
    goal_sheet: Vec<GoalSheet>,
    conn: DB,
    error: Option<Error>,
    exit: bool,
    toast_engine: Option<ToastEngine<ToastMessage>>,
    page_indicator: PageIndicator
}

impl App {
    pub fn new() -> Self{
        let conn = db::DB::open().unwrap();
        return Self { 
            error: None,
            message: "".to_string(),
            goal_sheet: Vec::new(),
            conn,
            exit: false,
            leader_until: None,
            toast_engine: None,
            page_indicator: PageIndicator::default()
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
        return self.leader_until.is_some_and(|until| until >= Instant::now());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.leader_until = Some(Instant::now() + Duration::from_secs(2));
            }
            _ => {}
        }
        if self.is_leader_active(){
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('1') => self.page_indicator.current = 1,
                KeyCode::Char('2') => self.page_indicator.current = 2,
                KeyCode::Char('3') => self.page_indicator.current = 3,
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
        
        let title_bar = TitleBar::new("MultiNui Goals Manager", &self.page_indicator);
        title_bar.render(chunks[0], buf);

        BottomBar::new(&self.leader_until)
            .render(chunks[2], buf);
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
