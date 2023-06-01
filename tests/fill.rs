use core::{
    board::{Bits, Board},
    fill::{king_fill, knight::knight_fill, slide::SlideFill},
    position::Position,
};

#[test]
fn test_north() {
    let mut pos =
        Position::from_fen("8/8/8/8/8/8/8/RRRRRRRR w - - 0 1".into()).expect("couldn't parse fen");

    let mut occl: Bits = SlideFill::north_occl(pos.white.rooks, !pos.get_all_pieces());

    assert_eq!(occl | pos.white.rooks, !0_u64);

    pos = Position::from_fen("8/8/pppppppp/8/8/8/8/RRRRRRRR w - - 0 1".into())
        .expect("couldn't parse fen");
    occl = SlideFill::north_occl(pos.white.rooks, !pos.get_all_pieces());

    assert_eq!(occl | pos.white.rooks, 0x000000ffffffffff);
}

#[test]
fn test_east() {
    let mut pos =
        Position::from_fen("R7/R7/R7/R7/R7/R7/R7/R7 w - - 0 1".into()).expect("couldn't parse fen");
    println!("{}", Board::bits_to_string(pos.white.rooks));

    let mut occl: Bits = SlideFill::east_occl(pos.white.rooks, !pos.get_all_pieces());

    println!("{}", Board::bits_to_string(occl));

    // test that filling on empty board will fill the entire board
    assert_eq!(occl | pos.white.rooks, !0_u64);

    pos = Position::from_fen("R4p2/R4p2/R4p2/R4p2/R4p2/R4p2/R4p2/R4p2 w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::east_occl(pos.white.rooks, !pos.get_all_pieces());

    // test that a row of pawns blocks the occl fill
    assert_eq!(occl | pos.white.rooks, 0x1f1f1f1f1f1f1f1f);
}

#[test]
fn test_south() {
    let mut pos =
        Position::from_fen("RRRRRRRR/8/8/8/8/8/8/8 w - - 0 1".into()).expect("couldn't parse fen");

    let mut occl = SlideFill::south_occl(pos.white.rooks, !pos.get_all_pieces());

    // test that filling on empty board will fill the entire board
    assert_eq!(occl, !0_u64);

    pos = Position::from_fen("RRRRRRRR/8/8/8/8/pppppppp/8/8 w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::south_occl(pos.white.rooks, !pos.get_all_pieces());

    // test that a row of pawns blocks the occl fill
    assert_eq!(occl | pos.white.rooks, 0xffffffffff000000);
}

#[test]
fn test_west() {
    let mut pos =
        Position::from_fen("7R/7R/7R/7R/7R/7R/7R/7R w - - 0 1".into()).expect("couldn't parse fen");

    let mut occl = SlideFill::west_occl(pos.white.rooks, !pos.get_all_pieces());

    // test that filling on empty board will fill the entire board
    assert_eq!(occl, !0_u64);

    pos = Position::from_fen("2p4R/2p4R/2p4R/2p4R/2p4R/2p4R/2p4R/2p4R w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::west_occl(pos.white.rooks, !pos.get_all_pieces());

    // test that a row of pawns blocks the occl fill
    assert_eq!(occl, 0xf8f8f8f8f8f8f8f8);
}

#[test]
fn test_ne() {
    let mut pos = Position::from_fen("Q7/Q7/Q7/Q7/Q7/Q7/Q7/QQQQQQQQ w - - 0 1".into())
        .expect("couldn't parse fen");

    let mut occl = SlideFill::ne_occl(pos.white.queens, !pos.get_all_pieces());

    // test that filling on an empty board will fill the entire board
    assert_eq!(occl, !0_u64);

    pos = Position::from_fen("Qpp5/Q1pp4/Q2pp3/Q3pp2/Q4pp1/Q5pp/Q6p/QQQQQQQQ w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::ne_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, 0x0103070f1f3f7fff);
}

#[test]
fn test_se() {
    let mut pos = Position::from_fen("QQQQQQQQ/Q7/Q7/Q7/Q7/Q7/Q7/Q7 w - - 0 1".into())
        .expect("couldn't parse fen");

    let mut occl = SlideFill::se_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, !0_u64);

    pos = Position::from_fen("QQQQQQQQ/Q6p/Q5pp/Q4pp1/Q3pp2/Q2pp3/Q1pp4/Qpp5 w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::se_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, 0xff7f3f1f0f070301);
}

#[test]
fn test_sw() {
    let mut pos = Position::from_fen("QQQQQQQQ/7Q/7Q/7Q/7Q/7Q/7Q/7Q w - - 0 1".into())
        .expect("couldn't parse fen");

    let mut occl = SlideFill::sw_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, !0_u64);

    pos = Position::from_fen("QQQQQQQQ/p6Q/pp5Q/1pp4Q/2pp3Q/3pp2Q/4pp1Q/5ppQ w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::sw_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, 0xfffefcf8f0e0c080);
}

#[test]
fn test_nw() {
    let mut pos = Position::from_fen("7Q/7Q/7Q/7Q/7Q/7Q/7Q/QQQQQQQQ w - - 0 1".into())
        .expect("couldn't parse fen");

    let mut occl = SlideFill::nw_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, !0_u64);

    pos = Position::from_fen("5ppQ/4pp1Q/3pp2Q/2pp3Q/1pp4Q/pp5Q/p6Q/QQQQQQQQ w - - 0 1".into())
        .expect("couldn't parse fen");

    occl = SlideFill::nw_occl(pos.white.queens, !pos.get_all_pieces());

    assert_eq!(occl, 0x80c0e0f0f8fcfeff);
}

#[test]
fn test_knight_fill() {
    let pos =
        Position::from_fen("8/8/1N4N1/8/4N3/8/N6N/8 w - - 0 1".into()).expect("couldn't parse fen");

    let fill = knight_fill(pos.white.knights);

    assert_eq!(fill, 0xa518285ce7642824);
}

#[test]
fn test_king_fill() {
    let pos =
        Position::from_fen("K6K/8/8/8/3K4/8/8/K6K w - - 0 1".into()).expect("couldn't parse fen");

    let fill = king_fill(pos.white.king);

    assert_eq!(fill, 0x42c3001c141cc342);
}
