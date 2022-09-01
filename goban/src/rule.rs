/// All types of moves that can be illegal.
/// typically accessed through [crate::Error].
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IllegalMove {
    /// Attempt to play on a spot on the [Board](crate::Board) that is already occupied
    NonEmptySpace,
    /// Playing a stone that is immediately dead.
    /// Only if [Rules::suicide_allowed] is true.
    SuicidalMove,
}

#[derive(Clone, Copy)]
pub struct Rules {
    pub suicide_allowed: bool,
}
impl Rules {
    pub const JAPANESE: Self = Self {
        suicide_allowed: false,
    };
}
