/// All types of moves that can be illegal.
/// typically accessed through [crate::Error].
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IllegalMove {
    /// Attempt to play on a spot on the [Board](crate::Board) that is already occupied
    NonEmptySpace,
    /// Playing a stone that is immediately dead.
    /// Only if [Rules::suicide_allowed] is true.
    SuicidalMove,
    /// Violating the normal ko rule.
    Ko,
    /// Repeating a past board state.
    /// Only applicable if [Rules::superko] is true
    SuperKo,
}

#[derive(Clone, Copy)]
pub struct Rules {
    /// Allow playing a move that results in the death of your won stone
    pub suicide_allowed: bool,
    /// Allow repeated board state
    pub superko: bool,
}
impl Rules {
    pub const JAPANESE: Self = Self {
        suicide_allowed: false,
        superko: true,
    };
}
