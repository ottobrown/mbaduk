use crate::tree::{SgfProp, SgfNode, SgfTree};

pub enum ParseError {}
pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub fn parse(input: String) -> ParseResult<SgfTree> {
    todo!()
}
