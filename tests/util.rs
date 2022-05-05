#[allow(dead_code)]
pub enum FenSegment {
  Pieces,
  Turn,
  Castles,
  EnPassant,
  MoveCount,
}

#[derive(Debug)]
struct SplitFen<'a> {
  pieces: &'a str,
  turn: &'a str,
  castles: &'a str,
  en_passant: &'a str,
  move_count: &'a str,
}

fn get_fen_segments(s: &str) -> SplitFen {
  let mut tokens: Vec<&str> = s.split_ascii_whitespace().collect();

  let mut move_count_half = false;
  let mut token_start = 0;
  for (i, c) in s.chars().enumerate() {
    if c == ' ' {
      // skip the space in the middle of the move count
      if tokens.len() == 4 && !move_count_half {
        move_count_half = true;
        continue;
      }

      if i - token_start > 1 {
        tokens.push(&s[token_start..i]);
      }
      token_start = i + 1;
    }
  }

  SplitFen {
    pieces: tokens[0],
    turn: tokens[1],
    castles: tokens[2],
    en_passant: tokens[3],
    move_count: tokens[4],
  }
}

pub fn assert_fen_eq(a: &str, b: &str, segments: Vec<FenSegment>) -> () {
  let a_split = get_fen_segments(a);
  let b_split = get_fen_segments(b);
  println!("{:?} {:?}", a_split, b_split);

  let mut success = true;
  for segment in segments {
    match segment {
      FenSegment::Pieces => success = success && a_split.pieces == b_split.pieces,
      FenSegment::Turn => success = success && a_split.turn == b_split.turn,
      FenSegment::Castles => success = success && a_split.castles == b_split.castles,
      FenSegment::EnPassant => success = success && a_split.en_passant == b_split.en_passant,
      FenSegment::MoveCount => success = success && a_split.move_count == b_split.move_count,
    }
  }

  if !success {
    panic!(
      "assertion failed: `differing fens`\n left: `\"{}\"`,\nright: `\"{}\"`",
      a, b
    );
  }
}
