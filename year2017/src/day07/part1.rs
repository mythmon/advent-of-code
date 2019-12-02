use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<NodeDesc>;
    type Output = String;

    fn name(&self) -> String {
        "2017-D07-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        // spell-checker: disable
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.lines().map(|l| l.parse().unwrap()).collect())
            .case(
                "Example",
                vec![
                    NodeDesc::new("xhth", &[]),
                    NodeDesc::new("cntj", &[]),
                    NodeDesc::new("ktlj", &[]),
                    NodeDesc::new("goyq", &[]),
                    NodeDesc::new("havc", &[]),
                    NodeDesc::new("pbga", &[]),
                    NodeDesc::new("jptl", &[]),
                    NodeDesc::new("ebii", &[]),
                    NodeDesc::new("gyxo", &[]),
                    NodeDesc::new("tknk", &["ugml", "padx", "fwft"]),
                    NodeDesc::new("fwft", &["ktlj", "cntj", "xhth"]),
                    NodeDesc::new("padx", &["pbga", "havc", "goyq"]),
                    NodeDesc::new("ugml", &["gyxo", "ebii", "jptl"]),
                ],
                "tknk".to_owned(),
            )
            .transformed_case("Solution", include_str!("input"), "gynfwly".to_owned())
            .collect()
        // spell-checker: enable
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut blocked_by = HashMap::new();

        for node in &input {
            for blocked in &node.blocks {
                let entry = blocked_by.entry(blocked).or_insert_with(Vec::new);
                entry.push(node.name.clone());
            }
        }

        let mut founds = vec![];
        for node in &input {
            if !blocked_by.contains_key(&node.name) {
                founds.push(node.name.clone());
            }
        }

        if founds.is_empty() {
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
    fn new(name: &str, blocks: &[&str]) -> Self {
        Self {
            name: String::from(name),
            blocks: blocks.iter().map(|s| String::from(*s)).collect(),
        }
    }
}

impl FromStr for NodeDesc {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();

        if parts.len() == 2 {
            Ok(Self {
                name: parts[0].clone(),
                blocks: vec![],
            })
        } else if parts.len() >= 4 {
            let blocks: Vec<String> = parts[3..]
                .iter()
                .map(|s| String::from(s.trim_end_matches(',')))
                .collect();
            Ok(Self {
                name: parts[0].clone(),
                blocks,
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
