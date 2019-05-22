use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use lazy_static::{__lazy_static_create, __lazy_static_internal, lazy_static};
use petgraph::stable_graph::StableGraph;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    iter::Iterator,
};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = String;

    fn name(&self) -> String {
        "2018-D07-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            // spell-checker: disable
            .case("Example", include_str!("example"), "CABDFE".to_owned())
            .case("Solution", include_str!("input"), "JDEKPFABTUHOQSXVYMLZCNIGRW".to_owned())
            // spell-checker: enable
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut path: Vec<char> = Vec::new();
        let mut graph = make_graph(input);

        while graph.node_count() > 0 {
            let mut ready_nodes: Vec<_> = graph
                .node_indices()
                .filter(|n| {
                    graph
                        .edges_directed(*n, petgraph::EdgeDirection::Incoming)
                        .count()
                        == 0
                })
                .collect();
            assert!(!ready_nodes.is_empty(), "At least one node should be ready");
            ready_nodes.sort_by_key(|k| graph.node_weight(*k));
            let n = *ready_nodes.iter().next().unwrap();
            path.push(graph.node_weight(n).unwrap().clone());
            graph.remove_node(n);
        }

        path.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("")
    }
}

fn make_graph(input: &str) -> StableGraph<char, ()> {
    let edge_descriptions: Vec<EdgeDescription> =
        input.trimmed_lines().map(|l| l.parse().unwrap()).collect();
    let mut graph: StableGraph<char, ()> = StableGraph::new();
    let mut name_to_idx: HashMap<char, _> = HashMap::new();

    for e in edge_descriptions {
        let from_idx = *name_to_idx
            .entry(e.name)
            .or_insert_with(|| graph.add_node(e.name));
        let to_idx = *name_to_idx
            .entry(e.blocks)
            .or_insert_with(|| graph.add_node(e.blocks));
        graph.add_edge(from_idx, to_idx, ());
    }

    graph
}

struct EdgeDescription {
    name: char,
    blocks: char,
}

impl std::str::FromStr for EdgeDescription {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INPUT_RE: Regex =
                Regex::new(r#"Step ([A-Z]) must be finished before step ([A-Z]) can begin."#)
                    .unwrap();
        }
        let matches = INPUT_RE.captures(s).ok_or(())?;
        Ok(Self {
            name: matches[1].chars().next().unwrap(),
            blocks: matches[2].chars().next().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = (&'static str, usize, u32);
    type Output = u32;

    fn name(&self) -> String {
        "2018-D07-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", (include_str!("example"), 2_usize, 0_u32), 15_u32)
            .case(
                "Solution",
                (include_str!("input"), 6_usize, 60_u32),
                1_048_u32,
            )
            .collect()
    }

    fn run_puzzle((input, max_workers, extra_time_per): Self::Input) -> Self::Output {
        let mut total_time = 0;
        let mut graph = make_graph(input);
        let mut work_in_progress = HashMap::new();

        while graph.node_count() > 0 || !work_in_progress.is_empty() {
            let mut ready_nodes: BTreeSet<_> = graph
                .node_indices()
                .filter(|n| {
                    graph
                        .edges_directed(*n, petgraph::EdgeDirection::Incoming)
                        .count()
                        == 0
                })
                .filter(|n| !work_in_progress.contains_key(n))
                .collect();

            while work_in_progress.len() < max_workers && !ready_nodes.is_empty() {
                let idx = ready_nodes.pop().unwrap();
                let work = *graph.node_weight(idx).unwrap();
                let task_time = task_length(work, extra_time_per);
                work_in_progress.insert(idx, task_time);
            }

            work_in_progress = work_in_progress
                .into_iter()
                .map(|(idx, time)| (idx, time - 1))
                .filter(|(idx, time)| {
                    if *time == 0 {
                        graph.remove_node(*idx);
                        false
                    } else {
                        true
                    }
                })
                .collect();

            total_time += 1;
        }

        total_time
    }
}

fn task_length(task_name: char, extra_time_per: u32) -> u32 {
    assert!(task_name.is_ascii());
    1 + (task_name.to_ascii_lowercase() as u32) - ('a' as u32) + extra_time_per
}

trait Pop<T> {
    fn pop(&mut self) -> Option<T>;
}

impl<T> Pop<T> for BTreeSet<T>
where
    T: std::cmp::Ord + Clone,
{
    fn pop(&mut self) -> Option<T> {
        let popped = self.iter().next();
        if let Some(v) = popped {
            let v = v.clone(); // ends the borrow of `self` from above
            self.take(&v)
        } else {
            None
        }
    }
}
