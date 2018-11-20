use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::str::FromStr;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> Vec<NodeDesc> {
    let input: &'static str = include_str!("input");
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[derive(Debug, Clone)]
struct NodeDesc {
    name: String,
    holding: Vec<String>,
    weight: u32,
}

impl FromStr for NodeDesc {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = input.split_whitespace().map(|s| String::from(s)).collect();

        if parts.len() == 2 {
            let weight = parts[1]
                .trim_left_matches("(")
                .trim_right_matches(")")
                .parse()
                .unwrap();
            Ok(NodeDesc {
                name: parts[0].clone(),
                holding: vec![],
                weight: weight,
            })
        } else if parts.len() >= 4 {
            let holding: Vec<String> = parts[3..]
                .iter()
                .map(|s| String::from(s.trim_right_matches(",")))
                .collect();
            let weight = parts[1]
                .trim_left_matches("(")
                .trim_right_matches(")")
                .parse()
                .unwrap();
            Ok(NodeDesc {
                name: parts[0].clone(),
                holding: holding,
                weight: weight,
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

#[derive(Debug, Clone)]
struct Node {
    name: String,
    weight: u32,
    holding: Vec<Weak<Node>>,
}

impl Node {
    fn total_weight(&self) -> u32 {
        let subweight: u32 = self.subweights().iter().sum();
        self.weight + subweight
    }

    fn subweights(&self) -> Vec<u32> {
        self.holding
            .iter()
            .map(|weak| weak.upgrade().unwrap().total_weight())
            .collect()
    }

    fn is_balanced(&self) -> bool {
        if self.holding.len() == 0 {
            true
        } else {
            let subweights = self.subweights();
            let first = subweights[0];
            for &sw in &subweights[1..] {
                if sw != first {
                    return false;
                }
            }
            true
        }
    }

    fn find_unbalance(&self) -> u32 {
        assert!(self.holding.len() > 0);
        let mut map = HashMap::new();

        for child in self.holding.iter() {
            let mut entry = map.entry(child.upgrade().unwrap().total_weight())
                .or_insert(vec![]);
            entry.push(child);
        }

        let mut odd_one_out = None;
        let mut normal_weight = None;
        for (k, v) in map.iter() {
            if v.len() == 1 {
                assert!(odd_one_out.is_none());
                odd_one_out = Some(v);
            } else {
                assert!(normal_weight.is_none());
                normal_weight = Some(k);
            }
        }
        odd_one_out.unwrap()[0].upgrade().unwrap().make_equal_to(
            *normal_weight
                .unwrap(),
        )
    }

    fn make_equal_to(&self, target: u32) -> u32 {
        if self.is_balanced() {
            // this node is the one that is out of whack
            let imbalance = self.total_weight() - target;
            self.weight - imbalance
        } else {
            self.find_unbalance()
        }
    }
}

fn puzzle(input: Vec<NodeDesc>) -> u32 {
    let mut held_by = HashMap::new();

    for node in input.iter() {
        for held in node.holding.iter() {
            let mut entry = held_by.entry(held).or_insert(vec![]);
            entry.push(node.name.clone());
        }
    }

    let mut founds = vec![];
    for node in input.iter() {
        if !held_by.contains_key(&node.name) {
            founds.push(node.name.clone());
        }
    }

    if founds.len() == 0 {
        panic!("didn't find a bottom");
    } else if founds.len() > 1 {
        panic!("found {} bottoms! {:?}", founds.len(), founds);
    }

    let root_name = founds.pop().unwrap();

    // ---

    let mut named_nodes = HashMap::new();

    let mut node_descs = input.clone();

    while node_descs.len() > 0 {
        let node_desc = node_descs.pop().unwrap();
        let mut ready = true;
        for held_name in node_desc.holding.iter() {
            if !named_nodes.contains_key(held_name) {
                ready = false;
            }
        }

        if !ready {
            node_descs.insert(0, node_desc);
        } else {
            let node = Node {
                name: node_desc.name.clone(),
                weight: node_desc.weight,
                holding: node_desc
                    .holding
                    .iter()
                    .map(|held_name| {
                        Rc::downgrade(named_nodes.get(held_name).unwrap())
                    })
                    .collect(),
            };
            named_nodes.insert(node.name.clone(), Rc::new(node));
        }
    }

    let root_node = named_nodes.get(&root_name).unwrap();
    root_node.find_unbalance()
}

#[test]
fn test_example() {
    // pbga (66)
    // xhth (57)
    // ebii (61)
    // havc (66)
    // ktlj (57)
    // fwft (72) -> ktlj, cntj, xhth
    // qoyq (66)
    // padx (45) -> pbga, havc, qoyq
    // tknk (41) -> ugml, padx, fwft
    // jptl (61)
    // ugml (68) -> gyxo, ebii, jptl
    // gyxo (61)
    // cntj (57)
    //

    let input = vec![
        NodeDesc {
            name: String::from("pbga"),
            weight: 66,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("xhth"),
            weight: 57,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("ebii"),
            weight: 61,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("havc"),
            weight: 66,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("ktlj"),
            weight: 57,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("fwft"),
            weight: 72,
            holding: vec![
                String::from("ktlj"),
                String::from("cntj"),
                String::from("xhth"),
            ],
        },
        NodeDesc {
            name: String::from("qoyq"),
            weight: 66,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("padx"),
            weight: 45,
            holding: vec![
                String::from("pbga"),
                String::from("havc"),
                String::from("qoyq"),
            ],
        },
        NodeDesc {
            name: String::from("tknk"),
            weight: 41,
            holding: vec![
                String::from("ugml"),
                String::from("padx"),
                String::from("fwft"),
            ],
        },
        NodeDesc {
            name: String::from("jptl"),
            weight: 61,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("ugml"),
            weight: 68,
            holding: vec![
                String::from("gyxo"),
                String::from("ebii"),
                String::from("jptl"),
            ],
        },
        NodeDesc {
            name: String::from("gyxo"),
            weight: 61,
            holding: vec![],
        },
        NodeDesc {
            name: String::from("cntj"),
            weight: 57,
            holding: vec![],
        },
    ];
    assert_eq!(puzzle(input), 60);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 1526);
}
