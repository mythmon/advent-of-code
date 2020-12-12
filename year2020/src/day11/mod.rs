use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
    twodee::Bounds,
    twodee::{Dir8, Grid, Point, VecGrid},
};
use indoc::indoc;
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::Display,
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Space {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Default for Space {
    fn default() -> Self {
        Space::Floor
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Floor => write!(f, "."),
            Space::EmptySeat => write!(f, "L"),
            Space::OccupiedSeat => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Space {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Space::Floor),
            'L' => Ok(Space::EmptySeat),
            '#' => Ok(Space::OccupiedSeat),
            _ => Err(format!("Invalid space character {}", c)),
        }
    }
}

fn parse_input(input: &str) -> Result<VecGrid<isize, Space>, Box<dyn Error>> {
    let lines: Vec<_> = input.trimmed_lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    assert!(lines.iter().all(|l| l.len() == width));

    let mut grid = VecGrid::new(Bounds::new(0, 0, width as isize, height as isize));

    for (y, line) in lines.into_iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.set(Point::new(x as isize, y as isize), char.try_into()?);
        }
    }

    Ok(grid)
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = VecGrid<isize, Space>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D11-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL
            "},
                37,
            )?
            .transformed_case("Solution", include_str!("input"), 2_296)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut current = input.clone();
        let mut next = input;

        loop {
            for p in current.iter_coords() {
                let occupied_neighbors = p
                    .neighbors8()
                    .iter()
                    .filter(|n| matches!(current.get(**n), Some(Space::OccupiedSeat)))
                    .count();
                match (current.get(p), occupied_neighbors) {
                    (Some(Space::EmptySeat), 0) => {
                        next.set(p, Space::OccupiedSeat);
                    }
                    (Some(Space::OccupiedSeat), c) if c >= 4 => {
                        next.set(p, Space::EmptySeat);
                    }
                    _ => {}
                }
            }

            if current == next {
                break;
            }

            current = next.clone();
        }

        Ok(current
            .iter_values()
            .filter(|v| **v == Space::OccupiedSeat)
            .count())
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = VecGrid<isize, Space>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D11-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL
            "},
                26,
            )?
            .transformed_case("Solution", include_str!("input"), 2_089)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut current = input.clone();
        let mut next = input;

        loop {
            for p in current.iter_coords() {
                let occupied_visible = Dir8::spin_iter()
                    .take(8)
                    .map(|dir| {
                        let mut n = p + dir;
                        while let Some(s) = current.get(n) {
                            if *s == Space::EmptySeat {
                                return 0;
                            } else if *s == Space::OccupiedSeat {
                                return 1;
                            }
                            n += dir;
                        }
                        0
                    })
                    .sum();

                match (current.get(p), occupied_visible) {
                    (Some(Space::EmptySeat), 0) => {
                        next.set(p, Space::OccupiedSeat);
                    }
                    (Some(Space::OccupiedSeat), c) if c >= 5 => {
                        next.set(p, Space::EmptySeat);
                    }
                    _ => {}
                }
            }

            if current == next {
                break;
            }

            current = next.clone();
        }

        Ok(current
            .iter_values()
            .filter(|v| **v == Space::OccupiedSeat)
            .count())
    }
}
