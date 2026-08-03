use std::io;

use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Style, widgets::{Tabs, Widget},
};
use ratatui_comfy_toaster::{ToastBuilder, ToastEngine, ToastEngineBuilder, ToastMessage};

use crate::{colors::{ROSE}, common::{db::{self, DB}, goal::GoalSheet, types::{Error, ErrorLevel::FATAL}}, utils::padding::add_padding, widgets::title::TitleBar};

mod colors;
mod widgets;
mod utils;
mod common;

struct App {
    message: String,
    goal_sheet: Vec<GoalSheet>,
    conn: DB,
    error: Option<Error>,
    exit: bool,
    toast_engine: Option<ToastEngine<ToastMessage>>
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
            toast_engine: None
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
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.message = "Left pressed".to_string(),
            KeyCode::Right => self.message = "Right pressed".to_string(),
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
            ratatui::prelude::Constraint::Length(3),
        ])
        .split(area);
        
        let title_bar = TitleBar::default();
        title_bar.render(chunks[0], buf);
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
