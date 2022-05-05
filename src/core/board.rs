use std::num::Wrapping;

pub type Bits = u64;
pub type BoardIndex = u8;

pub struct Board;

const K1: Bits = 0x00ff00ff00ff00ff;
const K2: Bits = 0x0000ffff0000ffff;

pub struct Castles;
impl Castles {
  pub const BLACK_KING: Bits = 0x4000000000000000;
  pub const BLACK_QUEEN: Bits = 0x0400000000000000;
  pub const WHITE_KING: Bits = 0x0000000000000040;
  pub const WHITE_QUEEN: Bits = 0x0000000000000004;
}

/// Returns the bit at the specified index (exactly 0 or 1).
/// The return type is Bits for ease of use, but only the least significant bit is relevant.
#[inline(always)]
pub fn get_bit(bits: Bits, index: BoardIndex) -> Bits {
  (bits >> index) & 1
}

/// Sets the bit at the specified index.
#[inline(always)]
pub fn set_bit(bits: &mut Bits, index: BoardIndex) -> () {
  *bits |= 1 << index;
}

/// Clears the bit at the specified index.
#[inline(always)]
pub fn clear_bit(bits: &mut Bits, index: BoardIndex) -> () {
  *bits &= !(1 << index);
}

/// Clears the bit at the specified index and returns it.
pub fn pop_bit(bits: &mut Bits, index: BoardIndex) -> u8 {
  let temp_bits: Bits = *bits & !(1_u64 << index);
  if temp_bits ^ *bits > 0 {
    *bits = temp_bits;
    1
  } else {
    0
  }
}

impl Board {
  /// Reflects the board across the 4th and 5th rank.
  pub fn mirror_board(board: Bits) -> Bits {
    let mut out = board;
    out = ((out >> 8) & K1) | ((out & K1) << 8);
    out = ((out >> 16) & K2) | ((out & K2) << 16);
    out = (out >> 32) | (out << 32);
    out
  }

  /// Returns the index of the square that is opposite the given index.
  pub fn mirror_index(index: u8) -> u8 {
    if index > 63 {
      return index;
    }
    (2 * index) % 16 + 56 - index
  }

  /// Returns a visual representation of a bitboard. 'x' denotes a set bit, and '-' denotes a clear bit.
  pub fn bits_to_string(board: Bits) -> String {
    let mut out = String::new();
    let mut row = String::new();
    for i in (0..64).rev() {
      let bit = get_bit(board, i);

      let c: String = if bit != 0 { "x".into() } else { "-".into() };
      row = c + " " + row.as_str();

      if i % 8 == 0 {
        out += &row;
        out += "\n";
        row.clear();
      }
    }

    out
  }

  /// Counts the number of set bits in the bitboard.
  pub fn get_number_of_pieces(board: Bits) -> u8 {
    let mut count = 0;
    let mut sbits = board as i64;
    while sbits != 0 {
      sbits = sbits & (Wrapping(sbits) - Wrapping(1)).0;
      count += 1;
    }
    count
  }
}
