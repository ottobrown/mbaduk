use std::fmt;

// TODO: optimize size of SgfProp, SgfNode, and SgfTree

pub struct SgfProp {
    pub(crate) id: String,
    pub(crate) values: Vec<String>,
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

pub struct SgfNode {
    pub(crate) props: Vec<SgfProp>,
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

pub struct SgfTree {
    pub(crate) nodes: Vec<SgfNode>,
    pub(crate) children: Vec<SgfTree>,
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

    // TODO: test for format tree
}
