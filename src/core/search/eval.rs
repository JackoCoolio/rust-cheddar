use crate::position::{Color, Position};

use super::Score;

pub fn piece_evaluator(pos: &Position) -> Score {
    let abs_score: Score =
        (pos.white.get_number_of_pieces() as i8 - pos.black.get_number_of_pieces() as i8) as Score;
    match pos.turn {
        Color::White => abs_score,
        Color::Black => -abs_score,
    }
}
