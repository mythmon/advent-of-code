use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use itertools::Itertools;
use lalrpop_util::lalrpop_mod;
use std::{cmp, iter::Iterator, str::FromStr};

#[cfg(windows)]
lalrpop_mod!(
    #[allow(clippy::all)]
    parser,
    "\\day10\\parser.rs"
);
#[cfg(unix)]
lalrpop_mod!(
    #[allow(clippy::all)]
    parser,
    "/day10/parser.rs"
);

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

// TODO(challenge) How can this return a meaningful answer instead of printing
// the output?

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = ();

    fn name(&self) -> String {
        "2018-D10-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", include_str!("example"), ())
            .case("Solution", include_str!("input"), ())
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let particles: Vec<Particle> = input.trimmed_lines().map(|l| l.parse().unwrap()).collect();
        let mut system = ParticleSystem { particles };
        let mut min_score = system.score();

        let max_iterations = 100_000;
        for _ in 0..max_iterations {
            system.step();
            let new_score = system.score();
            if new_score < min_score {
                min_score = new_score;
            } else {
                system.step_reverse();
                println!("{}", system);
                return;
            }
        }

        panic!(format!(
            "Did not find a likely solution after {} iterations",
            max_iterations
        ));
    }
}

struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn step(&mut self) {
        for mut particle in &mut self.particles {
            particle.position.0 += particle.velocity.0;
            particle.position.1 += particle.velocity.1;
        }
    }

    fn step_reverse(&mut self) {
        for mut particle in &mut self.particles {
            particle.position.0 -= particle.velocity.0;
            particle.position.1 -= particle.velocity.1;
        }
    }

    /// calculate bounding box of all particles
    fn bounds(&self) -> Bounds {
        let (min_x, max_x, min_y, max_y) = self.particles.iter().fold(
            (
                i32::max_value(),
                i32::min_value(),
                i32::max_value(),
                i32::min_value(),
            ),
            |(min_x, max_x, min_y, max_y), p| {
                (
                    cmp::min(min_x, p.position.0),
                    cmp::max(max_x, p.position.0),
                    cmp::min(min_y, p.position.1),
                    cmp::max(max_y, p.position.1),
                )
            },
        );
        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    // Calculate a score for this arrangement of particles. Low score are more
    // likely to be the solution
    fn score(&self) -> u64 {
        // Use the area of the bounding box as the score
        let Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        } = self.bounds();
        ((max_x - min_x).abs() as u64) * ((max_y - min_y).abs() as u64)
    }
}

impl std::fmt::Display for ParticleSystem {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bounds = self.bounds();
        let width = (bounds.max_x - bounds.min_x + 1) as usize;
        let height = (bounds.max_y - bounds.min_y + 1) as usize;

        let mut canvas: Vec<Vec<char>> = Vec::new();
        canvas.resize_with(height, || {
            let mut row = Vec::new();
            row.resize(width, ' ');
            row
        });

        for Particle {
            position: (px, py), ..
        } in &self.particles
        {
            let x = (px - bounds.min_x) as usize;
            let y = (py - bounds.min_y) as usize;
            canvas[y][x] = '#';
        }

        let output: String = canvas
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        writeln!(fmt, "{}", output)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Particle {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Particle {
    const fn new(position: (i32, i32), velocity: (i32, i32)) -> Self {
        Self { position, velocity }
    }
}

impl FromStr for Particle {
    // TODO better error handling
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::ParticleParser::new()
            .parse(s)
            .map_err(|e| format!("Error parsing {}: {}", s, e))
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2018-D10-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", include_str!("example"), 3_u32)
            .case("Solution", include_str!("input"), 10_304_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let particles: Vec<Particle> = input.trimmed_lines().map(|l| l.parse().unwrap()).collect();
        let mut system = ParticleSystem { particles };
        let mut min_score = system.score();

        let max_iterations = 100_000;
        for iteration in 0..max_iterations {
            system.step();
            let new_score = system.score();
            if new_score < min_score {
                min_score = new_score;
            } else {
                return iteration;
            }
        }

        panic!(format!(
            "Did not find a likely solution after {} iterations",
            max_iterations
        ));
    }
}
