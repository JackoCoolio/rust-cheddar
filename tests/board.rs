use core::{
    board::{Bits, Board, Castles},
    position::{alg_to_index, index_to_alg, Position},
};

#[test]
fn test_alg_to_index() {
    assert_eq!(alg_to_index("a1"), Ok(0));
    assert_eq!(alg_to_index("a8"), Ok(56));
    assert_eq!(alg_to_index("h8"), Ok(63));
    assert_eq!(alg_to_index("h1"), Ok(7));
    assert_eq!(
        alg_to_index("-"),
        Err("algebraic notation must be 2 characters".to_string())
    ); // invalid index
}

#[test]
fn test_index_to_alg() {
    assert_eq!(index_to_alg(0), "a1");
    assert_eq!(index_to_alg(56), "a8");
    assert_eq!(index_to_alg(63), "h8");
    assert_eq!(index_to_alg(7), "h1");
}

#[test]
fn test_standard_fen() {
    let pos = Position::from_fen(Position::STANDARD_FEN.into()).expect("couldn't parse fen");

    assert_eq!(pos.white.pawns, 0x000000000000ff00);
    assert_eq!(pos.white.knights, 0x0000000000000042);
    assert_eq!(pos.white.bishops, 0x0000000000000024);
    assert_eq!(pos.white.rooks, 0x0000000000000081);
    assert_eq!(pos.white.queens, 0x0000000000000008);
    assert_eq!(pos.white.king, 0x0000000000000010);

    assert_eq!(pos.black.pawns, 0x00ff000000000000);
    assert_eq!(pos.black.knights, 0x4200000000000000);
    assert_eq!(pos.black.bishops, 0x2400000000000000);
    assert_eq!(pos.black.rooks, 0x8100000000000000);
    assert_eq!(pos.black.queens, 0x0800000000000000);
    assert_eq!(pos.black.king, 0x1000000000000000);

    assert_eq!(pos.en_passant, Position::NO_EN_PASSANT);

    // test reverse (move count isn't implemented, so ignore that)
    assert!(Position::STANDARD_FEN.starts_with(&pos.to_fen()));
}

#[test]
fn test_fen_en_passant() {
    let pos =
        Position::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".into())
            .expect("couldn't parse fen");
    assert_eq!(pos.en_passant, 20);
}

#[test]
fn test_castles() {
    // all castles
    let mut pos = Position::from_fen(
        "rnb1kb1r/pp3p1p/2p1pnp1/q2p4/2PP4/2N1PN2/PP2BPPP/R1BQK2R w KQkq - 0 1".into(),
    )
    .unwrap();
    assert_eq!(
        pos.castles,
        Castles::WHITE_KING | Castles::WHITE_QUEEN | Castles::BLACK_KING | Castles::BLACK_QUEEN
    );

    // one castle missing
    pos = Position::from_fen(
        "rnb1kb1r/pp3p1p/2p1pnp1/q2p4/2PP4/2N1PN2/PP2BPPP/R1BQK2R w Kkq - 0 1".into(),
    )
    .unwrap();
    assert_eq!(
        pos.castles,
        Castles::WHITE_KING | Castles::BLACK_KING | Castles::BLACK_QUEEN,
    );

    // no castles
    pos = Position::from_fen(
        "rnb1kb1r/pp3p1p/2p1pnp1/q2p4/2PP4/2N1PN2/PP2BPPP/R1BQK2R w - - 0 1".into(),
    )
    .unwrap();
    assert_eq!(pos.castles, 0);

    // a bunch of junk characters
    Position::from_fen(
        "rnb1kb1r/pp3p1p/2p1pnp1/q2p4/2PP4/2N1PN2/PP2BPPP/R1BQK2R w asdfhfdieERTYTREB - 0 1".into(),
    )
    .expect_err("expected from_fen to return an error");
}

#[test]
fn test_mirror_board() {
    let board: Bits = 0x00000000000000ff;
    assert_eq!(Board::mirror_board(board), 0xff00000000000000);
}
