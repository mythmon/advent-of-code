use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::{indoc, indoc_impl};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::Iterator,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Day06Part1;

impl PuzzleRunner for Day06Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2018-D06-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n", 17usize)
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
                9usize,
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
            assert!(distances.is_empty());
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

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}

impl std::str::FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').map(|p| p.trim()).collect();
        if parts.len() != 2 {
            Err(From::from("Points must be 2d"))
        } else {
            Ok(Self {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
            })
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

impl Point {
    fn manhattan_distance<'a, T: Into<&'a Point>>(&self, other: T) -> usize {
        let other: &Point = other.into();
        self.x.difference(&other.x) + self.y.difference(&other.y)
    }
}

#[derive(Debug)]
struct Grid<T> {
    storage: Vec<T>,
    stride: usize,
}

impl<T> Grid<T> {
    fn width(&self) -> usize {
        self.stride
    }

    fn height(&self) -> usize {
        self.storage.len() / self.stride
    }

    fn iter_coordinates(&self) -> GridCoordinateIterator {
        GridCoordinateIterator {
            state: 0,
            width: self.width(),
            height: self.height(),
        }
    }

    fn iter_values(&self) -> GridValIterator<'_, T> {
        GridValIterator {
            state: 0,
            grid: &self,
        }
    }
}

impl<T: Default> Grid<T> {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut storage = Vec::with_capacity(size);
        storage.resize_default(size);
        Self {
            storage,
            stride: width,
        }
    }
}

struct GridCoordinateIterator {
    state: usize,
    width: usize,
    height: usize,
}

impl Iterator for GridCoordinateIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.state % self.width;
        let y = self.state / self.height;
        if y >= self.height {
            None
        } else {
            self.state += 1;
            Some((x, y).into())
        }
    }
}

struct GridValIterator<'a, T> {
    state: usize,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for GridValIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state >= self.grid.storage.len() {
            None
        } else {
            let rv = &self.grid.storage[self.state];
            self.state += 1;
            Some(rv)
        }
    }
}

impl<T, I> Index<I> for Grid<T>
where
    I: Into<Point>,
{
    type Output = T;

    fn index(&self, p: I) -> &T {
        let p = p.into();
        self.storage.index(p.x + p.y * self.stride)
    }
}

impl<T, I> IndexMut<I> for Grid<T>
where
    I: Into<Point>,
{
    fn index_mut(&mut self, p: I) -> &mut T {
        let p = p.into();
        self.storage.index_mut(p.x + p.y * self.stride)
    }
}

impl<T> std::fmt::Display for Grid<Option<T>>
where
    T: std::fmt::Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn symbol_for<T: std::fmt::Display>(v: &Option<T>) -> String {
            match v {
                Some(v) => format!("{}", v),
                None => ".".to_owned(),
            }
        }

        let max_width = self
            .iter_values()
            .map(|v| symbol_for(v).len())
            .max()
            .unwrap();

        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(
                    fmt,
                    "{:^width$}",
                    symbol_for(&self[(x, y)]),
                    width = max_width
                )?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

trait Difference {
    type Out = usize;
    fn difference(&self, other: &Self) -> Self::Out;
}

impl Difference for usize {
    type Out = usize;
    fn difference(&self, other: &Self) -> Self::Out {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

#[derive(Debug)]
pub struct Day06Part2;

impl PuzzleRunner for Day06Part2 {
    type Input = (&'static str, usize);
    type Output = usize;

    fn name(&self) -> String {
        "2018-D06-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                ("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n", 32usize),
                16usize,
            )
            .case("Solution", (include_str!("input"), 10_000usize), 40_376)
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
