use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Day07Part1;

impl PuzzleRunner for Day07Part1 {
    type Input = Vec<NodeDesc>;
    type Output = String;

    fn name(&self) -> String {
        "2017-D07-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.lines().map(|l| l.parse().unwrap()).collect())
            .case(
                "Example",
                vec![
                    NodeDesc::new("xhth", vec![]),
                    NodeDesc::new("cntj", vec![]),
                    NodeDesc::new("ktlj", vec![]),
                    NodeDesc::new("goyq", vec![]),
                    NodeDesc::new("havc", vec![]),
                    NodeDesc::new("pbga", vec![]),
                    NodeDesc::new("jptl", vec![]),
                    NodeDesc::new("ebii", vec![]),
                    NodeDesc::new("gyxo", vec![]),
                    NodeDesc::new("tknk", vec!["ugml", "padx", "fwft"]),
                    NodeDesc::new("fwft", vec!["ktlj", "cntj", "xhth"]),
                    NodeDesc::new("padx", vec!["pbga", "havc", "goyq"]),
                    NodeDesc::new("ugml", vec!["gyxo", "ebii", "jptl"]),
                ],
                "tknk".to_owned(),
            )
            .transformed_case("Solution", include_str!("input"), "gynfwly".to_owned())
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut blocked_by = HashMap::new();

        for node in input.iter() {
            for blocked in node.blocks.iter() {
                let entry = blocked_by.entry(blocked).or_insert(vec![]);
                entry.push(node.name.clone());
            }
        }

        let mut founds = vec![];
        for node in input.iter() {
            if !blocked_by.contains_key(&node.name) {
                founds.push(node.name.clone());
            }
        }

        if founds.len() == 0 {
            panic!("didn't find a bottom");
        } else if founds.len() > 1 {
            panic!("found {} bottoms! {:?}", founds.len(), founds);
        }

        founds.pop().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct NodeDesc {
    name: String,
    blocks: Vec<String>,
}

impl NodeDesc {
    #[allow(dead_code)]
    fn new(name: &str, blocks: Vec<&str>) -> Self {
        NodeDesc {
            name: String::from(name),
            blocks: blocks.iter().map(|s| String::from(*s)).collect(),
        }
    }
}

impl FromStr for NodeDesc {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = input.split_whitespace().map(|s| String::from(s)).collect();

        if parts.len() == 2 {
            Ok(NodeDesc {
                name: parts[0].clone(),
                blocks: vec![],
            })
        } else if parts.len() >= 4 {
            let blocks: Vec<String> = parts[3..]
                .iter()
                .map(|s| String::from(s.trim_right_matches(",")))
                .collect();
            Ok(NodeDesc {
                name: parts[0].clone(),
                blocks: blocks,
            })
        } else {
            panic!(format!(
                "Unexpected number of parts in {}: {}",
                input,
                parts.len()
            ));
        }
    }
}
