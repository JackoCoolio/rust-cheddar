use crate::position::{Color, Position};

use super::Score;

pub trait Evaluate {
    /// Return a score for the given position.
    fn evaluate(&self, pos: &Position) -> Score;
}

/// An evaluator function that returns the piece differential for the current color.
pub struct PieceEvaluate;
impl Evaluate for PieceEvaluate {
    fn evaluate(&self, pos: &Position) -> Score {
        let abs_score: Score = (pos.white.get_number_of_pieces() as i8
            - pos.black.get_number_of_pieces() as i8) as Score;
        match pos.turn {
            Color::White => abs_score,
            Color::Black => -abs_score,
        }
    }
}
