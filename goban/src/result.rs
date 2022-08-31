use crate::rule::IllegalMove;

/// The ways [mb_goban](crate) can fail
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    /// Attempt to interact with a point on the board that does not exist.
    CoordinatesOutOfBounds,
    /// Attempt to play a move that is illegal according to [Rules](crate::rule::Rules).
    IllegalMove(IllegalMove),
}

pub type Result<T> = std::result::Result<T, Error>;
