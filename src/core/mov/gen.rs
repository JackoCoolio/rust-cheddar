use std::num::Wrapping;

use crate::{
    board::{Bits, BoardIndex},
    fill::{
        king_fill,
        knight::{knight_fill, KnightFill},
        north_pawn_attacks,
        slide::SlideFill,
        south_pawn_attacks,
    },
    mov::{list::MoveList, Move, MoveFlag},
    position::{Color, Position},
};

pub fn least_set_one_bit(board: Bits) -> Bits {
    if board == 1_u64 << 63 {
        return board;
    };
    (board as i64 & -(board as i64)) as u64
}

#[derive(Debug)]
struct TargetBoards {
    north: Bits,
    nne: Bits,
    ne: Bits,
    ene: Bits,
    east: Bits,
    ese: Bits,
    se: Bits,
    sse: Bits,
    south: Bits,
    ssw: Bits,
    sw: Bits,
    wsw: Bits,
    west: Bits,
    wnw: Bits,
    nw: Bits,
    nnw: Bits,
}

impl TargetBoards {
    #[allow(dead_code)]
    pub fn all(&self) -> Bits {
        self.north
            | self.nne
            | self.ne
            | self.ene
            | self.east
            | self.ese
            | self.se
            | self.sse
            | self.south
            | self.ssw
            | self.sw
            | self.wsw
            | self.west
            | self.wnw
            | self.nw
            | self.nnw
    }
}

fn generate_white_legal_target_bitboards(pos: &Position) -> TargetBoards {
    let occupancy: Bits = !pos.get_all_pieces();

    // generate black attacks

    // west
    let mut _attacks: Bits = SlideFill::west_attacks(
        pos.black.rooks | pos.black.queens,
        occupancy ^ pos.white.king,
    );
    let mut attacks: Bits = _attacks;
    let mut _super_attacks: Bits = SlideFill::east_attacks(pos.white.king, occupancy);
    let mut super_attacks_orth: Bits = _super_attacks;
    let mut horizontal_between: Bits = _attacks & _super_attacks;

    // east
    _attacks = SlideFill::east_attacks(
        pos.black.rooks | pos.black.queens,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::west_attacks(pos.white.king, occupancy);
    super_attacks_orth |= _super_attacks;
    horizontal_between |= _attacks & _super_attacks;

    // north
    _attacks = SlideFill::north_attacks(
        pos.black.rooks | pos.black.queens,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::south_attacks(pos.white.king, occupancy);
    super_attacks_orth |= _super_attacks;
    let mut vertical_between = _attacks & _super_attacks;

    // south
    _attacks = SlideFill::south_attacks(
        pos.black.rooks | pos.black.queens,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::north_attacks(pos.white.king, occupancy);
    super_attacks_orth |= _super_attacks;
    vertical_between |= _attacks & _super_attacks;

    // north east
    _attacks = SlideFill::ne_attacks(
        pos.black.queens | pos.black.bishops,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::sw_attacks(pos.white.king, occupancy);
    let mut super_attacks_diag: Bits = _super_attacks;
    let pdiag_between: Bits = _attacks & _super_attacks;

    // south west
    _attacks = SlideFill::sw_attacks(
        pos.black.queens | pos.black.bishops,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::ne_attacks(pos.white.king, occupancy);
    super_attacks_diag |= _super_attacks;

    // north west
    _attacks = SlideFill::nw_attacks(
        pos.black.queens | pos.black.bishops,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::se_attacks(pos.white.king, occupancy);
    super_attacks_diag |= _super_attacks;
    let mut ndiag_between: Bits = _attacks & _super_attacks;

    // south west
    _attacks = SlideFill::se_attacks(
        pos.black.queens | pos.black.bishops,
        occupancy ^ pos.white.king,
    );
    attacks |= _attacks;
    _super_attacks = SlideFill::nw_attacks(pos.white.king, occupancy);
    super_attacks_diag |= _super_attacks;
    ndiag_between |= _attacks & _super_attacks;

    // knight attacks
    attacks |= knight_fill(pos.black.knights);

    // pawn attacks
    attacks |= south_pawn_attacks(pos.black.pawns);

    // king attacks
    attacks |= king_fill(pos.black.king);

    // white move gen
    let between: Bits = horizontal_between | vertical_between | pdiag_between | ndiag_between;
    let _blocks: Bits = between & occupancy;
    let _check_from: Bits = (super_attacks_orth & (pos.black.rooks | pos.black.queens))
        | (super_attacks_diag & (pos.black.bishops | pos.black.queens))
        | (knight_fill(pos.white.king) & pos.black.knights)
        | (north_pawn_attacks(pos.white.king) & pos.black.pawns);

    let _null_if_check: Bits = (((attacks & pos.white.king) as i64 - 1) >> 63) as u64;
    let _null_if_double_check: Bits =
        (((_check_from & ((_check_from as i64) - 1) as u64) as i64 - 1) >> 63) as u64;

    let _check_to = _check_from | _blocks | _null_if_check;
    let mut target_mask: Bits = !pos.white.get_all_pieces() & _check_to & _null_if_double_check;

    // sliders
    let mut _sliders: Bits = (pos.white.rooks | pos.white.queens) & !(between ^ horizontal_between);
    let mut west_move_targets: Bits = SlideFill::west_attacks(_sliders, occupancy) & target_mask;
    let mut east_move_targets: Bits = SlideFill::east_attacks(_sliders, occupancy) & target_mask;

    _sliders = (pos.white.rooks | pos.white.queens) & !(between ^ vertical_between);
    let mut north_move_targets: Bits = SlideFill::north_attacks(_sliders, occupancy) & target_mask;
    let mut south_move_targets: Bits = SlideFill::south_attacks(_sliders, occupancy) & target_mask;

    _sliders = (pos.white.bishops | pos.white.queens) & !(between ^ pdiag_between);
    let mut ne_move_targets: Bits = SlideFill::ne_attacks(_sliders, occupancy) & target_mask;
    let mut sw_move_targets: Bits = SlideFill::sw_attacks(_sliders, occupancy) & target_mask;

    _sliders = (pos.white.bishops | pos.white.queens) & !(between ^ ndiag_between);
    let mut nw_move_targets: Bits = SlideFill::nw_attacks(_sliders, occupancy) & target_mask;
    let mut se_move_targets: Bits = SlideFill::se_attacks(_sliders, occupancy) & target_mask;

    // knights
    let available_knights: Bits = pos.white.knights & !between;
    let nne_move_targets: Bits = KnightFill::nne(available_knights) & target_mask;
    let ene_move_targets: Bits = KnightFill::ene(available_knights) & target_mask;
    let ese_move_targets: Bits = KnightFill::ese(available_knights) & target_mask;
    let sse_move_targets: Bits = KnightFill::sse(available_knights) & target_mask;
    let ssw_move_targets: Bits = KnightFill::ssw(available_knights) & target_mask;
    let wsw_move_targets: Bits = KnightFill::wsw(available_knights) & target_mask;
    let wnw_move_targets: Bits = KnightFill::wnw(available_knights) & target_mask;
    let nnw_move_targets: Bits = KnightFill::nnw(available_knights) & target_mask;

    // pawn captures
    let pawn_targets: Bits = (pos.black.get_all_pieces() & target_mask)
        | if let Some(en_passant) = pos.en_passant {
            (1_u64 << en_passant) & -((en_passant < 64) as i64) as u64
        } else {
            0
        };
    let mut available_pawns: Bits = pos.white.pawns & !(between ^ pdiag_between);
    ne_move_targets |= SlideFill::ne_one(available_pawns) & pawn_targets;

    available_pawns = pos.white.pawns & !(between ^ ndiag_between);
    nw_move_targets |= SlideFill::nw_one(available_pawns) & pawn_targets;

    // pawn pushes
    // available_pawns = pos.white.pawns & !(between ^ vertical_between);
    let pawn_pushes: Bits = SlideFill::north_one(pos.white.pawns) & occupancy;
    north_move_targets |= pawn_pushes & target_mask;

    // double pawn pushes
    let double_pawn_pushes =
        SlideFill::north_one(pawn_pushes) & occupancy & target_mask & 0x00000000ff000000;
    north_move_targets |= double_pawn_pushes;

    // king moves
    target_mask = !(pos.white.get_all_pieces() | attacks);

    north_move_targets |= SlideFill::north_one(pos.white.king) & target_mask;
    ne_move_targets |= SlideFill::ne_one(pos.white.king) & target_mask;
    se_move_targets |= SlideFill::se_one(pos.white.king) & target_mask;
    south_move_targets |= SlideFill::south_one(pos.white.king) & target_mask;
    sw_move_targets |= SlideFill::sw_one(pos.white.king) & target_mask;
    nw_move_targets |= SlideFill::nw_one(pos.white.king) & target_mask;

    // my solution to king-side castling
    let mut east_king_moves = SlideFill::east_one(pos.white.king) & target_mask;
    east_king_moves |= SlideFill::east_one(east_king_moves)
        & (pos.castles & !attacks & _null_if_check & !pos.get_all_pieces());
    let mut west_king_moves = SlideFill::west_one(pos.white.king) & target_mask;
    let west_castle_mask = !pos.get_all_pieces() & !(pos.get_all_pieces() << 1);
    west_king_moves |= SlideFill::west_one(west_king_moves)
        & (pos.castles & !attacks & _null_if_check)
        & west_castle_mask;

    east_move_targets |= east_king_moves;
    west_move_targets |= west_king_moves;

    TargetBoards {
        north: north_move_targets,
        nne: nne_move_targets,
        ne: ne_move_targets,
        ene: ene_move_targets,
        east: east_move_targets,
        ese: ese_move_targets,
        se: se_move_targets,
        sse: sse_move_targets,
        south: south_move_targets,
        ssw: ssw_move_targets,
        sw: sw_move_targets,
        wsw: wsw_move_targets,
        west: west_move_targets,
        wnw: wnw_move_targets,
        nw: nw_move_targets,
        nnw: nnw_move_targets,
    }
}

const MAGIC_ARRAY: [BoardIndex; 64] = [
    0, 1, 48, 2, 57, 49, 28, 3, 61, 58, 50, 42, 38, 29, 17, 4, 62, 55, 59, 36, 53, 51, 43, 22, 45,
    39, 33, 30, 24, 18, 12, 5, 63, 47, 56, 27, 60, 41, 37, 16, 54, 35, 52, 21, 44, 32, 23, 11, 46,
    26, 40, 15, 34, 20, 31, 10, 25, 14, 19, 9, 13, 8, 7, 6,
];
const DE_BRUIJN_SEQ: Bits = 0x03f79d71b4cb0a89;

#[allow(arithmetic_overflow)]
fn bitscan(board: Bits) -> BoardIndex {
    MAGIC_ARRAY[((Wrapping(least_set_one_bit(board)) * Wrapping(DE_BRUIJN_SEQ)).0 >> 58) as usize]
}

pub fn generate_legal_moves(_pos: &Position) -> MoveList {
    let pos = match _pos.turn {
        Color::White => _pos.clone(),
        Color::Black => {
            let mut pos = _pos.mirror();
            pos.turn.flip();
            pos
        }
    };

    let mut move_list = MoveList::default();

    // generate move target bitboards
    let move_target_bitboards = generate_white_legal_target_bitboards(&pos);
    let all_pieces: Bits = pos.get_all_pieces();

    // north
    let mut north_move_targets = move_target_bitboards.north;
    while north_move_targets != 0 {
        let target = least_set_one_bit(north_move_targets);
        let piece = SlideFill::south_blocker(target, !all_pieces);

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        if pos.white.pawns & piece != 0 {
            let diff: i8 = to as i8 - from as i8;

            if to >= 56 {
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::KNIGHT_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::BISHOP_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::ROOK_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::QUEEN_PROMOTION | capture_flag,
                ));
            } else if target != 0 && diff == 16 {
                assert!(capture_flag == 0);
                move_list.append(Move::new(to, from, MoveFlag::DOUBLE_PAWN_PUSH));
            } else {
                move_list.append(Move::new(to, from, capture_flag));
            }
        } else {
            move_list.append(Move::new(to, from, capture_flag));
        }

        north_move_targets &= north_move_targets - 1;
    }

    // north north east
    let mut nne_move_targets = move_target_bitboards.nne;
    while nne_move_targets != 0 {
        let target = least_set_one_bit(nne_move_targets);

        let from = bitscan(KnightFill::ssw(target)); // don't need an ssw_blocker func
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        nne_move_targets &= nne_move_targets - 1;
    }

    // north east
    let mut ne_move_targets = move_target_bitboards.ne;
    while ne_move_targets != 0 {
        let target = least_set_one_bit(ne_move_targets); // bitboard of move target
        let piece = SlideFill::sw_blocker(target, !all_pieces); // bitboard of moving piece

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        if (pos.white.pawns & piece) != 0 {
            // if the moving piece is a pawn
            if to >= 56 {
                // promotion

                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::KNIGHT_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::BISHOP_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::ROOK_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::QUEEN_PROMOTION | capture_flag,
                ));
            } else if pos.en_passant.is_some_and(|val| val == to) {
                // en passant
                // we know it's a capture, so ignore capture_flag
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::EN_PASSANT | MoveFlag::CAPTURE,
                ));
            } else {
                move_list.append(Move::new(to, from, capture_flag));
            }
        } else {
            move_list.append(Move::new(to, from, capture_flag));
        }

        ne_move_targets &= ne_move_targets - 1;
    }

    // east north east
    let mut ene_move_targets = move_target_bitboards.ene;
    while ene_move_targets != 0 {
        let target = least_set_one_bit(ene_move_targets);

        let from = bitscan(KnightFill::wsw(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        ene_move_targets &= ene_move_targets - 1;
    }

    // east
    let mut east_move_targets = move_target_bitboards.east;
    while east_move_targets != 0 {
        let target = least_set_one_bit(east_move_targets);
        let piece = SlideFill::west_blocker(target, !all_pieces);

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        if pos.white.king == piece {
            // if moving piece is a king
            let diff = to - from;

            if diff == 2 {
                move_list.append(Move::new(to, from, MoveFlag::KING_CASTLE));
            } else {
                move_list.append(Move::new(to, from, capture_flag));
            }
        } else {
            move_list.append(Move::new(to, from, capture_flag));
        }

        east_move_targets &= east_move_targets - 1;
    }

    // east south east
    let mut ese_move_targets = move_target_bitboards.ese;
    while ese_move_targets != 0 {
        let target = least_set_one_bit(ese_move_targets);

        let from = bitscan(KnightFill::wnw(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        ese_move_targets &= ese_move_targets - 1;
    }

    // south east
    let mut se_move_targets = move_target_bitboards.se;
    while se_move_targets != 0 {
        let target = least_set_one_bit(se_move_targets);
        let piece = SlideFill::nw_blocker(target, !all_pieces);

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        se_move_targets &= se_move_targets - 1;
    }

    // south south east
    let mut sse_move_targets = move_target_bitboards.sse;
    while sse_move_targets != 0 {
        let target = least_set_one_bit(sse_move_targets);

        let from = bitscan(KnightFill::nnw(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        sse_move_targets &= sse_move_targets - 1;
    }

    // south
    let mut south_move_targets = move_target_bitboards.south;
    while south_move_targets != 0 {
        let target = least_set_one_bit(south_move_targets);
        let piece = SlideFill::north_blocker(target, !all_pieces);

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        south_move_targets &= south_move_targets - 1;
    }

    // south south west
    let mut ssw_move_targets = move_target_bitboards.ssw;
    while ssw_move_targets != 0 {
        let target = least_set_one_bit(ssw_move_targets);

        let from = bitscan(KnightFill::nne(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        ssw_move_targets &= ssw_move_targets - 1;
    }

    // south west
    let mut sw_move_targets = move_target_bitboards.sw;
    while sw_move_targets != 0 {
        let target = least_set_one_bit(sw_move_targets);
        let piece = SlideFill::ne_blocker(target, !all_pieces);

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        sw_move_targets &= sw_move_targets - 1;
    }

    // west south west
    let mut wsw_move_targets = move_target_bitboards.wsw;
    while wsw_move_targets != 0 {
        let target = least_set_one_bit(wsw_move_targets);

        let from = bitscan(KnightFill::ene(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        wsw_move_targets &= wsw_move_targets - 1;
    }

    // west
    let mut west_move_targets = move_target_bitboards.west;
    while west_move_targets != 0 {
        let target = least_set_one_bit(west_move_targets);
        let piece = SlideFill::east_blocker(target, !all_pieces);

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        if pos.white.king == piece {
            // if moving piece is a king
            let diff: i8 = to as i8 - from as i8;

            if diff == -3 {
                move_list.append(Move::new(to, from, MoveFlag::QUEEN_CASTLE));
            } else {
                move_list.append(Move::new(to, from, capture_flag));
            }
        } else {
            move_list.append(Move::new(to, from, capture_flag));
        }

        west_move_targets &= west_move_targets - 1;
    }

    // west north west
    let mut wnw_move_targets = move_target_bitboards.wnw;
    while wnw_move_targets != 0 {
        let target = least_set_one_bit(wnw_move_targets);

        let from = bitscan(KnightFill::ese(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        wnw_move_targets &= wnw_move_targets - 1;
    }

    // north west
    let mut nw_move_targets = move_target_bitboards.nw;
    while nw_move_targets != 0 {
        let target = least_set_one_bit(nw_move_targets); // bitboard of move target
        let piece = SlideFill::se_blocker(target, !all_pieces); // bitboard of moving piece

        let from = bitscan(piece);
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        if (pos.white.pawns & piece) != 0 {
            // if the moving piece is a pawn
            if to >= 56 {
                // promotion
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::KNIGHT_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::BISHOP_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::ROOK_PROMOTION | capture_flag,
                ));
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::PROMOTION | MoveFlag::QUEEN_PROMOTION | capture_flag,
                ));
            } else if pos.en_passant.is_some_and(|val| val == to) {
                // en passant
                // we know it's a capture, so ignore capture_flag
                move_list.append(Move::new(
                    to,
                    from,
                    MoveFlag::EN_PASSANT | MoveFlag::CAPTURE,
                ));
            } else {
                move_list.append(Move::new(to, from, capture_flag));
            }
        } else {
            move_list.append(Move::new(to, from, capture_flag));
        }

        nw_move_targets &= nw_move_targets - 1;
    }

    // north north west
    let mut nnw_move_targets = move_target_bitboards.nnw;
    while nnw_move_targets != 0 {
        let target = least_set_one_bit(nnw_move_targets);

        let from = bitscan(KnightFill::sse(target));
        let to = bitscan(target);

        let capture_flag = ((target & all_pieces) != 0) as u8 * 4;

        move_list.append(Move::new(to, from, capture_flag));

        nnw_move_targets &= nnw_move_targets - 1;
    }

    // match on unmirrored position
    match _pos.turn {
        Color::Black => move_list.flip(),
        Color::White => move_list,
    }
}
