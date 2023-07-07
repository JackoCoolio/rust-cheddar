use crate::{
    board::{get_bit, set_bit, BoardIndex, Castles},
    position::Color,
};

use super::{index_to_alg, Position};

impl Position {
    /// Initalizes a Position from a FEN string.
    pub fn from_fen(fen: String) -> Result<Position, String> {
        let mut pos: Position = Default::default();

        let mut stage = 0;
        let mut skip_offset: i32 = 0;
        let mut ep_file: Option<u8> = None;

        for (i, c) in fen.chars().enumerate() {
            if !c.is_ascii() {
                return Err(format!(
                    "fen parsing encountered non-ASCII symbol '{}' at index {}",
                    c, i
                ));
            }

            if c == ' ' {
                stage += 1;
            } else {
                match stage {
                    0 => {
                        if c == '/' {
                            skip_offset -= 1;
                            continue;
                        }

                        match c.to_digit(10) {
                            Some(x) => {
                                skip_offset += x as i32 - 1;
                            }
                            None => {
                                let index: u8 = (2 * ((i as i32 + skip_offset) % 8) + 56
                                    - i as i32
                                    - skip_offset)
                                    as u8;
                                match c {
                                    'p' => set_bit(&mut pos.black.pawns, index),
                                    'n' => set_bit(&mut pos.black.knights, index),
                                    'b' => set_bit(&mut pos.black.bishops, index),
                                    'r' => set_bit(&mut pos.black.rooks, index),
                                    'q' => set_bit(&mut pos.black.queens, index),
                                    'k' => set_bit(&mut pos.black.king, index),
                                    'P' => set_bit(&mut pos.white.pawns, index),
                                    'N' => set_bit(&mut pos.white.knights, index),
                                    'B' => set_bit(&mut pos.white.bishops, index),
                                    'R' => set_bit(&mut pos.white.rooks, index),
                                    'Q' => set_bit(&mut pos.white.queens, index),
                                    'K' => set_bit(&mut pos.white.king, index),
                                    _ => {
                                        return Err(format!(
                      "fen parsing (stage 0) encountered invalid symbol '{}' at index {}",
                      c, i
                    ))
                                    }
                                }
                            }
                        }
                    }
                    1 => {
                        pos.turn = match c {
                            'b' => Color::Black,
                            'w' => Color::White,
                            _ => {
                                return Err(format!(
                  "fen parsing (stage 1) encountered invalid symbol '{}' at index {}",
                  c, i
                ))
                            }
                        }
                    }
                    2 => {
                        if c == '-' {
                            stage += 1;
                            continue;
                        }

                        pos.castles |= match c {
                            'k' => Castles::BLACK_KING,
                            'q' => Castles::BLACK_QUEEN,
                            'K' => Castles::WHITE_KING,
                            'Q' => Castles::WHITE_QUEEN,
                            _ => {
                                return Err(format!(
                  "fen parsing (stage 2) encountered invalid symbol '{}' at index {}",
                  c, i
                ))
                            }
                        }
                    }
                    3 => {
                        if c == '-' {
                            continue;
                        }

                        if c.is_alphabetic() && ep_file.is_none() {
                            ep_file = Some(c.to_ascii_lowercase() as u8 - b'a');
                        } else if let Some(ep_file) = ep_file {
                            match c.to_digit(10) {
                                Some(x) => {
                                    pos.en_passant =
                                        Some((x as BoardIndex - 1) * 8 + ep_file as BoardIndex);
                                }
                                None => {
                                    return Err(format!(
                    "fen parsing (stage 3) encountered invalid symbol '{}' at index {}",
                    c, i
                  ))
                                }
                            }
                        } else {
                            return Err("invalid en passant formatting".to_string());
                        }
                    }
                    _ => break,
                }
            }
        }

        Ok(pos)
    }

    /// Constructs this position's FEN string.
    pub fn to_fen(&self) -> String {
        let mut out = String::new();

        let mut blanks: u8 = 0;
        for i in 0..64 {
            let index = 2 * (i % 8) + 56 - i;

            if blanks != 0 && (get_bit(self.get_all_pieces(), index) != 0 || i % 8 == 0) {
                out.push((blanks + b'0') as char);
                blanks = 0;
            }

            if i % 8 == 0 && i != 0 {
                out.push('/');
            }

            if get_bit(self.white.pawns, index) != 0 {
                out.push('P');
            } else if get_bit(self.white.knights, index) != 0 {
                out.push('N');
            } else if get_bit(self.white.bishops, index) != 0 {
                out.push('B');
            } else if get_bit(self.white.rooks, index) != 0 {
                out.push('R');
            } else if get_bit(self.white.queens, index) != 0 {
                out.push('Q');
            } else if get_bit(self.white.king, index) != 0 {
                out.push('K');
            } else if get_bit(self.black.pawns, index) != 0 {
                out.push('p');
            } else if get_bit(self.black.knights, index) != 0 {
                out.push('n');
            } else if get_bit(self.black.bishops, index) != 0 {
                out.push('b');
            } else if get_bit(self.black.rooks, index) != 0 {
                out.push('r');
            } else if get_bit(self.black.queens, index) != 0 {
                out.push('q');
            } else if get_bit(self.black.king, index) != 0 {
                out.push('k');
            } else {
                blanks += 1;
            }
        }

        if blanks > 0 {
            out.push((blanks + b'0') as char);
        }

        out.push(' ');

        // turn section
        out.push(match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        });

        out.push(' ');

        if self.castles == 0 {
            out.push('-');
        } else {
            if Castles::WHITE_KING & self.castles != 0 {
                out.push('K');
            }
            if Castles::WHITE_QUEEN & self.castles != 0 {
                out.push('Q');
            }
            if Castles::BLACK_KING & self.castles != 0 {
                out.push('k');
            }
            if Castles::BLACK_QUEEN & self.castles != 0 {
                out.push('q');
            }
        }

        out.push(' ');

        if let Some(en_passant) = self.en_passant {
            out.push_str(&index_to_alg(en_passant));
        } else {
            out.push('-');
        }

        out
    }
}
