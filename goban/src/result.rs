use crate::rule::IllegalMove;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    CoordinatesOutOfBounds,
    IllegalMove(IllegalMove),
    PointAlreadyCategorized,
}

pub type Result<T> = std::result::Result<T, Error>;
