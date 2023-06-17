use crate::color::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: i32,
    col: i32,
    lvl: i32,
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}{}{}",
            match self.lvl {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                3 => 'D',
                4 => 'E',
                _ => '?',
            },
            match self.col {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                _ => '?',
            },
            self.row + 1
        )
    }
}

#[allow(unused)]
impl Position {
    // Create a instance of Position
    // row: (0 ~ 4) , col: (0 ~ 4), lvl: (0 ~ 4)
    pub const fn new(row: i32, col: i32, lvl: i32) -> Self {
        Self { row, col, lvl }
    }

    // Return the starting position for a given color's king.
    pub const fn king_pos(color: Color) -> Self {
        match color {
            Color::White => Self::new(0, 2, 0),
            Color::Black => Self::new(4, 4, 4),
        }
    }

    pub const fn queen_pos(color: Color) -> Self {
        match color {
            Color::White => Self::new(0, 2, 1),
            Color::Black => Self::new(3, 2, 3),
        }
    }

    pub fn is_on_board(&self) -> bool {
        !self.is_off_board()
    }

    // Check if the position NOT is a valid spot on the board
    // The board is 5 * 5 * 5
    fn is_off_board(&self) -> bool {
        self.row < 0 || self.row > 4 || self.col < 0 || self.col > 4 || self.lvl < 0 || self.lvl > 4
    }

    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_col(&self) -> i32 {
        self.col
    }

    pub fn get_lvl(&self) -> i32 {
        self.lvl
    }

    pub fn get_all(&self) -> (i32, i32, i32) {
        (self.row, self.col, self.lvl)
    }

    // Return the distance between row, column, and level
    fn get_distance(&self, other: Self) -> (i32, i32, i32) {
        let drow = (self.row - other.row).abs();
        let dcol = (self.col - other.col).abs();
        let dlvl = (self.lvl - other.lvl).abs();
        (drow, dcol, dlvl)
    }

    fn add_row(&self, drow: i32) -> Self {
        let mut result = *self;
        result.row += drow;
        result
    }

    fn add_col(&self, dcol: i32) -> Self {
        let mut result = *self;
        result.col += dcol;
        result
    }

    fn add_lvl(&self, dlvl: i32) -> Self {
        let mut result = *self;
        result.lvl += dlvl;
        result
    }

    // Is this position diagonal to another position?
    pub fn is_diagonal_to(&self, other: Self) -> bool {
        let (drow, dcol, dlvl) = self.get_distance(other);
        if dlvl == 0 {
            drow == dcol
        } else {
            (drow == 0 && dcol == dlvl) || (dcol == 0 && drow == dlvl)
        }
    }

    // Get the diagonal distance between two positions
    fn diagonal_distance(&self, other: Self) -> i32 {
        let (drow, dcol, dlvl) = self.get_distance(other);
        if dlvl == 0 {
            dcol
        } else {
            dlvl
        }
    }

    pub fn is_orthogonal_to(&self, other: Self) -> bool {
        let (drow, dcol, dlvl) = self.get_distance(other);
        if dlvl == 0 {
            drow == 0 || dcol == 0
        } else {
            // is diagonal to in 3d
            drow == 0 && dcol == 0
        }
    }

    fn orthogonal_distance(&self, other: Self) -> i32 {
        let (drow, dcol, dlvl) = self.get_distance(other);
        if dlvl == 0 {
            drow + dcol
        } else {
            dlvl
        }
    }

    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) {
            self.orthogonal_distance(other) == 1
        } else if self.is_diagonal_to(other) {
            self.diagonal_distance(other) == 1
        } else {
            false
        }
    }

    // Is this position at the back of another position
    pub fn at_back_of(&self, other: Self) -> bool {
        self.row < other.row
    }

    pub fn at_front_of(&self, other: Self) -> bool {
        self.row > other.row
    }

    pub fn at_left_of(&self, other: Self) -> bool {
        self.col < other.col
    }

    pub fn at_right_of(&self, other: Self) -> bool {
        self.col > other.col
    }

    pub fn at_bottom_of(&self, other: Self) -> bool {
        self.lvl < other.lvl
    }

    pub fn at_top_of(&self, other: Self) -> bool {
        self.lvl > other.lvl
    }

    // The next_direction function might return invalid position
    pub fn next_back(&self) -> Self {
        Self::new(self.row - 1, self.col, self.lvl)
    }

    pub fn next_front(&self) -> Self {
        Self::new(self.row + 1, self.col, self.lvl)
    }

    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1, self.lvl)
    }

    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1, self.lvl)
    }

    pub fn next_down(&self) -> Self {
        Self::new(self.row, self.col, self.lvl - 1)
    }

    pub fn next_up(&self) -> Self {
        Self::new(self.row, self.col, self.lvl + 1)
    }

    // dlvl: the change in level
    // drow: the change in row
    // dcol: the change in column
    // Might return invalid position
    pub fn next_by(&self, drow: i32, dcol: i32, dlvl: i32) -> Self {
        Self::new(self.row + drow, self.col + dcol, self.lvl + dlvl)
    }

    pub fn pawn_forward(&self, ally_color: Color) -> Self {
        match ally_color {
            WHITE => self.next_front(),
            BLACK => self.next_back(),
        }
    }

    pub fn pawn_vertical(&self, ally_color: Color) -> Self {
        match ally_color {
            WHITE => self.next_up(),
            BLACK => self.next_down(),
        }
    }

    // Get the list of diagonal positions between "from" and "to"
    // This does NOT include the "from" position, include the "to" position
    pub fn diagonals_to(&self, to: Position) -> Vec<Position> {
        if !self.is_diagonal_to(to) {
            return Vec::new();
        }

        let (drow, dcol, dlvl) = self.get_distance(to);
        let row_step = if self.at_back_of(to) { 1 } else { -1 };
        let col_step = if self.at_left_of(to) { 1 } else { -1 };
        let lvl_step = if self.at_bottom_of(to) { 1 } else { -1 };

        let mut acc = *self;
        let mut result = Vec::new();
        for _ in 0..self.diagonal_distance(to) {
            if dlvl == 0 {
                acc = acc.add_row(row_step).add_col(col_step);
            } else if drow == 0 {
                acc = acc.add_col(col_step).add_lvl(lvl_step);
            } else if dcol == 0{
                acc = acc.add_row(row_step).add_lvl(lvl_step);
            } else {
                acc = acc.add_row(row_step).add_col(col_step).add_lvl(lvl_step);
            }
            result.push(acc);
        }
        result
    }

    // Get the list of orthogonal positions between "from" and "to"
    // This does NOT include the "from" position, include the "to" position
    pub fn orthogonals_to(&self, to: Position) -> Vec<Position> {
        if !self.is_orthogonal_to(to) {
            return Vec::new();
        }

        let mut row_step = 0;
        let mut col_step = 0;
        let mut lvl_step = 0;
        if self.at_back_of(to) {
            row_step = 1;
        } else if self.at_left_of(to) {
            row_step = -1;
        } else if self.at_left_of(to) {
            col_step = 1;
        } else if self.at_left_of(to) {
            col_step = -1;
        } else if self.at_bottom_of(to) {
            lvl_step = 1;
        } else if self.at_top_of(to) {
            lvl_step = -1;
        }

        let mut acc = *self;
        let mut result = Vec::new();

        for _ in 0 ..self.orthogonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step).add_lvl(lvl_step);
            result.push(acc);
        }

        result
    }

    // Whether the move from the current position to the other position
    // is a valid knight move
    pub fn is_knight_move(&self, other: Position) -> bool {
        let (drow, dcol, dlvl) = self.get_distance(other);

        (drow == 2 && dcol == 1 && dlvl == 0)
        || (drow == 1 && dcol == 2 && dlvl == 0)
        || (drow == 2 && dlvl == 1 && dcol == 0)
        || (drow == 1 && dlvl == 2 && dcol == 0)
        || (dlvl == 2 && dcol == 1 && drow == 0)
        || (dlvl == 1 && dcol == 2 && drow == 0)
    }

    pub fn is_unicorn_move(&self, other: Position) -> bool {
        let (drow, dcol, dlvl) = self.get_distance(other);

        drow == dcol && dcol == dlvl
    }
}
