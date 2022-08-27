#![allow(clippy::needless_return)]

pub mod result;
pub mod rule;

pub use result::{Error, Result};
use rule::{IllegalMove, Rules};

/// Represents a point on a [Board]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Stone {
    Empty,
    Black,
    White,
}

/// Represents the current state of a go game
#[derive(Clone, PartialEq)]
pub struct Board {
    stones: Vec<Stone>,
    size: (usize, usize),
}
impl Board {
    /// Return a [Board] filled with Stone::Empty with the given dimensions
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            stones: vec![Stone::Empty; width * height],
            size: (width, height),
        }
    }

    /// Get the index on [self.stones] corresponding to the given coordinates.
    /// Note that `x` and `y` are zero-indexed, starting from the top-left.
    fn index(&self, x: usize, y: usize) -> Result<usize> {
        if x >= self.size.0 {
            return Err(Error::CoordinatesOutOfBounds);
        }
        if y >= self.size.1 {
            return Err(Error::CoordinatesOutOfBounds);
        }

        return Ok(y * self.size.0 + x);
    }

    /// Get the [Stone] at the given coordinate
    /// Note that `x` and `y` are zero-indexed, starting from the top-left.
    pub fn get(&self, x: usize, y: usize) -> Result<Stone> {
        let i = self.index(x, y)?;

        return Ok(self.stones[i]);
    }

    fn set(&mut self, x: usize, y: usize, s: Stone) -> Result<()> {
        let i = self.index(x, y)?;

        self.stones[i] = s;

        Ok(())
    }

    /// Play a move according to the given [Rules].
    /// Note that `x` and `y` are zero-indexed, starting from the top-left.
    pub fn play(&mut self, x: usize, y: usize, s: Stone, _rules: &Rules) -> Result<()> {
        let i = self.index(x, y)?;

        if self.stones[i] != Stone::Empty {
            return Err(Error::IllegalMove(IllegalMove::NonEmptySpace));
        }

        self.set(x, y, s)?;

        Ok(())
    }

    /// Returns the (width, height) of the board
    pub fn size(&self) -> (usize, usize) {
        self.size
    }
}

/// Blank 19x19 board
impl Default for Board {
    fn default() -> Self {
        Self::empty(19, 19)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_index() {
        let board = Board::empty(9, 9);

        // a + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + b + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + c

        // a
        assert_eq!(board.index(0, 0).unwrap(), 0);

        // b
        assert_eq!(board.index(4, 4).unwrap(), 40);

        // c
        assert_eq!(board.index(8, 8).unwrap(), 80);
    }

    #[test]
    fn wrong_index() {
        let board = Board::empty(9, 9);

        // + + + + + + + + + a
        // + + + + + + + + + .
        // + + + + + + + + + .
        // + + + + + + + + + .
        // + + + + + + + + + .
        // + + + + + + + + + .
        // + + + + + + + + + .
        // + + + + + + + + + .
        // + + + + + + + + + .
        // c . . . . . . . . b

        // a
        assert_eq!(board.index(9, 0), Err(Error::CoordinatesOutOfBounds));

        // b
        assert_eq!(board.index(9, 9), Err(Error::CoordinatesOutOfBounds));

        // c
        assert_eq!(board.index(0, 9), Err(Error::CoordinatesOutOfBounds));
    }

    #[test]
    fn non_empty_space() {
        let mut board = Board::empty(9, 9);
        let rules = Rules {};

        board
            .play(0, 0, Stone::White, &rules)
            .expect("failed to play!");

        assert_eq!(
            board.play(0, 0, Stone::White, &rules),
            Err(Error::IllegalMove(IllegalMove::NonEmptySpace))
        );
    }
}
