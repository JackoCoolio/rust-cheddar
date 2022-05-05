use crate::board::Bits;

use super::{A_FILE, B_FILE, G_FILE, H_FILE};

pub struct KnightFill;
impl KnightFill {
  pub fn nne(knights: Bits) -> Bits {
    (knights & !H_FILE) << 17
  }

  pub fn ene(knights: Bits) -> Bits {
    (knights & !G_FILE & !H_FILE) << 10
  }

  pub fn ese(knights: Bits) -> Bits {
    (knights & !G_FILE & !H_FILE) >> 6
  }

  pub fn sse(knights: Bits) -> Bits {
    (knights & !H_FILE) >> 15
  }

  pub fn ssw(knights: Bits) -> Bits {
    (knights & !A_FILE) >> 17
  }

  pub fn wsw(knights: Bits) -> Bits {
    (knights & !A_FILE & !B_FILE) >> 10
  }

  pub fn wnw(knights: Bits) -> Bits {
    (knights & !A_FILE & !B_FILE) << 6
  }

  pub fn nnw(knights: Bits) -> Bits {
    (knights & !A_FILE) << 15
  }
}

pub fn knight_fill(knights: Bits) -> Bits {
  KnightFill::nne(knights)
    | KnightFill::ene(knights)
    | KnightFill::ese(knights)
    | KnightFill::sse(knights)
    | KnightFill::ssw(knights)
    | KnightFill::wsw(knights)
    | KnightFill::wnw(knights)
    | KnightFill::nnw(knights)
}
