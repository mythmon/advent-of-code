use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::{Grid, Point},
};
use itertools::Itertools;
use rayon::prelude::*;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = i32;
    type Output = String;

    fn name(&self) -> String {
        "2018-D11-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", 18, "33,45".to_owned())
            .case(
                "Solution",
                include_str!("input").trim().parse::<i32>().unwrap(),
                "21,41".to_owned(),
            )
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let size = 300;

        let mut grid: Grid<i32> = Grid::new(size, size);
        for idx in grid.iter_coordinates() {
            grid[idx] = power_level(idx, input);
        }

        let max_point = (0..size - 2)
            .cartesian_product(0..size - 2)
            .map(Into::into)
            .map(|p: Point| {
                let power_sum = (p.x..p.x + 3)
                    .cartesian_product(p.y..p.y + 3)
                    .map(|(xd, yd)| grid[(xd, yd)])
                    .sum();
                (power_sum, p)
            })
            .max_by_key(|(power, _coord): &(i32, Point)| *power)
            .unwrap()
            .1;

        format!("{},{}", max_point.x, max_point.y)
    }
}

const fn power_level(cell: Point, grid_serial: i32) -> i32 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id: i32 = cell.x as i32 + 10;
    // Begin with a power level of the rack ID times the Y coordinate.
    let mut power: i32 = rack_id * cell.y as i32;
    // Increase the power level by the value of the grid serial number
    power += grid_serial;
    // Set the power level to itself multiplied by the rack ID.
    power *= rack_id;
    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers
    // with no hundreds digit become 0).
    power = (power % 1000) / 100;
    // Subtract 5 from the power level.
    power - 5
}

#[cfg(test)]
mod tests {
    use super::power_level;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level((3, 5).into(), 8), 4);
        assert_eq!(power_level((122, 79).into(), 57), -5);
        assert_eq!(power_level((217, 196).into(), 39), 0);
        assert_eq!(power_level((101, 153).into(), 71), 4);
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = i32;
    type Output = String;

    fn name(&self) -> String {
        "2018-D11-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", 18, "90,269,16".to_owned())
            .case("Example", 42, "232,251,12".to_owned())
            .case(
                "Solution",
                include_str!("input").trim().parse::<i32>().unwrap(),
                "227,199,19".to_owned(),
            )
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        // TODO(performance) This really isn't fast enough. There are more
        // efficient algorithms for maximum subset of a grid, use them.
        let grid_size = 300;

        let mut grid: Grid<i32> = Grid::new(grid_size, grid_size);
        for idx in grid.iter_coordinates() {
            grid[idx] = power_level(idx, input);
        }

        let candidates: Vec<(usize, usize, usize)> = (1..=20)
            .flat_map(|square_size| {
                let axis_iter = 0..=grid_size - square_size;
                axis_iter
                    .clone()
                    .cartesian_product(axis_iter)
                    .map(move |(x, y)| (x, y, square_size))
            })
            .collect();

        let max_ident = candidates
            .par_iter()
            .map(|(x, y, size)| {
                let power_sum: i32 = (*x..x + size)
                    .cartesian_product(*y..y + size)
                    .map(|(xd, yd)| grid[(xd, yd)])
                    .sum();
                (power_sum, (x, y, size))
            })
            .max_by_key(|(power, _ident)| *power)
            .unwrap()
            .1;

        format!("{},{},{}", max_ident.0, max_ident.1, max_ident.2)
    }
}
