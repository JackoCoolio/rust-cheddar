use crate::{
    mov::{gen::generate_legal_moves, list::MoveList, Move},
    position::Position,
};

use super::Score;

pub fn negamax<E: FnMut(&Position) -> Score>(
    pos: &Position,
    evaluator: &mut E,
    depth: u8,
) -> Score {
    if depth == 0 {
        return evaluator(pos);
    }

    let mut max = std::i32::MIN + 1;
    let moves = generate_legal_moves(pos);

    for mov in moves.into_iter() {
        let (next_pos, _) = pos.apply_move(&mov);
        let score = -negamax(&next_pos, evaluator, depth - 1);
        if score > max {
            max = score;
        }
    }

    max
}

pub fn find_best_move<E: FnMut(&Position) -> Score>(
    pos: &Position,
    eval: &mut E,
    depth: u8,
) -> Move {
    let mut best: (Move, i32) = (Default::default(), std::i32::MIN);
    let moves: MoveList = generate_legal_moves(pos);

    for mov in moves.into_iter() {
        let (next_pos, _) = pos.apply_move(&mov);
        let score: i32 = negamax(&next_pos, eval, depth);
        if score > best.1 {
            best = (mov, score);
        }
    }

    best.0
}
