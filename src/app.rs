use std::{
    sync::mpsc::{Receiver, TryRecvError},
    time::Duration,
};

use color_eyre::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use shakmaty::fen::Fen;

use crate::api::TvData;

#[derive(Debug)]
pub struct App {
    rx: Receiver<TvData>,
    data: Fen,
    exit: bool,
}

impl App {
    pub fn new(rx: Receiver<TvData>) -> Self {
        App {
            rx,
            data: Fen::empty(),
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.handle_api_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_millis(20);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn handle_api_events(&mut self) -> Result<()> {
        match self.rx.try_recv() {
            Ok(data) => {
                self.data = match data {
                    TvData::Featured(data) => data.fen,
                    TvData::Fen(data) => data.fen,
                };
                Ok(())
            }
            Err(TryRecvError::Empty) => Ok(()),
            Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected)?,
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Lichess TV ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut counter_text = Text::from(vec![Line::from(vec![
            "Fullmoves: ".into(),
            self.data.as_setup().fullmoves.to_string().into(),
        ])]);

        for line in format!("{:?}", self.data.as_setup().board).split("\n") {
            counter_text.push_line(line.to_owned());
        }

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
