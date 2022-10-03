#![allow(clippy::needless_return)]

mod parse;
mod tree;
pub mod util;

pub use parse::{parse, ParseError, ParseResult};
pub use tree::{SgfNode, SgfProp, SgfTree};
