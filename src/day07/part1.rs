use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> Vec<NodeDesc> {
    let input: &'static str = include_str!("input");
    input.lines().map(|l| l.parse().unwrap()).collect()
}

struct NodeDesc {
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

fn puzzle(input: Vec<NodeDesc>) -> String {
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

#[test]
fn test_example() {
    let input = vec![
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
    ];
    assert_eq!(puzzle(input), String::from("tknk"));
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), "gynfwly");
}
