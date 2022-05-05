use crate::{
  position::Position,
  r#move::{gen::generate_legal_moves, list::MoveList, Move},
};

use super::{eval::Evaluate, Score};

pub fn negamax<E>(pos: &Position, eval: &E, depth: u8) -> Score
where
  E: Evaluate,
{
  if depth == 0 {
    return eval.evaluate(pos);
  }

  let mut max = std::i32::MIN + 1;
  let moves = generate_legal_moves(pos);
  println!("found {} moves:\n{}", moves.count(), moves.to_string());

  for mov in moves.into_iter() {
    let (next_pos, _) = pos.apply_move(&mov);
    println!("considering {}", mov.to_string());
    let score = -dbg!(negamax(&next_pos, eval, depth - 1));
    if score > max {
      max = score;
    }
  }

  max
}

pub fn find_best_move<E>(pos: &Position, eval: &E, depth: u8) -> Move
where
  E: Evaluate,
{
  let mut best: (Move, i32) = (Default::default(), std::i32::MIN);
  let moves: MoveList = generate_legal_moves(pos);
  println!("found {} moves:\n{}", moves.count(), moves.to_string());

  for mov in moves.into_iter() {
    println!("considering {}", mov.to_string());
    let (next_pos, _) = pos.apply_move(&mov);
    let score: i32 = negamax(&next_pos, eval, depth);
    if score > best.1 {
      best = (mov, score);
    }
  }

  best.0
}
