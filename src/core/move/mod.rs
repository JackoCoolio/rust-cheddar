use crate::{
    board::Board,
    position::{index_to_alg, Piece},
};

pub mod gen;
pub mod list;

pub struct MoveFlag;
impl MoveFlag {
    pub const QUIET: u8 = 0x0 << 2;
    pub const CAPTURE: u8 = 0x4;
    pub const PROMOTION: u8 = 0x8;
    pub const DOUBLE_PAWN_PUSH: u8 = 0x1;
    pub const KING_CASTLE: u8 = 0x2;
    pub const QUEEN_CASTLE: u8 = 0x3;
    pub const EN_PASSANT: u8 = 0x1;
    pub const KNIGHT_PROMOTION: u8 = 0x0;
    pub const BISHOP_PROMOTION: u8 = 0x1;
    pub const ROOK_PROMOTION: u8 = 0x2;
    pub const QUEEN_PROMOTION: u8 = 0x3;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Move {
    // flag  from    to
    // XXXX XXXXXX XXXXXX
    pub bits: u16,
}

impl Move {
    pub fn new(to: u8, from: u8, flags: u8) -> Move {
        Move {
            bits: (((flags & 0xf) as u16) << 12)
                | (((from & 0x3f) as u16) << 6)
                | (to as u16 & 0x3f),
        }
    }

    pub fn get_to(&self) -> u8 {
        (self.bits & 0x3f) as u8
    }

    pub fn get_from(&self) -> u8 {
        ((self.bits >> 6) & 0x3f) as u8
    }

    pub fn get_flags(&self) -> u8 {
        ((self.bits >> 12) & 0xf) as u8
    }

    pub fn set_to(&mut self, to: u8) {
        self.bits &= !0x3f;
        self.bits |= to as u16 & 0x3f;
    }

    pub fn set_from(&mut self, from: u8) {
        self.bits &= !(0x3f << 6);
        self.bits |= (from as u16 & 0x3f) << 6;
    }

    pub fn is_capture(&self) -> bool {
        (((MoveFlag::CAPTURE as u16) << 12) & self.bits) != 0
    }

    pub fn get_butterfly_index(&self) -> u8 {
        (self.bits & 0xfff) as u8
    }

    pub fn flip(mov: &Move) -> Move {
        // Move {
        //   bits: ((63 - (mov.get_to() as u16)) & 0x3f)
        //     | ((63 - (mov.get_from() as u16)) & 0x3f)
        //     | ((mov.get_flags() as u16) << 12),
        // }
        Move::new(
            Board::mirror_index(mov.get_to()),
            Board::mirror_index(mov.get_from()),
            mov.get_flags(),
        )
    }

    pub fn is_promotion(&self) -> bool {
        (((MoveFlag::PROMOTION as u16) << 12) & self.bits) != 0
    }

    pub fn get_promotion_piece(&self) -> Option<Piece> {
        if !self.is_promotion() {
            return None;
        }

        // important that queen is checked first, because Q = R | B
        if self.bits & ((MoveFlag::QUEEN_PROMOTION as u16) << 12) != 0 {
            Some(Piece::Queen)
        } else if self.bits & ((MoveFlag::ROOK_PROMOTION as u16) << 12) != 0 {
            Some(Piece::Rook)
        } else if self.bits & ((MoveFlag::BISHOP_PROMOTION as u16) << 12) != 0 {
            Some(Piece::Bishop)
        } else if self.bits & ((MoveFlag::KNIGHT_PROMOTION as u16) << 12) != 0 {
            Some(Piece::Knight)
        } else {
            None
        }
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        // todo: make this algebraic notation
        format!(
            "{} -> {}",
            index_to_alg(self.get_from()),
            index_to_alg(self.get_to())
        )
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Move) -> bool {
        other.bits == self.bits
    }
}
