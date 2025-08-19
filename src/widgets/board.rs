use ratatui::{
    layout::Alignment,
    text::{Line, Text},
    widgets::{Paragraph, Widget},
};
use shakmaty::fen::Fen;

#[derive(Debug)]
pub struct ChessBoard {
    alignment: Alignment,
    data: Fen,
}

impl ChessBoard {
    pub fn new(data: Fen) -> Self {
        ChessBoard {
            alignment: Alignment::Center,
            data,
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl Widget for ChessBoard {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let board_str = format!("{:?}", self.data.as_setup().board);
        let board_text = Text::from_iter(board_str.split("\n").map(Line::from));
        Paragraph::new(board_text)
            .alignment(self.alignment)
            .render(area, buf);
    }
}
