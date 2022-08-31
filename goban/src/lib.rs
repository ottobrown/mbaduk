#![allow(clippy::needless_return)]

pub mod result;
pub mod rule;
pub mod board;

pub use result::{Error, Result};
pub use board::{Stone, Board, Group};
