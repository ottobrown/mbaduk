#![allow(clippy::needless_return)]

mod board;
mod result;
mod rule;

pub use board::{Board, Group, Stone};
pub use result::{Error, Result};
pub use rule::{Rules, IllegalMove};
