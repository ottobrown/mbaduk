use std::collections::HashSet;

use crate::rule::{IllegalMove, Rules};
use crate::{Error, Result};

/// Represents a point on a [Board]
#[derive(Clone, Copy, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum Stone {
    Empty,
    Black,
    White,
}
impl std::ops::Not for Stone {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::Empty => Stone::Empty,
        }
    }
}

/// Represents the current state of a go game
#[derive(Clone, PartialEq, Hash)]
pub struct Board {
    stones: Vec<Stone>,
    size: (usize, usize),

    hashes: Vec<u64>,
}
impl Board {
    /// Return a [Board] filled with Stone::Empty with the given dimensions
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            stones: vec![Stone::Empty; width * height],
            size: (width, height),

            hashes: Vec::new(),
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
    pub fn play(&mut self, x: usize, y: usize, s: Stone, rules: &Rules) -> Result<()> {
        let mut new = self.clone();

        let i = new.index(x, y)?;

        if new.stones[i] != Stone::Empty {
            return Err(Error::IllegalMove(IllegalMove::NonEmptySpace));
        }

        new.set(x, y, s)?;

        let group = new.get_group(x, y)?;

        let mut enemy_groups: Vec<Group> = Vec::new();
        let mut categorized: HashSet<(usize, usize)> = HashSet::new();

        for s in group.enemy_neighbors {
            if !categorized.contains(&s) {
                enemy_groups.push(new.get_group(s.0, s.1)?);
            }

            categorized.insert(s);
        }

        for g in enemy_groups {
            if g.liberties.is_empty() {
                new.kill_group(&g)?;
            }
        }

        let group = new.get_group(x, y)?;

        if !rules.suicide_allowed && group.liberties.is_empty() {
            return Err(Error::IllegalMove(IllegalMove::SuicidalMove));
        }

        let hash = fxhash::hash64(&new.stones);

        if Some(&hash) == new.hashes.iter().rev().nth(1) {
            return Err(Error::IllegalMove(IllegalMove::Ko));
        }

        if new.hashes.contains(&hash) && rules.superko {
            return Err(Error::IllegalMove(IllegalMove::SuperKo));
        }

        new.hashes.push(hash);

        *self = new;
        Ok(())
    }

    /// Returns the (width, height) of the board
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    fn kill_group(&mut self, g: &Group) -> Result<()> {
        for s in &g.points {
            self.set(s.0, s.1, Stone::Empty)?;
        }

        Ok(())
    }

    /// Get a [Group] that contains the given point
    pub fn get_group(&self, x: usize, y: usize) -> Result<Group> {
        let mut group = Group {
            color: self.get(x, y)?,
            points: HashSet::new(),
            liberties: HashSet::new(),
            enemy_neighbors: HashSet::new(),
        };

        group.points.insert((x, y));

        self.build_group(&mut group, (x, y));

        return Ok(group);
    }

    fn try_group_point(&self, x: usize, y: usize, group: &mut Group) {
        if group.categorized((x, y)) {
            return;
        }

        let stone = match self.get(x, y) {
            Ok(s) => s,
            Err(_) => return,
        };

        if stone == group.color {
            group.points.insert((x, y));
            self.build_group(group, (x, y))
        } else if stone == Stone::Empty {
            group.liberties.insert((x, y));
        } else {
            group.enemy_neighbors.insert((x, y));
        }
    }

    /// Assumes p is in group.points
    fn build_group(&self, group: &mut Group, p: (usize, usize)) {
        let left_edge = p.0 == 0;
        let top_edge = p.1 == 0;

        if !left_edge {
            self.try_group_point(p.0 - 1, p.1, group);
        }

        if !top_edge {
            self.try_group_point(p.0, p.1 - 1, group);
        }

        self.try_group_point(p.0 + 1, p.1, group);

        self.try_group_point(p.0, p.1 + 1, group);
    }

    pub fn star_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();

        let (w, h) = self.size;

        // if the board has an exact center
        if w % 2 == 1 && h % 2 == 1 {
            // add a center star point
            points.push((w / 2, h / 2));
        }

        if w < 9 || h < 9 {
            return points;
        }

        // 3-3 points
        if w < 13 || h < 13 {
            points.push((2, 2));
            points.push((2, h - 3));
            points.push((w - 3, 2));
            points.push((w - 3, h - 3));

            return points;
        }

        // sides
        if w > 13 {
            if h % 2 == 1 {
                points.push((3, h / 2));
                points.push((w - 4, h / 2));
            }

            if w % 2 == 1 {
                points.push((w / 2, 3));
                points.push((w / 2, h - 4));
            }
        }

        // 4-4 points
        points.push((3, 3));
        points.push((3, h - 4));
        points.push((w - 4, 3));
        points.push((w - 4, h - 4));

        return points;
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
    /// [Empty](Stone::Empty) stones bordering the Group.
    pub liberties: HashSet<(usize, usize)>,
    /// Enemy [Stone]s bordering the Group.
    pub enemy_neighbors: HashSet<(usize, usize)>,
}
impl Group {
    pub fn categorized(&self, p: (usize, usize)) -> bool {
        self.points.contains(&p) || self.liberties.contains(&p) || self.enemy_neighbors.contains(&p)
    }
}

#[cfg(test)]
mod board_tests {
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
        let rules = Rules::JAPANESE;

        board
            .play(0, 0, Stone::White, &rules)
            .expect("failed to play!");

        assert_eq!(
            board.play(0, 0, Stone::White, &rules),
            Err(Error::IllegalMove(IllegalMove::NonEmptySpace))
        );
    }
}

#[cfg(test)]
mod group_tests {
    use super::*;

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

        let rules = Rules::JAPANESE;

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

        let rules = Rules::JAPANESE;

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

        let rules = Rules::JAPANESE;

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

        let rules = Rules::JAPANESE;

        for p in &points_in_group {
            board
                .play(p.0, p.1, Stone::Black, &rules)
                .expect("Failed to play");
        }

        let group = board.get_group(0, 0).expect("Failed to create group");

        assert_eq!(group.points, points_in_group);
    }

    #[test]
    fn group_neighbors() {
        let mut board = Board::empty(9, 9);
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + w b + + + + +
        // + + w b + + + + +
        // + + w b + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +

        let mut black: HashSet<(usize, usize)> = HashSet::new();
        let mut white: HashSet<(usize, usize)> = HashSet::new();

        black.insert((3, 3));
        black.insert((3, 4));
        black.insert((3, 5));

        white.insert((2, 3));
        white.insert((2, 4));
        white.insert((2, 5));

        let rules = Rules::JAPANESE;

        for p in &black {
            board
                .play(p.0, p.1, Stone::Black, &rules)
                .expect("Failed to play");
        }

        for p in &white {
            board
                .play(p.0, p.1, Stone::White, &rules)
                .expect("Failed to play");
        }

        let black_group = board.get_group(3, 3).expect("Failed to create group");
        let white_group = board.get_group(2, 3).expect("Failed to create group");

        assert_eq!(black, black_group.points);
        assert_eq!(white, white_group.points);

        assert_eq!(black, white_group.enemy_neighbors);
        assert_eq!(white, black_group.enemy_neighbors);
    }

    #[test]
    fn single_stone_group() {
        let mut board = Board::empty(9, 9);

        board
            .play(5, 5, Stone::Black, &Rules::JAPANESE)
            .expect("failed to play");

        let mut intended = HashSet::new();
        intended.insert((5, 5));

        let group = board.get_group(5, 5).unwrap();

        assert_eq!(group.points, intended);
    }
}

#[cfg(test)]
mod capturing_tests {
    use super::*;

    #[test]
    fn kill_group_center() -> Result<()> {
        let mut board = Board::empty(9, 9);

        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + b b + + +
        // + + + b w w b + +
        // + + b w w b + + +
        // + + + b b + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +

        let rules = Rules::JAPANESE;

        board.play(3, 4, Stone::White, &rules)?;
        board.play(4, 4, Stone::White, &rules)?;
        board.play(4, 3, Stone::White, &rules)?;
        board.play(5, 3, Stone::White, &rules)?;

        board.play(2, 4, Stone::Black, &rules)?;
        board.play(3, 3, Stone::Black, &rules)?;
        board.play(3, 5, Stone::Black, &rules)?;
        board.play(4, 2, Stone::Black, &rules)?;
        board.play(4, 5, Stone::Black, &rules)?;
        board.play(5, 2, Stone::Black, &rules)?;
        board.play(5, 4, Stone::Black, &rules)?;
        board.play(6, 3, Stone::Black, &rules)?;

        assert_eq!(board.get(3, 4)?, Stone::Empty);
        assert_eq!(board.get(4, 4)?, Stone::Empty);
        assert_eq!(board.get(4, 3)?, Stone::Empty);
        assert_eq!(board.get(5, 3)?, Stone::Empty);

        Ok(())
    }

    #[test]
    fn single_ko() -> Result<()> {
        let mut board = Board::empty(9, 9);

        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + b + + + +
        // + + + b w b + + +
        // + + + w b w + + +
        // + + + + w + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +

        let rules = Rules::JAPANESE;

        board.play(4, 2, Stone::Black, &rules)?;
        board.play(3, 3, Stone::Black, &rules)?;
        board.play(5, 3, Stone::Black, &rules)?;

        board.play(3, 4, Stone::White, &rules)?;
        board.play(4, 5, Stone::White, &rules)?;
        board.play(5, 4, Stone::White, &rules)?;
        board.play(4, 3, Stone::White, &rules)?;

        // Capture the white stone, creating the ko
        board.play(4, 4, Stone::Black, &rules)?;

        assert_eq!(
            board.play(4, 3, Stone::White, &rules),
            Err(Error::IllegalMove(IllegalMove::Ko))
        );

        Ok(())
    }
}
