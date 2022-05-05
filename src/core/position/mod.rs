use crate::{
  board::{clear_bit, get_bit, set_bit, Bits, Board, BoardIndex},
  r#move::{gen::generate_legal_moves, list::MoveList, Move},
};

pub mod fen;

/// Black or white.
#[derive(Clone, Debug)]
pub enum Color {
  White = 1,
  Black = 0,
}

impl Color {
  /// Returns the opposite color.
  pub fn opposite(&self) -> Color {
    match *self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }

  /// Flips this color to the opposite color.
  pub fn flip(&mut self) -> &Self {
    *self = self.opposite();
    self
  }
}

/// A piece on a chess board.
#[derive(Debug)]
pub enum Piece {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

pub type ColoredPiece = (Color, Piece);

/// Piece positions for one color.
#[derive(Clone, Debug)]
pub struct PositionPieces {
  pub pawns: Bits,
  pub knights: Bits,
  pub bishops: Bits,
  pub rooks: Bits,
  pub queens: Bits,
  pub king: Bits,
}

impl PositionPieces {
  /// Returns an empty board.
  pub fn empty() -> PositionPieces {
    PositionPieces {
      pawns: 0,
      knights: 0,
      bishops: 0,
      rooks: 0,
      queens: 0,
      king: 0,
    }
  }

  /// Returns the bitwise OR of all bitboards of this color.
  pub fn get_all_pieces(&self) -> Bits {
    self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
  }

  /// Reflects the board across the 4th and 5th rank.
  pub fn mirror(&self) -> PositionPieces {
    PositionPieces {
      pawns: Board::mirror_board(self.pawns),
      knights: Board::mirror_board(self.knights),
      bishops: Board::mirror_board(self.bishops),
      rooks: Board::mirror_board(self.rooks),
      queens: Board::mirror_board(self.queens),
      king: Board::mirror_board(self.king),
    }
  }

  pub fn get_number_of_pieces(&self) -> u8 {
    Board::get_number_of_pieces(self.get_all_pieces())
  }
}

/// Converts an algebraic notation square to its board index.
pub fn alg_to_index(s: &str) -> Result<BoardIndex, String> {
  if s.len() != 2 {
    return Err("algebraic notation must be 2 characters".to_string());
  }

  let mut chars = s.chars();

  let file = chars.next().unwrap() as u8 - 'a' as u8;
  if file > 7 {
    return Err("invalid file char".to_string());
  }

  let rank = chars.next().unwrap() as u8 - '1' as u8;
  if rank > 7 {
    return Err("invalid rank char".to_string());
  }

  debug_assert!(rank * 8 + file < 64);
  Ok(rank * 8 + file)
}

pub fn index_to_alg(index: BoardIndex) -> String {
  assert!(index < 64);
  let rank = index / 8;
  let file = index % 8;

  let rank_char = ('1' as u8 + rank) as char;
  let file_char = ('a' as u8 + file) as char;

  let mut out = String::new();
  out.push(file_char);
  out.push(rank_char);

  out
}

/// An instance of a board position.
/// Stores information like piece positions, available castles, and current turn count.
#[derive(Clone, Debug)]
pub struct Position {
  pub white: PositionPieces,
  pub black: PositionPieces,
  pub en_passant: BoardIndex,
  pub castles: Bits,
  pub turn: Color,
  move_cache: Option<MoveList>,
}

impl Position {
  pub const NO_EN_PASSANT: u8 = 255;
  #[allow(dead_code)]
  pub const STANDARD_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";

  /// Returns this position, mirrored across the 4th and 5th rank. Clears the move cache.
  pub fn mirror(self: &Position) -> Position {
    Position {
      white: self.black.mirror(),
      black: self.white.mirror(),
      en_passant: Board::mirror_index(self.en_passant),
      castles: Board::mirror_board(self.castles),
      turn: self.turn.clone(),
      move_cache: None, // discard move cache
    }
  }

  pub fn get_all_pieces(&self) -> Bits {
    self.white.get_all_pieces() | self.black.get_all_pieces()
  }

  /// Sets the specified square to a `color` `piece`. Clears the move cache.
  fn set_square(&mut self, index: BoardIndex, color: Color, piece: Piece) -> () {
    self.clear_square(index);
    let pieces = match color {
      Color::Black => &mut self.black,
      Color::White => &mut self.white,
    };

    match piece {
      Piece::Pawn => set_bit(&mut pieces.pawns, index),
      Piece::Knight => set_bit(&mut pieces.knights, index),
      Piece::Bishop => set_bit(&mut pieces.bishops, index),
      Piece::Rook => set_bit(&mut pieces.rooks, index),
      Piece::Queen => set_bit(&mut pieces.queens, index),
      Piece::King => set_bit(&mut pieces.king, index),
    };

    self.move_cache = None;
  }

  /// Clears the specified square.
  /// Note: this function assumes that the current position is valid (<= 1 piece per square).
  /// Also clears the move cache.
  fn clear_square(&mut self, index: BoardIndex) -> () {
    clear_bit(&mut self.white.pawns, index);
    clear_bit(&mut self.white.knights, index);
    clear_bit(&mut self.white.bishops, index);
    clear_bit(&mut self.white.rooks, index);
    clear_bit(&mut self.white.queens, index);
    clear_bit(&mut self.white.king, index); // idk why we would clear king, but why not :)

    clear_bit(&mut self.black.pawns, index);
    clear_bit(&mut self.black.knights, index);
    clear_bit(&mut self.black.bishops, index);
    clear_bit(&mut self.black.rooks, index);
    clear_bit(&mut self.black.queens, index);
    clear_bit(&mut self.black.king, index);

    self.move_cache = None;
  }

  /// Returns the color and piece of a specified square or None if there is no piece there.
  fn get_square(&self, index: BoardIndex) -> Option<ColoredPiece> {
    // if there is no white piece here, this saves time
    if self.white.get_all_pieces() & (1_u64 << index) != 0 {
      if get_bit(self.white.pawns, index) != 0 {
        return Some((Color::White, Piece::Pawn));
      }
      if get_bit(self.white.knights, index) != 0 {
        return Some((Color::White, Piece::Knight));
      }
      if get_bit(self.white.bishops, index) != 0 {
        return Some((Color::White, Piece::Bishop));
      }
      if get_bit(self.white.rooks, index) != 0 {
        return Some((Color::White, Piece::Rook));
      }
      if get_bit(self.white.queens, index) != 0 {
        return Some((Color::White, Piece::Queen));
      }
      if get_bit(self.white.king, index) != 0 {
        return Some((Color::White, Piece::King));
      }
    // likewise, if there is no black piece, this saves time
    } else if self.black.get_all_pieces() & (1_u64 << index) != 0 {
      if get_bit(self.black.pawns, index) != 0 {
        return Some((Color::Black, Piece::Pawn));
      }
      if get_bit(self.black.knights, index) != 0 {
        return Some((Color::Black, Piece::Knight));
      }
      if get_bit(self.black.bishops, index) != 0 {
        return Some((Color::Black, Piece::Bishop));
      }
      if get_bit(self.black.rooks, index) != 0 {
        return Some((Color::Black, Piece::Rook));
      }
      if get_bit(self.black.queens, index) != 0 {
        return Some((Color::Black, Piece::Queen));
      }
      if get_bit(self.black.king, index) != 0 {
        return Some((Color::Black, Piece::King));
      }
    }

    // so if there is no piece at all, we save 12 calls to get_bit()
    None
  }

  /// Clears the specified square and returns the piece that was previously there.
  /// Note: this function assumes that the current position is valid (<= 1 piece per square).
  /// Clears the move cache.
  fn pop_square(&mut self, index: BoardIndex) -> Option<ColoredPiece> {
    let piece = self.get_square(index);
    if piece.is_none() {
      return None;
    }

    self.clear_square(index);
    piece
  }

  /// Returns true if the given move is valid in this position.
  pub fn validate_move(&self, mov: &Move) -> bool {
    let moves = match &self.move_cache {
      None => {
        let _moves = generate_legal_moves(self);
        _moves
      }
      Some(x) => x.clone(),
    };
    moves.contains(mov)
  }

  /// Applies the given move to the position.
  /// Assumes the move is valid.
  /// Returns the new Position and the captured piece, if there was one.
  /// Clears the move cache.
  pub fn apply_move(&self, mov: &Move) -> (Position, Option<Piece>) {
    let mut pos: Position = self.clone();
    let capture: Option<Piece> = if mov.is_capture() {
      Some(
        pos
          .pop_square(mov.get_to())
          .expect("tried to capture an empty square")
          .1,
      )
    } else {
      None
    };

    match pos.pop_square(mov.get_from()) {
      Some((_, Piece::Pawn)) => {
        let piece = match mov.get_promotion_piece() {
          None => Piece::Pawn,
          Some(p) => p,
        };
        pos.set_square(mov.get_to(), pos.turn.clone(), piece);
      }
      Some((_, p)) => pos.set_square(mov.get_to(), pos.turn.clone(), p),
      None => panic!(
        "tried to apply an invalid move for {:?}: {} in position:\n{}",
        pos.turn,
        mov.to_string(),
        self.to_string()
      ),
    }

    // flip color
    pos.turn.flip();
    (pos, capture)
  }
}

impl Default for Position {
  fn default() -> Self {
    Position {
      white: PositionPieces::empty(),
      black: PositionPieces::empty(),
      en_passant: Position::NO_EN_PASSANT,
      castles: 0,
      turn: Color::Black,
      move_cache: None,
    }
  }
}

impl ToString for Position {
  fn to_string(&self) -> String {
    let mut out = String::new();
    let mut row = String::new();

    for i in (0..64).rev() {
      let piece = self.get_square(i);

      let c: String = match piece {
        None => "-",
        Some((Color::White, Piece::Pawn)) => "P",
        Some((Color::White, Piece::Knight)) => "N",
        Some((Color::White, Piece::Bishop)) => "B",
        Some((Color::White, Piece::Rook)) => "R",
        Some((Color::White, Piece::Queen)) => "Q",
        Some((Color::White, Piece::King)) => "K",
        Some((Color::Black, Piece::Pawn)) => "p",
        Some((Color::Black, Piece::Knight)) => "n",
        Some((Color::Black, Piece::Bishop)) => "b",
        Some((Color::Black, Piece::Rook)) => "r",
        Some((Color::Black, Piece::Queen)) => "q",
        Some((Color::Black, Piece::King)) => "k",
      } // &str
      .into();

      row = c + if i % 8 == 7 { "" } else { " " } + row.as_str();

      if i % 8 == 0 {
        out += &row;
        out += "\n";
        row.clear();
      }
    }

    out
  }
}
