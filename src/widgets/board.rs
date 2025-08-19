use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Painter, Shape};
use shakmaty::fen::Fen;
use shakmaty::{File, Rank, Role, Square};

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
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        for f in 0..=7 {
            for r in 0..=7 {
                let sq = Square::from_coords(File::new(f), Rank::new(r));
                let shape = self.data.as_setup().board.piece_at(sq).map(|piece| {
                    let fg_color = match piece.color {
                        shakmaty::Color::Black => Color::Black,
                        shakmaty::Color::White => Color::White,
                    };
                    ChessPiece::new(piece.role, fg_color)
                });
                let bg_color = if (f + r) % 2 == 0 {
                    Color::Gray
                } else {
                    Color::DarkGray
                };
                let area = Rect::new(area.x + f as u16 * 4, area.y + (7 - r as u16) * 2, 4, 2);
                Canvas::default()
                    .background_color(bg_color)
                    .paint(|ctx| {
                        if let Some(shape) = &shape {
                            ctx.draw(shape);
                        }
                    })
                    .render(area, buf);
            }
        }
    }
}

struct ChessPiece {
    role: Role,
    color: Color,
}

impl ChessPiece {
    pub fn new(role: Role, color: Color) -> Self {
        ChessPiece { role, color }
    }
}

impl Shape for ChessPiece {
    fn draw(&self, painter: &mut Painter) {
        let font = match self.role {
            Role::Pawn => CHESS_FONT[0],
            Role::Bishop => CHESS_FONT[1],
            Role::Rook => CHESS_FONT[2],
            Role::Knight => CHESS_FONT[3],
            Role::Queen => CHESS_FONT[4],
            Role::King => CHESS_FONT[5],
        };
        for segment in font {
            for i in segment[1]..segment[2] {
                painter.paint(i as usize, segment[0] as usize, self.color);
            }
        }
    }
}

const CHESS_FONT: [&[[u8; 3]]; 6] = [
    // Pawn
    &[
        [1, 3, 5],
        [2, 2, 6],
        [3, 2, 6],
        [4, 3, 5],
        [5, 3, 5],
        [6, 2, 6],
    ],
    // Bishop
    &[
        [0, 4, 5],
        [1, 3, 4],
        [2, 2, 3],
        [2, 5, 6],
        [3, 2, 6],
        [4, 3, 5],
        [5, 3, 5],
        [6, 2, 6],
    ],
    // Rook
    &[
        [1, 1, 2],
        [1, 3, 5],
        [1, 6, 7],
        [2, 1, 7],
        [3, 2, 6],
        [4, 2, 6],
        [5, 2, 6],
        [6, 1, 7],
    ],
    // Knight
    &[
        [0, 3, 6],
        [1, 2, 7],
        [2, 1, 7],
        [3, 3, 7],
        [4, 2, 6],
        [5, 2, 6],
        [6, 1, 7],
    ],
    // Queen
    &[
        [0, 1, 2],
        [0, 6, 7],
        [1, 2, 3],
        [1, 5, 6],
        [2, 0, 1],
        [2, 3, 5],
        [2, 7, 8],
        [3, 1, 7],
        [4, 2, 3],
        [4, 5, 6],
        [5, 2, 6],
        [6, 1, 7],
    ],
    // King
    &[
        [0, 3, 5],
        [1, 1, 3],
        [1, 5, 7],
        [2, 0, 1],
        [2, 2, 6],
        [2, 7, 8],
        [3, 0, 1],
        [3, 3, 5],
        [3, 7, 8],
        [4, 1, 7],
        [5, 2, 6],
        [6, 1, 7],
    ],
];
