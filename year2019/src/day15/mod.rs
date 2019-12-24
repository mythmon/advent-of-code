use crate::intcode::{IntcodeComputer, PauseReason};
use advent_lib::{
    cases::{ExpectedValue, GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    twodee::{Dir, Grid, HashGrid, Point},
};
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fmt,
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(Part1),
        // Box::new(Part2),
    ]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<isize>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D15-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 272)
            .collect()
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        let mut grid = HashGrid::<Area>::default();
        grid.set(Point::zero(), Area::Origin);

        let walkable_tiles = {
            let mut rv = HashSet::new();
            rv.insert(Area::Floor);
            rv.insert(Area::OxygenSystem);
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

            let directions = match position.direction_to(goal) {
                Some(dir) => vec![dir],
                None => {
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
                                from.direction_to(to).unwrap_or_else(|| {
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
                }
            };

            for dir in directions {
                robot.add_input(match dir {
                    Dir::Up => 1,
                    Dir::Down => 2,
                    Dir::Left => 3,
                    Dir::Right => 4,
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

                        println!(
                            "{}",
                            grid.with_patch(position, Area::Robot)
                                .with_patch(goal, Area::Goal)
                        );

                        return grid
                            .astar(Point::zero(), position, &walkable_tiles)
                            .expect(&format!("no route found from origin to {}", position))
                            .len()
                            - 1;
                    }
                    _ => panic!("Invalid robot activity"),
                }
            }
        }

        panic!("No solution found");
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

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
