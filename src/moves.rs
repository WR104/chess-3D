use crate::board::Board;
use crate::color::Color;
use crate::piece::Piece;
use crate::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    Piece(Position, Position),
    Promotion(Position, Position, Piece),
    Resign,
}

impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            Move::Piece(from, to) => write!(f, "{} {}", from, to),
            Move::Promotion(from, to, piece) => {
                write!(f, "{} {} {}", from, to, piece.get_type())
            }
            Move::Resign => write!(f, "Resign"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResult {
    Continuing(Board),
    Victory(Board, Color),
    Stalemate(Board),
    IllegalMove(Move),
}

pub trait Evaluate: Sized {
    fn value_for(&self, color: Color) -> f64;

    fn get_current_player_color(&self) -> Color;

    fn get_legal_moves(&self) -> Vec<Move>;

    fn apply_eval_move(&self, m: Move) -> Self;

    // Get the best move for the current player with `depth` number of moves
    // of lookahead.
    //
    // This method returns
    // 1. The best move
    // 2. The number of boards evaluated to come to a conclusion
    // 3. The rating of the best move
    //
    // It's best not to use the rating value by itself for anything, as it
    // is relative to the other player's move ratings as well.
    fn get_best_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -999999.0;
        let mut best_move = Move::Resign;

        let color = self.get_current_player_color();

        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.apply_eval_move(*m).minimax(
                depth,
                -1000000.0,
                1000000.0,
                false,
                color,
                &mut board_count,
            );
            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        (best_move, board_count, best_move_value)
    }

    fn minimax(
        &self,
        depth: i32,
        mut alpha: f64,
        mut beta: f64,
        is_maximizing: bool,
        getting_move_for: Color,
        board_count: &mut u64,
    ) -> f64 {
        *board_count += 1;

        if depth == 0 {
            return self.value_for(getting_move_for);
        }

        let legal_moves = self.get_legal_moves();
        let mut best_move_value;

        if is_maximizing {
            best_move_value = -999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_eval_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
                    board_count,
                );

                if child_board_value > best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value > alpha {
                    alpha = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        } else {
            best_move_value = 999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_eval_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
                    board_count,
                );
                if child_board_value < best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value < beta {
                    beta = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        }

        best_move_value
    }
}
