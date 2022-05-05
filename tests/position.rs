use core::{
  position::Position,
  r#move::{Move, MoveFlag},
};

use util::{assert_fen_eq, FenSegment};

mod util;

#[test]
fn test_position_to_string() {
  let pos = Position::from_fen(Position::STANDARD_FEN.into()).expect("couldn't parse fen");
  assert_eq!(
    pos.to_string(),
    "\
r n b q k b n r
p p p p p p p p
- - - - - - - -
- - - - - - - -
- - - - - - - -
- - - - - - - -
P P P P P P P P
R N B Q K B N R
"
  );
}

#[test]
fn test_apply_move() {
  let mut pos = Position::from_fen(Position::STANDARD_FEN.into()).expect("couldn't parse fen");
  pos = pos.apply_move(&Move::new(16, 8, 0)).0;
  assert_fen_eq(
    &pos.to_fen(),
    "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
    vec![FenSegment::Pieces, FenSegment::Turn, FenSegment::Castles],
  );

  pos = Position::from_fen("8/4n2P/4p3/2k1Pp2/5Pp1/6N1/3K4/8 w - - 0 1".into())
    .expect("couldn't parse fen");
  pos = pos
    .apply_move(&Move::new(
      63,
      55,
      MoveFlag::PROMOTION | MoveFlag::QUEEN_PROMOTION,
    ))
    .0;

  assert_fen_eq(
    &pos.to_fen(),
    "7Q/4n3/4p3/2k1Pp2/5Pp1/6N1/3K4/8 b - - 0 1",
    vec![FenSegment::Pieces, FenSegment::Turn, FenSegment::Castles],
  );
}
