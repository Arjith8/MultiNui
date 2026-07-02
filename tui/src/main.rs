use std::io;

use ratatui::{DefaultTerminal, Frame, crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Stylize}, symbols::border, text::Line, widgets::{Block, Widget}};

#[derive(Debug, Default)]
struct App {
    message: String,
    exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        let block = Block::bordered()
            .title(Line::from(self.message.as_str()).centered())
            .border_set(border::THICK)
            .render(area, buf);
    }
    
}
fn main() -> io::Result<()>{
    ratatui::run(|terminal| App::default().run(terminal))
}
