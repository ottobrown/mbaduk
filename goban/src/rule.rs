#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IllegalMove {
    /// Attempt to play on a spot on the [Board](crate::Board) that is already occupied
    NonEmptySpace,
}

#[derive(Clone, Copy)]
pub struct Rules {}
