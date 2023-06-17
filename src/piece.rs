use crate::board::Board;
use crate::color::*;
use crate::moves::Move;
use crate::position::Position;
use crate::weights::*;

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
            Self::Pawn(_, _) => "P",
        }
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

        // Get the corresponding one for the given color piece
        fn mirror_weights(white_weights: &[[[f64; 5]; 5]; 5]) -> [[[f64; 5]; 5]; 5] {
            let mut black_weights = [[[0.0; 5]; 5]; 5];
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        black_weights[x][y][z] = white_weights[4 -x][4 - y][4 - z];
                    }
                }
            }
            black_weights
        }

        let weights = match self {
            Self::King(c, _) => match *c {
               WHITE => WHITE_KING_POSITION_WEIGHTS,
               BLACK => BLACK_KING_POSITION_WEIGHTS,    //mirror_weights(&WHITE_KING_POSITION_WEIGHTS) 
            }

            Self::Queen(c, _) => match *c {
                WHITE => WHITE_QUEEN_POSITION_WEIGHTS,
                BLACK => mirror_weights(&WHITE_QUEEN_POSITION_WEIGHTS),
            }

            Self::Unicorn(c, _) => match *c {
                WHITE => WHITE_UNICORN_POSITION_WEIGHTS,
                BLACK => mirror_weights(&WHITE_UNICORN_POSITION_WEIGHTS),
            }

            Self::Rook(c, _) => match *c {
                WHITE => WHITE_ROOK_POSITION_WEIGHTS,
                BLACK => mirror_weights(&WHITE_ROOK_POSITION_WEIGHTS),
            }

            Self::Bishop(c, _) => match *c {
                WHITE => WHITE_BISHOP_POSITION_WEIGHTS,
                BLACK => mirror_weights(&WHITE_BISHOP_POSITION_WEIGHTS),
            }

            Self::Knight(c, _) => match *c {
                WHITE => WHITE_KNIGHT_POSITION_WEIGHTS,
                BLACK => mirror_weights(&WHITE_KNIGHT_POSITION_WEIGHTS)
            }

            Self::Pawn(c, _) => match *c {
                WHITE => WHITE_PAWN_POSITION_WEIGHTS,
                BLACK => mirror_weights(&WHITE_PAWN_POSITION_WEIGHTS),
            }
        };

        let (row, col, lvl) = self.get_pos().get_all();
        let c = self.get_color();
        let row = match c {
            WHITE => 4 - row,
            BLACK => row,
        };

        weights[lvl as usize][row as usize][col as usize] + (self.get_piece_value() * 10) as f64
    }

    pub fn with_color(&self, color: Color) -> Self {
        match *self {
            Self::King(_, pos) => Self::King(color, pos),
            Self::Queen(_, pos) => Self::Queen(color, pos),
            Self::Unicorn(_, pos) => Self::Unicorn(color, pos),
            Self::Rook(_, pos) => Self::Rook(color, pos),
            Self::Bishop(_, pos) => Self::Bishop(color, pos),
            Self::Knight(_, pos) => Self::Knight(color, pos),
            Self::Pawn(_, pos) => Self::Pawn(color, pos),
        }
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

    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_, _))
    }

    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen(_, _))
    }

    pub fn is_unicorn(&self) -> bool {
        matches!(self, Self::Unicorn(_, _))
    }

    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook(_, _))
    }

    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop(_, _))
    }

    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight(_, _))
    }

    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_, _))
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

    pub(crate) fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        let mut result = Vec::new();
        match *self {
            Self::Pawn(ally_color, pos) => {
                let frontward = pos.pawn_forward(ally_color);
                let upward = pos.pawn_vertical(ally_color);

                for new_pos in &[frontward, upward] {
                    if new_pos.is_on_board() && !board.has_piece(*new_pos) {
                        result.push(Move::Piece(pos, *new_pos));
                    }
                }

                for new_pos in &[
                    frontward.next_left(),
                    frontward.next_right(),
                    upward.next_left(),
                    upward.next_right(),
                ] {
                    if new_pos.is_on_board() && board.has_enemy_piece(*new_pos, ally_color) {
                        result.push(Move::Piece(pos, *new_pos));
                    }
                }
            }

            Self::King(ally_color, pos) => {
                for dlvl in -1..=1 {
                    for drow in -1..=1 {
                        for dcol in -1..=1 {
                            if dlvl != 0 || drow != 0 || dcol != 0 {
                                let new_pos = pos.next_by(drow, dcol, dlvl);

                                if new_pos.is_on_board()
                                    && !board.has_ally_piece(new_pos, ally_color)
                                {
                                    result.push(Move::Piece(pos, new_pos));
                                }
                            }
                        }
                    }
                }
            }

            Self::Queen(ally_color, pos) => {
                for dlvl in -4i32..=4 {
                    for drow in -4i32..=4 {
                        for dcol in -4i32..=4 {
                            if (dlvl != 0 || drow != 0 || dcol != 0)
                                && (dlvl == 0
                                    || drow == 0
                                    || dcol == 0
                                    || dlvl.abs() == drow.abs()
                                    || dlvl.abs() == dcol.abs()
                                    || drow.abs() == dcol.abs())
                            {
                                let new_pos = pos.next_by(drow, dcol, dlvl);
                                if new_pos.is_on_board()
                                    && !board.has_ally_piece(new_pos, ally_color)
                                {
                                    result.push(Move::Piece(pos, new_pos));
                                }
                            }
                        }
                    }
                }
            }

            Self::Unicorn(ally_color, pos) => {
                for dlvl in -4..=4 {
                    let new_pos = pos.next_by(dlvl, dlvl, dlvl);
                    if new_pos.is_on_board()
                        && !board.has_ally_piece(new_pos, ally_color)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }

                    let new_pos = pos.next_by(dlvl, dlvl, -dlvl);
                    if new_pos.is_on_board()
                        && !board.has_ally_piece(new_pos, ally_color)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }

                    let new_pos = pos.next_by(dlvl, -dlvl, dlvl);
                    if new_pos.is_on_board()
                        && !board.has_ally_piece(new_pos, ally_color)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }

                    let new_pos = pos.next_by(dlvl, -dlvl, -dlvl);
                    if new_pos.is_on_board()
                        && !board.has_ally_piece(new_pos, ally_color)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }

                // This is the concise verison
                // for dlvl in -4..=4 {
                //     for drow in [-1, 1].iter().cloned() {
                //         for dcol in [-1, 1].iter().cloned() {
                //             let new_pos = pos.next_by(dlvl, drow * dlvl, dcol * dlvl);
                //             if new_pos.is_on_board() && !board.has_ally_piece(new_pos, ally_color) {
                //                 result.push(Move::Piece(pos, new_pos));
                //             }
                //         }
                //     }
                // }
            }

            Self::Rook(ally_color, pos) => {
                for lvl in 0..5 {
                    let new_pos = Position::new(lvl, pos.get_row(), pos.get_col());
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for row in 0..5 {
                    let new_pos = Position::new(pos.get_lvl(), row, pos.get_col());
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..5 {
                    let new_pos = Position::new(pos.get_lvl(), pos.get_row(), col);
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
            }

            Self::Bishop(ally_color, pos) => {
                for lvl in 0..5 {
                    for row in 0..5 {
                        let new_pos = Position::new(lvl, row, pos.get_col());
                        if new_pos != pos
                            && !board.has_ally_piece(new_pos, ally_color)
                            && new_pos.is_diagonal_to(pos)
                        {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
                for lvl in 0..5 {
                    for col in 0..5 {
                        let new_pos = Position::new(lvl, pos.get_row(), col);
                        if new_pos != pos
                            && !board.has_ally_piece(new_pos, ally_color)
                            && new_pos.is_diagonal_to(pos)
                        {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
                for row in 0..5 {
                    for col in 0..5 {
                        let new_pos = Position::new(pos.get_lvl(), row, col);
                        if new_pos != pos
                            && !board.has_ally_piece(new_pos, ally_color)
                            && new_pos.is_diagonal_to(pos)
                        {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
            }

            Self::Knight(ally_color, pos) => {
                for dlvl in -2i32..=2 {
                    for drow in -2i32..=2 {
                        for dcol in -2i32..=2 {
                            if (dlvl.abs() + drow.abs() + dcol.abs() == 3)
                                && (dlvl.abs() == 2 || drow.abs() == 2 || dcol.abs() == 2)
                            {
                                let new_pos = pos.next_by(dlvl, drow, dcol);
                                if new_pos.is_on_board() 
                                    && !board.has_ally_piece(new_pos, ally_color)
                                {
                                    result.push(Move::Piece(pos, new_pos));
                                }
                            }
                        }
                    }
                }
            }
        }

        let color = self.get_color();
        result 
            .into_iter()
            .filter(|x| match x {
                Move::Piece(from, to) | Move::Promotion(from, to, _) => {
                    from.is_on_board() && to.is_on_board() && board.is_legal_move(*x, color)
                }
                Move::Resign => board.is_legal_move(*x, color),
            })
            .collect::<Vec<Move>>()

    }


    // Verify that moving to a new position is a legal move
    pub(crate) fn is_legal_move(&self, new_pos: Position, board: &Board) -> bool{
        if board.has_ally_piece(new_pos, self.get_color()) || !new_pos.is_on_board() {
            return  false;
        }

        match *self {
            Piece::King(_, pos) => pos.is_adjacent_to(new_pos),
            Self::Queen(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            Piece::Unicorn(_, pos) => pos.is_unicorn_move(new_pos),
            Piece::Rook(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }

                    true
                } else {
                    false
                }
            },
            Piece::Knight(_, pos) => pos.is_knight_move(new_pos),
            Piece::Bishop(_, pos) => {
                if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            },
            Piece::Pawn(ally_color, pos) => {
                let front = pos.pawn_forward(ally_color);
                let vertical = pos.pawn_vertical(ally_color);
                (board.has_enemy_piece(new_pos, ally_color) && new_pos == front.next_left())
                || (board.has_enemy_piece(new_pos, ally_color) && new_pos == front.next_right())
                || (!board.has_piece(new_pos) && new_pos == front)
                || (board.has_enemy_piece(new_pos, ally_color) && new_pos == vertical.next_left())
                || (board.has_enemy_piece(new_pos, ally_color) && new_pos == vertical.next_right())
                || (!board.has_piece(new_pos) && new_pos == vertical)
            },
        }
    }
}
