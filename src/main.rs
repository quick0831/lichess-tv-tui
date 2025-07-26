use std::{
    io::{BufRead, BufReader},
    str::FromStr,
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
use serde::{Deserialize, Deserializer, de::Visitor};
use shakmaty::{Square, fen::Fen};

#[allow(dead_code)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize)]
#[serde(tag = "t", content = "d")]
enum TvData {
    #[serde(rename = "featured")]
    Featured(FeaturedData),
    #[serde(rename = "fen")]
    Fen(FenData),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FeaturedData {
    id: String,
    orientation: Color,
    players: [Player; 2],
    fen: Fen,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Player {
    color: Color,
    user: User,
    rating: i32,
    seconds: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    title: Option<String>,
    flair: Option<String>,
    #[serde(default)]
    patron: bool,
    id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FenData {
    fen: Fen,
    #[serde(deserialize_with = "parse_lm")]
    lm: [Square; 2],
    wc: i32,
    bc: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
enum Color {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}

fn parse_lm<'de, D>(deserializer: D) -> Result<[Square; 2], D::Error>
where
    D: Deserializer<'de>,
{
    struct StrVisitor;

    impl Visitor<'_> for StrVisitor {
        type Value = [Square; 2];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a standard UCI chess move")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok([
                Square::from_str(&v[0..2]).unwrap(),
                Square::from_str(&v[2..4]).unwrap(),
            ])
        }
    }

    deserializer.deserialize_str(StrVisitor)
}

fn main() -> Result<()> {
    set_panic_hook();
    color_eyre::install()?;
    std::thread::spawn(get_lichess_tv);
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        hook(panic_info);
    }));
}

fn get_lichess_tv() {
    let a = ureq::get("https://lichess.org/api/tv/feed")
        .call()
        .unwrap()
        .into_body()
        .into_reader();

    let a = BufReader::new(a);

    for line in a.lines() {
        match line {
            Ok(line) => match serde_json::from_str::<TvData>(&line) {
                Ok(_data) => { /* println!("{:?}", data) */ }
                Err(err) => panic!("JSON parse failed: {err}"),
            },
            Err(_) => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
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
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
