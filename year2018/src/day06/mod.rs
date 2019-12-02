use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::{Grid, Point, StringAdventExt},
};
use indoc::indoc;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2018-D06-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n", 17_usize)
            .case("Solution", include_str!("input"), 4215)
            .case(
                "Test from Reddit 1",
                indoc!(
                    "
                    1, 1
                    1, 101
                    48, 51
                    51, 48
                    51, 51
                    51, 54
                    54, 51
                    101, 1
                    101, 101
                "
                ),
                9_usize,
            )
            .case(
                "Test from Reddit 2",
                indoc!(
                    "
                    0, 0
                    0, 100
                    1, 50
                    80, 20
                    80, 50
                    80, 80
                    100, 0
                    100, 50
                    100, 100
                "
                ),
                1876,
            )
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let points: Vec<Point> = input.trimmed_lines().map(|l| l.parse().unwrap()).collect();

        let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), next| {
            (cmp::max(max_x, next.x), cmp::max(max_y, next.y))
        });

        // Voronoi on all points, ignoring ties
        let mut grid: Grid<Option<usize>> = Grid::new(max_x + 1, max_y + 1);
        assert_eq!(grid[(0, 0)], None);
        for point in grid.iter_coordinates() {
            let mut distances: Vec<_> = points
                .iter()
                .map(|p| p.manhattan_distance(&point))
                .enumerate()
                .collect::<Vec<_>>();
            distances.sort_by_key(|(_idx, distance)| *distance);
            // ignore ties
            assert!(!distances.is_empty());
            let closest_id = if distances.len() == 1 || distances[0].1 != distances[1].1 {
                Some(distances[0].0)
            } else {
                None
            };
            grid[point] = closest_id;
        }

        // find ids on the edges, which are infinite
        let mut infinite_groups: HashSet<usize> = HashSet::new();
        for x in 0..grid.width() {
            if let Some(id) = grid[(x, 0)] {
                infinite_groups.insert(id);
            }
            if let Some(id) = grid[(x, grid.height() - 1)] {
                infinite_groups.insert(id);
            }
        }
        for y in 0..grid.height() {
            if let Some(id) = grid[(0, y)] {
                infinite_groups.insert(id);
            }
            if let Some(id) = grid[(grid.width() - 1, y)] {
                infinite_groups.insert(id);
            }
        }

        // get counts
        let counts = grid.iter_values()
            .filter_map(|p| *p)  // remove Nones
            .fold(HashMap::new(), |mut acc, point_id| {
                *acc.entry(point_id).or_insert(0) += 1;
                acc
            });

        // highest count that is not in the infinite set
        let biggest_pair = counts
            .iter()
            .filter(|(id, _count)| !infinite_groups.contains(*id))
            .max_by_key(|(_id, count)| *count)
            .unwrap();
        *biggest_pair.1
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = (&'static str, usize);
    type Output = usize;

    fn name(&self) -> String {
        "2018-D06-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                ("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n", 32_usize),
                16_usize,
            )
            .case("Solution", (include_str!("input"), 10_000_usize), 40_376)
            .collect()
    }

    fn run_puzzle((input, max): Self::Input) -> Self::Output {
        let points: Vec<Point> = input.trimmed_lines().map(|l| l.parse().unwrap()).collect();

        let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), next| {
            (cmp::max(max_x, next.x), cmp::max(max_y, next.y))
        });

        // For every grid coordinate, find the sum of the manhattan distances to all
        // given points.
        let mut grid: Grid<usize> = Grid::new(max_x + 1, max_y + 1);
        for coordinate in grid.iter_coordinates() {
            grid[coordinate] = points
                .iter()
                .map(|p| p.manhattan_distance(&coordinate))
                .sum();
        }
        grid.iter_values().filter(|v| **v < max).count()
    }
}
