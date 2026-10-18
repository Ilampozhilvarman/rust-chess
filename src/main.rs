use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Game".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    row: u32,
    column: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    White,
    Black,
}

enum PieceKind {
    Pawn { has_moved: bool },
    Rook,
    Queen,
    Bishop,
    King,
    Knight,
}

struct Piece {
    side: Side,
    pos: Position,
    kind: PieceKind,
}

struct Board {
    pieces: Vec<Board>,
}

impl Board {
    fn is_occupied(&self, pos: Position) -> Option<Side> {
        for piece in &self.pieces {
            if piece.pos == pos {
                return Some(piece.side);
            }
        }
        None
    }
}

impl Piece {
    fn new(&self, side: Side, row: char, column: u32, kind: PieceKind) -> Self {
        Self {
            side,
            pos: Position { row, column },
            kind,
        }
    }

    fn is_at(&self, row: char, column: u32) -> bool {
        self.pos.row == row && self.pos.column == column
    }

    fn get_valid_moves(&self, board: &Board) {
        match self.kind {
            PieceKind::Bishop => {}
            PieceKind::King => {}
            PieceKind::Pawn { has_moved } => {}
            PieceKind::Rook => {}
            PieceKind::Queen => {}
            PieceKind::Knight => {}
        }
    }
}

impl Position {
    fn get_position_behind(&self) -> Option<Position> {
        if self.row <= 1 {
            None
        } else {
            Some(Position {
                row: self.row - 1,
                column: self.column,
            })
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
