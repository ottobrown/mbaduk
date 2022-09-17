use crate::tree::{SgfNode, SgfProp, SgfTree};

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Clone, Debug)]
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
    let tree_rule = SgfParser::parse(Rule::tree, input)?.next().unwrap();

    parse_tree(tree_rule)
}

fn parse_tree(r: Pair<'_, Rule>) -> ParseResult<SgfTree> {
    let mut tree = SgfTree::default();

    for i in r.into_inner() {
        match i.as_rule() {
            Rule::tree => tree.children.push(parse_tree(i)?),
            Rule::node => tree.nodes.push(parse_node(i)?),

            _ => unreachable!(),
        }
    }

    Ok(tree)
}

fn parse_node(n: Pair<'_, Rule>) -> ParseResult<SgfNode> {
    let mut node = SgfNode::default();

    for i in n.into_inner() {
        match i.as_rule() {
            Rule::prop => node.props.push(parse_prop(i)?),

            _ => unreachable!(),
        }
    }

    Ok(node)
}

fn parse_prop(p: Pair<'_, Rule>) -> ParseResult<SgfProp> {
    let mut prop = SgfProp::default();

    for i in p.into_inner() {
        match i.as_rule() {
            Rule::prop_id => prop.id = String::from(i.as_str()),
            Rule::prop_value => prop.values.push(String::from(i.as_str())),

            _ => unreachable!(),
        }
    }

    Ok(prop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_comment() {
        let comment = "(;GC[This can contain spaces])";

        let tree = SgfTree {
            nodes: vec![SgfNode {
                props: vec![SgfProp::new("GC", "This can contain spaces")],
            }],

            children: Vec::new(),
        };

        assert_eq!(parse(comment).unwrap(), tree);
    }

    #[test]
    fn parse_tree() {
        let sgf_data = "(;AB[cd][ef]AW[aa][bb];B[qq](;W[aq])(;W[bq]))";
        let sgf_data_with_spaces = "(
            ;AB[cd][ef] AW[aa][bb]
            ;B[qq]
            (;W[aq])
            (;W[bq])
        )";

        let tree = SgfTree {
            nodes: vec![
                SgfNode {
                    props: vec![
                        SgfProp::new_many("AB", vec!["cd", "ef"]),
                        SgfProp::new_many("AW", vec!["aa", "bb"]),
                    ],
                },
                SgfNode {
                    props: vec![SgfProp::new("B", "qq")],
                },
            ],

            children: vec![
                SgfTree {
                    nodes: vec![SgfNode {
                        props: vec![SgfProp::new("W", "aq")],
                    }],
                    children: Vec::new(),
                },
                SgfTree {
                    nodes: vec![SgfNode {
                        props: vec![SgfProp::new("W", "bq")],
                    }],
                    children: Vec::new(),
                },
            ],
        };

        assert_eq!(parse(sgf_data).unwrap(), tree);
        assert_eq!(parse(sgf_data_with_spaces).unwrap(), tree);
    }
}
