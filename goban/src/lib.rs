#![allow(clippy::needless_return)]

use std::collections::HashSet;

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

    /// Get a [Group] that contains the given point
    pub fn get_group(&self, x: usize, y: usize) -> Result<Group> {
        let mut group = Group {
            color: self.get(x, y)?,
            points: HashSet::new(),
            outside: HashSet::new(),
        };

        self.build_group(&mut group, (x, y));

        return Ok(group);
    }

    /// Assumes p is in group.points
    fn build_group(&self, group: &mut Group, p: (usize, usize)) {
        let left_edge = p.0 == 0;
        let top_edge = p.1 == 0;

        if !left_edge {
            if !group.categorized((p.0 - 1, p.1)) {
                if self.get(p.0 - 1, p.1) == Ok(group.color) {
                    group.points.insert((p.0 - 1, p.1));
                    self.build_group(group, (p.0 - 1, p.1))
                } else {
                    group.outside.insert((p.0 - 1, p.1));
                }
            }
        }

        if !top_edge {
            if !group.categorized((p.0, p.1 - 1)) {
                if self.get(p.0, p.1 - 1) == Ok(group.color) {
                    group.points.insert((p.0, p.1 - 1));
                    self.build_group(group, (p.0, p.1 - 1))
                } else {
                    group.outside.insert((p.0, p.1 - 1));
                }
            }
        }

        match self.get(p.0 + 1, p.1) {
            Err(_) => {}
            Ok(s) => {
                if !group.categorized((p.0 + 1, p.1)) {
                    if s == group.color {
                        group.points.insert((p.0 + 1, p.1));
                        self.build_group(group, (p.0 + 1, p.1))
                    } else {
                        group.outside.insert((p.0 + 1, p.1));
                    }
                }
            }
        }

        match self.get(p.0, p.1 + 1) {
            Err(_) => {}
            Ok(s) => {
                if !group.categorized((p.0, p.1 + 1)) {
                    if s == group.color {
                        group.points.insert((p.0, p.1 + 1));
                        self.build_group(group, (p.0, p.1 + 1))
                    } else {
                        group.outside.insert((p.0, p.1 + 1));
                    }
                }
            }
        }
    }
}

/// Blank 19x19 board
impl Default for Board {
    fn default() -> Self {
        Self::empty(19, 19)
    }
}

/// A set of connected stones of the same color
pub struct Group {
    pub color: Stone,
    /// Points we know are inside the group.
    pub points: HashSet<(usize, usize)>,
    /// Points we know are outside of the group.
    pub outside: HashSet<(usize, usize)>,
}
impl Group {
    pub fn categorized(&self, p: (usize, usize)) -> bool {
        self.points.contains(&p) || self.outside.contains(&p)
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

    #[test]
    fn center_group() {
        let mut board = Board::empty(9, 9);

        // + + + + + + + + +
        // + + + + + + + + +
        // + + + b b + + + +
        // + + + + b + + + +
        // + + + + b b + + +
        // + + + + + b + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +

        let mut points_in_group: HashSet<(usize, usize)> = HashSet::new();

        points_in_group.insert((3, 2));
        points_in_group.insert((4, 2));
        points_in_group.insert((4, 3));
        points_in_group.insert((4, 4));
        points_in_group.insert((5, 4));
        points_in_group.insert((5, 5));

        let rules = Rules {};

        for p in &points_in_group {
            board
                .play(p.0, p.1, Stone::Black, &rules)
                .expect("Failed to play");
        }

        let group = board.get_group(3, 2).expect("Failed to create group");

        assert_eq!(group.points, points_in_group);
    }

    #[test]
    fn left_group() {
        let mut board = Board::empty(9, 9);

        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + b b + + + + +
        // b b b b + + + + +
        // b b + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +

        let mut points_in_group: HashSet<(usize, usize)> = HashSet::new();

        points_in_group.insert((0, 4));
        points_in_group.insert((0, 5));
        points_in_group.insert((1, 4));
        points_in_group.insert((1, 5));
        points_in_group.insert((2, 3));
        points_in_group.insert((2, 4));
        points_in_group.insert((3, 3));
        points_in_group.insert((3, 4));

        let rules = Rules {};

        for p in &points_in_group {
            board
                .play(p.0, p.1, Stone::Black, &rules)
                .expect("Failed to play");
        }

        let group = board.get_group(0, 4).expect("Failed to create group");

        assert_eq!(group.points, points_in_group);
    }

    #[test]
    fn right_group() {
        let mut board = Board::empty(9, 9);

        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + b +
        // + + + + + + b b b
        // + + + b b + + + b
        // + + + b b b b b b
        // + + + + + + + + +
        // + + + + + + + + +

        let mut points_in_group: HashSet<(usize, usize)> = HashSet::new();

        points_in_group.insert((8, 4));
        points_in_group.insert((8, 5));
        points_in_group.insert((8, 6));
        points_in_group.insert((7, 3));
        points_in_group.insert((7, 4));
        points_in_group.insert((7, 6));
        points_in_group.insert((6, 4));
        points_in_group.insert((6, 6));
        points_in_group.insert((5, 6));
        points_in_group.insert((4, 6));
        points_in_group.insert((4, 5));
        points_in_group.insert((3, 6));
        points_in_group.insert((3, 5));

        let rules = Rules {};

        for p in &points_in_group {
            board
                .play(p.0, p.1, Stone::Black, &rules)
                .expect("Failed to play");
        }

        let group = board.get_group(8, 4).expect("Failed to create group");

        assert_eq!(group.points, points_in_group);
    }

    #[test]
    fn round_group() {
        let mut board = Board::empty(9, 9);

        // b b b b b b b b b
        // b + + + + + + + b
        // b + + + + + + + b
        // b + + + + + + + b
        // b + + + + + + + b
        // b + + + + + + + b
        // b + + + + + + + b
        // b + + + + + + + b
        // b b b b b b b b b

        let mut points_in_group: HashSet<(usize, usize)> = HashSet::new();

        points_in_group.insert((0, 0));
        points_in_group.insert((0, 1));
        points_in_group.insert((0, 2));
        points_in_group.insert((0, 3));
        points_in_group.insert((0, 4));
        points_in_group.insert((0, 5));
        points_in_group.insert((0, 6));
        points_in_group.insert((0, 7));
        points_in_group.insert((0, 8));
        points_in_group.insert((1, 0));
        points_in_group.insert((2, 0));
        points_in_group.insert((3, 0));
        points_in_group.insert((4, 0));
        points_in_group.insert((5, 0));
        points_in_group.insert((6, 0));
        points_in_group.insert((7, 0));
        points_in_group.insert((8, 0));
        points_in_group.insert((8, 1));
        points_in_group.insert((8, 2));
        points_in_group.insert((8, 3));
        points_in_group.insert((8, 4));
        points_in_group.insert((8, 5));
        points_in_group.insert((8, 6));
        points_in_group.insert((8, 7));
        points_in_group.insert((8, 8));
        points_in_group.insert((1, 8));
        points_in_group.insert((2, 8));
        points_in_group.insert((3, 8));
        points_in_group.insert((4, 8));
        points_in_group.insert((5, 8));
        points_in_group.insert((6, 8));
        points_in_group.insert((6, 8));
        points_in_group.insert((7, 8));

        let rules = Rules {};

        for p in &points_in_group {
            board
                .play(p.0, p.1, Stone::Black, &rules)
                .expect("Failed to play");
        }

        let group = board.get_group(0, 0).expect("Failed to create group");

        assert_eq!(group.points, points_in_group);
    }
}
