use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    twodee::Point,
};
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Map;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D10-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Debug", "###\n#..\n...", 3)?
            .transformed_case("Example 0", include_str!("example0"), 8)?
            .transformed_case("Example 1", include_str!("example1"), 33)?
            .transformed_case("Example 2", include_str!("example2"), 35)?
            .transformed_case("Example 3", include_str!("example3"), 41)?
            .transformed_case("Example 4", include_str!("example4"), 210)?
            .transformed_case("Solution", include_str!("input"), 260)?
            .collect())
    }

    fn run_puzzle(map: Self::Input) -> Self::Output {
        map.best_detector().1
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Map;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D10-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Example 4", include_str!("example4"), 802)?
            .transformed_case("Solution", include_str!("input"), 608)?
            .collect())
    }

    fn run_puzzle(mut map: Self::Input) -> Self::Output {
        let target = 200;
        let (base, _) = map.best_detector();
        let mut num_vaporized = 0;
        let mut last_shell = None;
        while num_vaporized < target {
            let vaporization_shell = map.visible_from(base);
            num_vaporized += vaporization_shell.len();
            for a in &vaporization_shell {
                map.set(*a, false);
            }
            last_shell = Some(vaporization_shell);
        }
        let excess = num_vaporized - target;
        let mut last_shell = last_shell.unwrap();
        last_shell.sort_by_key(|target| {
            (f32::atan2(
                target.x as f32 - base.x as f32,
                target.y as f32 - base.y as f32,
            ) * 1000_f32) as i32
        });
        let target_asteroid = last_shell[excess];
        target_asteroid.x * 100 + target_asteroid.y
    }
}

// TODO unify this between here and 2017-d13
const fn gcd(mut a: u32, mut b: u32) -> u32 {
    // euclid's algorithm
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Clone, Debug)]
pub struct Map {
    space: Vec<Vec<bool>>,
}

impl Map {
    fn get(&self, Point { x, y }: Point<usize>) -> bool {
        self.space[y][x]
    }

    fn set(&mut self, Point { x, y }: Point<usize>, v: bool) {
        self.space[y][x] = v;
    }

    fn width(&self) -> usize {
        self.space.len()
    }

    fn height(&self) -> usize {
        self.space[0].len()
    }

    fn asteroid_locations(&self) -> Vec<Point<usize>> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| Point::new(x, y)))
            .filter(|pos| self.get(*pos))
            .collect()
    }

    fn visible_from(&self, base: Point<usize>) -> Vec<Point<usize>> {
        self.asteroid_locations()
            .into_iter()
            .filter(|target| {
                if base == *target {
                    return false;
                }
                let target_i32: Point<i32> = (*target).into();
                let base_i32: Point<i32> = base.into();
                let delta = target_i32 - base_i32;
                let reduced = gcd(delta.x.abs() as u32, delta.y.abs() as u32) as i32;

                if reduced == 1 {
                    // trivially visible
                    return true;
                }

                let step = delta / reduced;
                (1..reduced).all(|i| !self.get((base_i32 + step * i).into()))
            })
            .collect()
    }

    fn best_detector(&self) -> (Point<usize>, usize) {
        self.asteroid_locations()
            .into_iter()
            .map(|base| (base, self.visible_from(base).len()))
            .max_by_key(|(_, num_visible)| *num_visible)
            .unwrap()
    }
}

impl std::str::FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("bad character: {}", c),
                    })
                    .collect()
            })
            .collect();
        Ok(Map { space })
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.space {
            writeln!(
                fmt,
                "{}",
                row.iter()
                    .map(|a| if *a { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
    }
}
