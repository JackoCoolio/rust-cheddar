use crate::board::Bits;

use self::slide::SlideFill;

pub mod knight;
pub mod slide;

const A_FILE: Bits = 0x0101010101010101;
const B_FILE: Bits = 0x0202020202020202;
const G_FILE: Bits = 0x4040404040404040;
const H_FILE: Bits = 0x8080808080808080;
// const FIRST_RANK: Bits = 0x00000000000000ff;
// const EIGHTH_RANK: Bits = 0xff00000000000000;

fn ks_dir_fill(gen: &mut Bits, pro: &mut Bits, dir: u8) -> () {
  *gen |= *pro & (*gen << dir);
  *pro &= *pro << dir;
  *gen |= *pro & (*gen << (dir * 2));
  *pro &= *pro << (dir * 2);
  *gen |= *pro & (*gen << (dir * 4));
}

fn ks_dir_fill_neg(gen: &mut Bits, pro: &mut Bits, dir: u8) -> () {
  *gen |= *pro & (*gen >> dir);
  *pro &= *pro >> dir;
  *gen |= *pro & (*gen >> (dir * 2));
  *pro &= *pro >> (dir * 2);
  *gen |= *pro & (*gen >> (dir * 4));
}

// KING

pub fn king_fill(king: Bits) -> Bits {
  let mut out = king | SlideFill::east_one(king) | SlideFill::west_one(king);
  out |= SlideFill::north_one(out) | SlideFill::south_one(out);

  out ^ king
}

// PAWNS

pub fn north_pawn_attacks(board: Bits) -> Bits {
  SlideFill::nw_one(board) | SlideFill::ne_one(board)
}

pub fn south_pawn_attacks(board: Bits) -> Bits {
  SlideFill::sw_one(board) | SlideFill::se_one(board)
}
