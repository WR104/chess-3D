use crate::board::Board;
use crate::color::*;
use crate::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Unicorn(Color, Position),
    Rook(Color, Position),
    Knight(Color, Position),
    Bishop(Color, Position),
    Pawn(Color, Position),
}

#[allow(unused)]
impl Piece {
    pub fn get_type(&self) -> &str {
        match self {
            Self::King(_, _) => "K",
            Self::Queen(_, _) => "Q",
            Self::Unicorn(_, _) => "U",
            Self::Rook(_, _) => "R",
            Self::Bishop(_, _) => "B",
            Self::Knight(_, _) => "N",
            Self::Pawn(_, _) => "P",        }
    }

    pub fn get_piece_value(&self) -> i32 {
        match self {
            Self::King(_, _) => 9999,
            Self::Queen(_, _) => 9,
            Self::Unicorn(_, _) => 2,
            Self::Rook(_, _) => 5,
            Self::Bishop(_, _) => 3,
            Self::Knight(_, _) => 3,
            Self::Pawn(_, _) => 1,
        }
    }

    pub fn get_weighted_value(&self) -> f64 {
        todo!()
    }

    pub fn get_color(&self) -> Color {
        match self {
            Self::King(c, _)
            | Self::Queen(c, _)
            | Self::Unicorn(c, _)
            | Self::Rook(c, _)
            | Self::Bishop(c, _)
            | Self::Knight(c, _)
            | Self::Pawn(c, _) => *c,
        }
    }

    pub fn get_pos(&self) -> Position {
        match self {
            Self::King(_, p)
            | Self::Queen(_, p)
            | Self::Unicorn(_, p)
            | Self::Rook(_, p)
            | Self::Bishop(_, p)
            | Self::Knight(_, p)
            | Self::Pawn(_, p) => *p,
        }
    }

    pub fn move_to(&self, new_pos: Position) -> Self {
        match *self {
            Self::King(c, _) => Self::King(c, new_pos),
            Self::Queen(c, _) => Self::Queen(c, new_pos),
            Self::Unicorn(c, _) => Self::Unicorn(c, new_pos),
            Self::Rook(c, _) => Self::Rook(c, new_pos),
            Self::Bishop(c, _) => Self::Bishop(c, new_pos),
            Self::Knight(c, _) => Self::Knight(c, new_pos),
            Self::Pawn(c, _) => Self::Pawn(c, new_pos),
        }
    }

    pub(crate) fn get_legal_moves(&self) {
        todo!()
    }

    pub(crate) fn is_legal_move() {
        todo!()
    }

    pub(crate) fn is_legal_attack(&self, new_pos: Position, board: &Board) -> bool {
        todo!()
    }
}