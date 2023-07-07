use crate::mov::Move;

#[derive(Clone, Debug)]
pub struct MoveList {
    /// The list of moves.
    moves: [Move; 256],
    /// The number of Moves in the list.
    count: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [Default::default(); 256],
            count: 0,
        }
    }
}

impl MoveList {
    pub fn append(&mut self, mov: Move) {
        self.moves[self.count] = mov;
        self.count += 1;
    }

    pub fn flip(&mut self) -> Self {
        let mut flipped = MoveList::default();
        for i in 0..self.count {
            flipped.append(Move::flip(&self.moves[i]));
        }
        flipped
    }

    pub fn contains(&self, mov: &Move) -> bool {
        for i in 0..self.count {
            if &self.moves[i] == mov {
                return true;
            }
        }
        false
    }

    pub fn get_moves(&self) -> &[Move; 256] {
        &self.moves
    }

    pub fn get_moves_mut(&mut self) -> &mut [Move; 256] {
        &mut self.moves
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl ToString for MoveList {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for mov in self.clone().into_iter() {
            out += &(mov.to_string() + "\n");
        }
        out
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = MoveListIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        MoveListIntoIterator {
            moves: self.moves,
            index: 0,
            count: self.count,
        }
    }
}

pub struct MoveListIntoIterator {
    moves: [Move; 256],
    index: usize,
    count: usize,
}

impl Iterator for MoveListIntoIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            let mov = self.moves[self.index];
            self.index += 1;
            Some(mov)
        }
    }
}
