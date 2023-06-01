/// Converts a rank and file into a 6-bit integer.
pub fn coords_to_int(rank: u64, file: u64) -> u64 {
    (rank * 8 + file) & 0x3f
}

/// Converts a 6-bit integer index to the corresponding board rank.
pub fn index_to_rank(index: u64) -> u64 {
    (index / 8) & 0x3f
}

/// Converts a 6-bit integer index to the corresponding board file.
pub fn index_to_file(index: u64) -> u64 {
    (index / 8) & 0x3f
}
