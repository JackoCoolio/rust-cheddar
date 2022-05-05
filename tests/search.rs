use core::{
  position::Position,
  search::{eval::PieceEvaluate, tree::find_best_move},
};

#[test]
fn test_find_best_move() {
  let pos = Position::from_fen(Position::STANDARD_FEN.into()).expect("couldn't parse fen");
  println!("finding best move");
  println!("{}", find_best_move(&pos, &PieceEvaluate {}, 3).to_string());
}
