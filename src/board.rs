use crate::color::*;
use crate::piece::Piece;
use crate::position::Position;
use crate::square::{Square, EMPTY_SQUARE};

use core::cmp::Ordering;

const SQUARES_NUM: usize = 5 * 5 * 5;

// Convert the row, col, and lvl to the index of self.squares
pub fn convert_to_index(pos: Position) -> usize {
    let (row, col, lvl) = pos.get_all();
    let index = ((4 - row) * 5 + col) + (4 - lvl) * (5 * 5);
    index as usize
}

// Convert the index of self.squares to row, col, and lvl
pub fn convert_from_index(index: i32) -> Position {
    let lvl = 4 - (index / (5 * 5));
    let index = index % (5 * 5);
    let row = 4 - (index / 5);
    let col = index % 5;
    Position::new(row, col, lvl)    
}

pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    pub fn new() -> Self {
        let board = Board::empty();
        Self { board }
    }

    pub fn piece(mut self, piece: Piece) -> Self {
        let pos = piece.get_pos();
        *self.board.get_square(pos) = Square::from(piece);
        self
    }

    pub fn row(mut self, piece: Piece) -> Self {
        let mut pos = piece.get_pos();
        while pos.get_col() > 0 {
            pos = pos.next_left();
        }

        for _ in 0..5 {
            *self.board.get_square(pos) = Square::from(piece.move_to(pos));
            pos = pos.next_right();
        }

        self
    }

    pub fn set_turn(mut self, color: Color) -> Self {
        self.board = self.board.set_turn_color(color);
        self
    }

    pub fn build(self) -> Board {
        self.board
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    squares: [Square; SQUARES_NUM],
    turn: Color,
}

// impl Evaluate for Board
//     todo!()

#[allow(unused)]
impl Board {
    pub fn new() -> Board {
        BoardBuilder::new()
            .piece(Piece::Rook(BLACK,  Position::new(4, 0, 4)))
            .piece(Piece::Knight(BLACK, Position::new(4, 1, 4)))
            .piece(Piece::King(BLACK, Position::new(4, 2, 4)))
            .piece(Piece::Knight(BLACK, Position::new(4, 3, 4)))
            .piece(Piece::Rook(BLACK, Position::new(4, 4, 4)))
            .row(Piece::Pawn(BLACK, Position::new(3, 0, 4)))
            .piece(Piece::Bishop(BLACK, Position::new(4, 0, 3)))
            .piece(Piece::Unicorn(BLACK, Position::new(4, 1, 3)))
            .piece(Piece::Queen(BLACK, Position::new(4, 2, 3)))
            .piece(Piece::Bishop(BLACK, Position::new(4, 3, 3)))
            .piece(Piece::Unicorn(BLACK, Position::new(4, 4, 3)))
            .row(Piece::Pawn(BLACK, Position::new(3, 0, 3)))
            
            .row(Piece::Pawn(WHITE, Position::new(1, 0, 1)))
            .piece(Piece::Bishop(WHITE, Position::new(0, 0, 1)))
            .piece(Piece::Unicorn(WHITE, Position::new(0, 1, 1)))
            .piece(Piece::Queen(WHITE, Position::new(0, 2, 1)))
            .piece(Piece::Bishop(WHITE, Position::new(0, 3, 1)))
            .piece(Piece::Unicorn(WHITE, Position::new(0, 4, 1)))
            .row(Piece::Pawn(WHITE, Position::new(1, 0, 0)))
            .piece(Piece::Rook(WHITE, Position::new(0, 0, 0)))
            .piece(Piece::Knight(WHITE, Position::new(0, 1, 0)))
            .piece(Piece::King(WHITE, Position::new(0, 2, 0)))
            .piece(Piece::Knight(WHITE, Position::new(0, 3, 0)))
            .piece(Piece::Rook(WHITE, Position::new(0, 4, 0)))
            .set_turn(WHITE)
            .build()
    }

    pub fn empty() -> Self {
        Self {
            squares: [EMPTY_SQUARE; SQUARES_NUM],
            turn: Color::White,
        }
    }

    pub fn rating_value(&self, len: usize) -> f64 {
        todo!()
    }

    pub fn get_turn_color(&self) -> Color {
        self.turn
    }

    pub fn set_turn_color(&self, color: Color) -> Self {
        let mut result = *self;
        result.turn = color;
        result
    }

    pub fn get_material_advantage(&self, color: Color) -> i32 {
        todo!()
    }

    fn get_square(&mut self, pos: Position) -> &mut Square {
        let index = convert_to_index(pos);
        &mut self.squares[index]
    }

    pub fn squares(&self) -> &[Square; SQUARES_NUM] {
        &self.squares
    }

    fn add_piece(&mut self, piece: Piece) {
        let pos = piece.get_pos();
        *self.get_square(pos) = Square::from(piece);
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if !pos.is_on_board() {
            return  None;
        }

        let index = convert_to_index(pos);
        self.squares[index].get_piece()
    }

    pub fn has_ally_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == ally_color
        } else {
            false
        }
    }

    pub fn has_enemy_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == !ally_color
        } else {
            false
        }
    }

    pub fn has_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) != None
    }

    pub fn get_king_pos(&self, color: Color) -> Option<Position> {
        let mut king_pos = None;
        for square in &self.squares {
            if let Some(Piece::King(c, pos)) = square.get_piece() {
                if c == color {
                    king_pos = Some(pos)
                }
            }
        }

        king_pos
    }

    pub fn is_threatened(&self, pos: Position, ally_color: Color) -> bool {
        for (i, square) in self.squares.iter().enumerate() {
            let square_pos: Position = convert_from_index(i as i32);
            if !square_pos.is_orthogonal_to(pos)
                && !square_pos.is_diagonal_to(pos)
                && !square_pos.is_knight_move(pos)
                && !square_pos.is_unicorn_move(pos) {
                    continue;
            }

            if let Some(piece) = square.get_piece() {
                if piece.get_color() == ally_color {
                    continue;
                }
                if piece.is_legal_attack(pos, self) {
                    return true;
                }
            }
        }

        false
    }
}
