use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use indoc::indoc;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<NodeDesc>;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D07-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        // spell-checker: disable
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(|s| s.lines().map(|l| l.parse()).collect())
            .transformed_case(
                "Example",
                indoc!(
                    "
                    pbga (66)
                    xhth (57)
                    ebii (61)
                    havc (66)
                    ktlj (57)
                    fwft (72) -> ktlj, cntj, xhth
                    qoyq (66)
                    padx (45) -> pbga, havc, qoyq
                    tknk (41) -> ugml, padx, fwft
                    jptl (61)
                    ugml (68) -> gyxo, ebii, jptl
                    gyxo (61)
                    cntj (57)
                    "
                ),
                60,
            )?
            .transformed_case("Solution", include_str!("input"), 1_526)?
            .collect())
        // spell-checker: enable
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut held_by = HashMap::new();

        for node in &input {
            for held in &node.holding {
                let entry = held_by.entry(held).or_insert_with(Vec::new);
                entry.push(node.name.clone());
            }
        }

        let mut founds = vec![];
        for node in &input {
            if !held_by.contains_key(&node.name) {
                founds.push(node.name.clone());
            }
        }

        if founds.is_empty() {
            panic!("didn't find a bottom");
        } else if founds.len() > 1 {
            panic!("found {} bottoms! {:?}", founds.len(), founds);
        }

        let root_name = founds.pop().unwrap();

        // ---

        let mut named_nodes = HashMap::new();

        let mut node_descriptions = input;

        while !node_descriptions.is_empty() {
            let node_desc = node_descriptions.pop().unwrap();
            let mut ready = true;
            for held_name in &node_desc.holding {
                if !named_nodes.contains_key(held_name) {
                    ready = false;
                }
            }

            if ready {
                let node = Node {
                    name: node_desc.name.clone(),
                    weight: node_desc.weight,
                    holding: node_desc
                        .holding
                        .iter()
                        .map(|held_name| Rc::downgrade(&named_nodes[held_name]))
                        .collect(),
                };
                named_nodes.insert(node.name.clone(), Rc::new(node));
            } else {
                node_descriptions.insert(0, node_desc);
            }
        }

        let root_node: &Rc<Node> = &named_nodes[&root_name];
        root_node.find_unbalance()
    }
}

#[derive(Debug, Clone)]
pub struct NodeDesc {
    name: String,
    holding: Vec<String>,
    weight: u32,
}

impl FromStr for NodeDesc {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = input.split_whitespace().map(String::from).collect();

        if parts.len() == 2 {
            let weight = parts[1]
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse()?;
            Ok(Self {
                name: parts[0].clone(),
                holding: vec![],
                weight,
            })
        } else if parts.len() >= 4 {
            let holding: Vec<String> = parts[3..]
                .iter()
                .map(|s| String::from(s.trim_end_matches(',')))
                .collect();
            let weight = parts[1]
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse()?;
            Ok(Self {
                name: parts[0].clone(),
                holding,
                weight,
            })
        } else {
            Err(format!(
                "Unexpected number of parts in {}: {}",
                input,
                parts.len()
            ).into())
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
        if self.holding.is_empty() {
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
        assert!(!self.holding.is_empty());
        let mut map = HashMap::new();

        for child in &self.holding {
            let entry = map
                .entry(child.upgrade().unwrap().total_weight())
                .or_insert_with(Vec::new);
            entry.push(child);
        }

        let mut odd_one_out = None;
        let mut normal_weight = None;
        for (k, v) in &map {
            if v.len() == 1 {
                assert!(odd_one_out.is_none());
                odd_one_out = Some(v);
            } else {
                assert!(normal_weight.is_none());
                normal_weight = Some(k);
            }
        }
        odd_one_out.unwrap()[0]
            .upgrade()
            .unwrap()
            .make_equal_to(*normal_weight.unwrap())
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
