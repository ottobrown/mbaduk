use crate::tree::{SgfProp, SgfNode, SgfTree};

use pest::Parser;
use pest_derive::Parser;

pub enum ParseError {
    Pest(pest::error::Error<Rule>),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::Pest(e)
    }
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Parser)]
#[grammar = "sgf.pest"]
struct SgfParser;

pub fn parse(input: &str) -> ParseResult<SgfTree> {
    todo!()
}
