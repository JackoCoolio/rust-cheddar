use core::{
    position::Position,
    search::{eval::piece_evaluator, tree::find_best_move},
};

#[test]
fn test_find_best_move() {
    let pos = Position::from_fen(Position::STANDARD_FEN.into()).expect("couldn't parse fen");
    println!("finding best move");
    let best = find_best_move(&pos, &mut piece_evaluator, 5).to_string();
    assert!(!best.is_empty());
    println!("best move: {}", best);
}
