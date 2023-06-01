use crate::board::Bits;

use super::{ks_dir_fill, ks_dir_fill_neg, A_FILE, H_FILE};

pub struct SlideFill;

impl SlideFill {
    // NORTH

    pub fn north_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open;

        ks_dir_fill(&mut gen, &mut pro, 8);
        gen
    }

    pub fn north_one(board: Bits) -> Bits {
        board << 8
    }

    #[inline]
    pub fn north_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::north_one(SlideFill::north_occl(board, open))
    }

    pub fn north_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::north_attacks(board, open)
    }

    // SOUTH

    pub fn south_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open;

        ks_dir_fill_neg(&mut gen, &mut pro, 8);
        gen
    }

    pub fn south_one(board: Bits) -> Bits {
        board >> 8
    }

    #[inline]
    pub fn south_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::south_one(SlideFill::south_occl(board, open))
    }

    pub fn south_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::south_attacks(board, open)
    }

    // EAST

    pub fn east_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open & !A_FILE;

        ks_dir_fill(&mut gen, &mut pro, 1);
        gen
    }

    pub fn east_one(board: Bits) -> Bits {
        !A_FILE & (board << 1)
    }

    pub fn east_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::east_one(SlideFill::east_occl(board, open))
    }

    pub fn east_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::east_attacks(board, open)
    }

    // WEST

    pub fn west_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open;

        ks_dir_fill_neg(&mut gen, &mut pro, 1);
        gen
    }

    pub fn west_one(board: Bits) -> Bits {
        !H_FILE & (board >> 1)
    }

    pub fn west_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::west_one(SlideFill::west_occl(board, open))
    }

    pub fn west_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::west_attacks(board, open)
    }

    // NORTH EAST

    pub fn ne_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open & !A_FILE;

        ks_dir_fill(&mut gen, &mut pro, 9);
        gen
    }

    pub fn ne_one(board: Bits) -> Bits {
        !A_FILE & (board << 9)
    }

    pub fn ne_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::ne_one(SlideFill::ne_occl(board, open))
    }

    pub fn ne_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::ne_attacks(board, open)
    }

    // SOUTH EAST

    pub fn se_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open & !A_FILE;

        ks_dir_fill_neg(&mut gen, &mut pro, 7);
        gen
    }

    pub fn se_one(board: Bits) -> Bits {
        !A_FILE & (board >> 7)
    }

    pub fn se_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::se_one(SlideFill::se_occl(board, open))
    }

    pub fn se_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::se_attacks(board, open)
    }

    // SOUTH WEST

    pub fn sw_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open & !H_FILE;

        ks_dir_fill_neg(&mut gen, &mut pro, 9);
        gen
    }

    pub fn sw_one(board: Bits) -> Bits {
        !H_FILE & (board >> 9)
    }

    pub fn sw_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::sw_one(SlideFill::sw_occl(board, open))
    }

    pub fn sw_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::sw_attacks(board, open)
    }

    // NORTH WEST

    pub fn nw_occl(board: Bits, open: Bits) -> Bits {
        let mut gen = board;
        let mut pro = open & !H_FILE;

        ks_dir_fill(&mut gen, &mut pro, 7);
        gen
    }

    pub fn nw_one(board: Bits) -> Bits {
        !H_FILE & (board << 7)
    }

    pub fn nw_attacks(board: Bits, open: Bits) -> Bits {
        SlideFill::nw_one(SlideFill::nw_occl(board, open))
    }

    pub fn nw_blocker(board: Bits, open: Bits) -> Bits {
        !open & SlideFill::nw_attacks(board, open)
    }
}
