#![allow(clippy::needless_return)]

pub mod board;
pub mod result;
pub mod rule;

pub use board::{Board, Group, Stone};
pub use result::{Error, Result};
