use std::fmt;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfProp {
    pub id: String,
    pub values: Vec<String>,
}
impl SgfProp {
    pub fn new(id: &str, value: &str) -> Self {
        Self {
            id: id.into(),
            values: vec![value.into()],
        }
    }

    pub fn new_many(id: &str, values: Vec<&str>) -> Self {
        Self {
            id: id.into(),
            values: values.iter().map(|&s| s.into()).collect(),
        }
    }
}

impl fmt::Display for SgfProp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)?;

        for v in &self.values {
            write!(f, "[{}]", v)?;
        }

        Ok(())
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfNode {
    pub props: Vec<SgfProp>,
}
impl SgfNode {
    pub fn new(props: impl Into<Vec<SgfProp>>) -> Self {
        Self {
            props: props.into(),
        }
    }
}

impl fmt::Display for SgfNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ";")?;

        for p in &self.props {
            write!(f, "{p}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfTree {
    pub nodes: Vec<SgfNode>,
    pub children: Vec<SgfTree>,
}
impl SgfTree {
    pub fn new(nodes: impl Into<Vec<SgfNode>>, children: impl Into<Vec<SgfTree>>) -> Self {
        Self {
            nodes: nodes.into(),
            children: children.into(),
        }
    }
}

impl fmt::Display for SgfTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for n in &self.nodes {
            write!(f, "{n}")?;
        }

        for c in &self.children {
            write!(f, "{c}")?;
        }

        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_prop() {
        let prop = SgfProp {
            id: String::from("AB"),
            values: vec![String::from("cd"), String::from("ef")],
        };

        assert_eq!(format!("{}", prop), String::from("AB[cd][ef]"));
    }

    #[test]
    fn format_node() {
        let node = SgfNode {
            props: vec![
                SgfProp {
                    id: String::from("AB"),
                    values: vec![String::from("cd"), String::from("ef")],
                },
                SgfProp {
                    id: String::from("AW"),
                    values: vec![String::from("aa"), String::from("bb")],
                },
            ],
        };

        assert_eq!(format!("{}", node), String::from(";AB[cd][ef]AW[aa][bb]"));
    }

    #[test]
    fn format_tree() {
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

        assert_eq!(
            format!("{tree}"),
            String::from("(;AB[cd][ef]AW[aa][bb];B[qq](;W[aq])(;W[bq]))")
        )
    }
}
