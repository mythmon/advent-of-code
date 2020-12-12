use crate::intcode::{IntcodeComputer, PauseReason};
use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    twodee::{Dir4, Grid, HashGrid, Point},
};
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fmt,
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

// TODO this uses A* with a lot of repetition. It would probably be better
// served by accumulating a graph of distances from A-B for all A and B. Maybe
// dijkstra's, or something fancier?

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<isize>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D15-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 272)?
            .collect())
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        let grid = explore_grid(program).expect("Problem exploring grid");
        let walkable_tiles = {
            let mut rv = HashSet::new();
            rv.insert(Area::Floor);
            rv.insert(Area::OxygenSystem);
            rv.insert(Area::Robot);
            rv.insert(Area::Goal);
            rv
        };
        let (o2_position, _) = grid
            .cells
            .iter()
            .find(|(_, area)| **area == Area::OxygenSystem)
            .expect("No answer found");

        grid.astar(Point::zero(), *o2_position, &walkable_tiles)
            .expect("no path found")
            .len()
            - 1
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D15-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 398)?
            .collect())
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        let grid = explore_grid(program).expect("Problem exploring grid");
        let walkable_tiles = {
            let mut rv = HashSet::new();
            rv.insert(Area::Floor);
            rv.insert(Area::OxygenSystem);
            rv.insert(Area::Robot);
            rv.insert(Area::Goal);
            rv
        };
        let (o2_position, _) = grid
            .cells
            .iter()
            .find(|(_, area)| **area == Area::OxygenSystem)
            .expect("No answer found");

        grid.cells
            .iter()
            .filter(|(_p, c)| walkable_tiles.contains(c))
            .map(|(p, _)| {
                grid.astar(*o2_position, *p, &walkable_tiles)
                    .expect("no path")
                    .len()
                    - 1
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Area {
    Origin,
    Wall,
    Floor,
    OxygenSystem,
    Robot,
    Goal,
}

impl Default for Area {
    fn default() -> Self {
        Area::Floor
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Wall => write!(f, "#")?,
            Self::Floor => write!(f, ".")?,
            Self::OxygenSystem => write!(f, "!")?,
            Self::Origin => write!(f, "o")?,
            Self::Goal => write!(f, "X")?,
            Self::Robot => write!(f, "%")?,
        }
        Ok(())
    }
}

fn explore_grid(program: Vec<isize>) -> Result<HashGrid<Area>, String> {
    let mut grid = HashGrid::<Area>::default();
    grid.set(Point::zero(), Area::Origin);

    let walkable_tiles = {
        let mut rv = HashSet::new();
        rv.insert(Area::Floor);
        rv.insert(Area::OxygenSystem);
        rv.insert(Area::Robot);
        rv.insert(Area::Goal);
        rv
    };
    let mut robot = IntcodeComputer::build(program).done();

    let mut position: Point<isize> = Point::zero();
    let mut todo: VecDeque<Point<isize>> = VecDeque::new();
    let mut seen: HashSet<Point<isize>> = HashSet::new();

    for neighbor in &Point::zero().neighbors4() {
        todo.push_back(*neighbor);
    }

    'next_goal: while !todo.is_empty() {
        let goal = todo.pop_front().unwrap();
        if !seen.insert(goal) {
            // skipping duplicate
            continue;
        }
        if let Some(Area::Wall) = grid.get(goal) {
            // Don't bother trying to route into walls
            continue;
        }

        let directions = if let Some(dir) = position.direction4_to(goal) {
            vec![dir]
        } else {
            // assume the target is an empty floor for path finding purposes
            let should_reset = if grid.get(goal).is_none() {
                grid.set(goal, Area::Floor);
                true
            } else {
                false
            };

            let route = grid.astar(position, goal, &walkable_tiles);

            if should_reset {
                grid.remove(&goal);
            }

            if let Some(route) = route {
                route
                    .iter()
                    .tuple_windows()
                    .map(|(&from, &to)| {
                        from.direction4_to(to).unwrap_or_else(|| {
                            panic!(
                                "Invalid astar path, {:?} -> {:?} is not a 4-direction",
                                from, to
                            )
                        })
                    })
                    .collect()
            } else {
                continue 'next_goal;
            }
        };

        for dir in directions {
            robot.add_input(match dir {
                Dir4::Up => 1,
                Dir4::Down => 2,
                Dir4::Left => 3,
                Dir4::Right => 4,
            });
            match robot.run_until_io() {
                PauseReason::Output(0) => {
                    // Hit a wall
                    grid.set(position + dir, Area::Wall);
                }
                PauseReason::Output(1) => {
                    // moved and found floor
                    position += dir;
                    grid.entry(position).or_insert(Area::Floor);
                    todo.extend(position.neighbors4().iter().filter(|n| !seen.contains(n)));
                }
                PauseReason::Output(2) => {
                    // moved and also found the oxygen system
                    position += dir;
                    grid.entry(position).or_insert(Area::OxygenSystem);
                }
                _ => return Err("Invalid robot activity".into()),
            }
        }
    }

    Ok(grid)
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
