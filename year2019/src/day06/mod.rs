use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use petgraph::{graph::NodeIndex, Direction, Graph};
use std::{
    collections::{HashMap, VecDeque},
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2019-D06-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", include_str!("example1"), 42)
            .case("Solution", include_str!("input"), 387_356)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let orbits = parse_input(input);
        build_orbit_graph(orbits)
            .graph
            .node_weights_mut()
            .map(|count| count.expect("disconnected element"))
            .sum()
    }
}

#[derive(Debug, Default)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D06-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", include_str!("example2"), 4)
            .case("Solution", include_str!("input"), 532)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let orbits = parse_input(input);
        let OrbitInfo { graph, name_map } = build_orbit_graph(orbits);
        let mut you_path = path_to_com(&graph, name_map["YOU"]);
        let mut san_path = path_to_com(&graph, name_map["SAN"]);

        loop {
            match (you_path.last(), san_path.last()) {
                (Some(a), Some(b)) if a == b => {
                    you_path.pop();
                    san_path.pop();
                }
                _ => break,
            }
        }

        // subtract 2 to exclude the links from SAN and YOU to their parents, as
        // indicated in puzzle
        you_path.len() + san_path.len() - 2
    }
}

fn parse_input<'a>(input: &'a str) -> Vec<(&'a str, &'a str)> {
    input.trim()
        // .trimmed_lines()
        .lines()
        .map(|line| {
            let parts = line.trim().split(')').collect::<Vec<_>>();
            if parts.len() != 2 {
                panic!(format!("could not parse line {}", line));
            }
            (parts[0].clone(), parts[1].clone())
        })
        .collect()
}

struct OrbitInfo<'a> {
    graph: Graph<Option<u32>, (), petgraph::Directed, u32>,
    name_map: HashMap<&'a str, NodeIndex>,
}

fn build_orbit_graph<'a>(orbits: Vec<(&'a str, &'a str)>) -> OrbitInfo<'a> {
    let mut graph: Graph<Option<u32>, (), petgraph::Directed, u32> = Graph::new();
    let mut com_index = None;

    let mut name_map = HashMap::new();

    for (parent, satellite) in orbits.into_iter() {
        let is_com = parent == "COM";
        let satellite_index = *name_map
            .entry(satellite)
            .or_insert_with(|| graph.add_node(None));
        let parent_index = *name_map
            .entry(parent)
            .or_insert_with(|| graph.add_node(None));
        graph.add_edge(parent_index, satellite_index, ());
        if is_com {
            com_index = Some(parent_index);
        }
    }

    let com_index = com_index.expect("No center-of-mass (COM) found in input");
    graph[com_index] = Some(0);

    let mut todo = VecDeque::new();
    todo.push_back(com_index);

    while !todo.is_empty() {
        let parent_index = todo.pop_front().unwrap();
        let parent_count = graph[parent_index].expect("disconnected element");
        let mut walker = graph.neighbors(parent_index).detach();
        while let Some((_, satellite_index)) = walker.next(&graph) {
            graph[satellite_index] = Some(parent_count + 1);
            todo.push_back(satellite_index);
        }
    }

    OrbitInfo { graph, name_map }
}

fn path_to_com(orbit_graph: &Graph<Option<u32>, ()>, starting_point: NodeIndex) -> Vec<NodeIndex> {
    let mut path = vec![starting_point];
    loop {
        let parents: Vec<_> = orbit_graph
            .neighbors_directed(*path.last().unwrap(), Direction::Incoming)
            .collect();
        match parents.len() {
            0 => break,
            1 => path.push(parents[0]),
            _ => panic!("graph is not a tree"),
        }
    }
    path
}
