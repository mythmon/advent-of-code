#![allow(clippy::match_ref_pats)]

use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::{
    cmp::{Ord, Ordering},
    collections::HashSet,
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Point3>;
    type Output = i32;

    fn name(&self) -> String {
        "2019-D12-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 5_517)
            .collect()
    }

    fn run_puzzle(planet_positions: Self::Input) -> Self::Output {
        let planets: Vec<Planet> = planet_positions
            .into_iter()
            .map(|pos| Planet {
                pos,
                vel: Point3 { x: 0, y: 0, z: 0 },
            })
            .collect();

        let mut universe = Universe {
            step_count: 0,
            planets,
        };

        for _ in 0..1000 {
            universe.step();
        }

        // calculate energy
        universe
            .planets
            .into_iter()
            .map(|planet| {
                let potential = planet.pos.x.abs() + planet.pos.y.abs() + planet.pos.z.abs();
                let kinetic = planet.vel.x.abs() + planet.vel.y.abs() + planet.vel.z.abs();
                potential * kinetic
            })
            .sum()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Point3>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D12-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 303_070_460_651_184)
            .collect()
    }

    fn run_puzzle(planet_positions: Self::Input) -> Self::Output {
        let planets_x: Vec<Planet1d> = planet_positions
            .iter()
            .map(|pos| Planet1d { pos: pos.x, vel: 0 })
            .collect();
        let planets_y: Vec<Planet1d> = planet_positions
            .iter()
            .map(|pos| Planet1d { pos: pos.y, vel: 0 })
            .collect();
        let planets_z: Vec<Planet1d> = planet_positions
            .iter()
            .map(|pos| Planet1d { pos: pos.z, vel: 0 })
            .collect();

        let mut universes = vec![
            Universe1d {
                step_count: 0,
                planets: planets_x,
            },
            Universe1d {
                step_count: 0,
                planets: planets_y,
            },
            Universe1d {
                step_count: 0,
                planets: planets_z,
            },
        ];

        let loop_points: Vec<usize> = universes
            .iter_mut()
            .map(|u| {
                let mut seen_states = HashSet::new();
                while seen_states.insert(u.get_state()) {
                    u.step();
                }
                u.step_count
            })
            .collect();

        if let &[a, b, c] = &loop_points[..] {
            let first = (a * b) / gcd(a, b);
            (first * c) / gcd(first, c)
        } else {
            panic!(
                "Wrong number of loop points, expected 3, got: {:?}",
                loop_points
            );
        }
    }
}

// TODO unify this between here and 2017-d13 and 2019-d10
const fn gcd(mut a: usize, mut b: usize) -> usize {
    // euclid's algorithm
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn parse_input(input: &str) -> Vec<Point3> {
    input
        .trimmed_lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl std::str::FromStr for Point3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .replace('<', "")
            .replace('>', "")
            .replace(' ', "")
            .split(',')
            .map(|c| c.split('=').last().unwrap().parse().unwrap())
            .collect();
        if parts.len() == 3 {
            Ok(Self {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        } else {
            Err(format!("Wrong number of parts, wanted 3 got: {:?}", &parts))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Planet {
    pos: Point3,
    vel: Point3,
}

struct Universe {
    planets: Vec<Planet>,
    step_count: usize,
}

impl Universe {
    fn step(&mut self) {
        // gravity
        for i in 0..(self.planets.len() - 1) {
            for j in (i + 1)..self.planets.len() {
                match self.planets[i].pos.x.cmp(&self.planets[j].pos.x) {
                    Ordering::Less => {
                        self.planets[i].vel.x += 1;
                        self.planets[j].vel.x -= 1;
                    }
                    Ordering::Greater => {
                        self.planets[i].vel.x -= 1;
                        self.planets[j].vel.x += 1;
                    }
                    Ordering::Equal => {}
                }

                match self.planets[i].pos.y.cmp(&self.planets[j].pos.y) {
                    Ordering::Less => {
                        self.planets[i].vel.y += 1;
                        self.planets[j].vel.y -= 1;
                    }
                    Ordering::Greater => {
                        self.planets[i].vel.y -= 1;
                        self.planets[j].vel.y += 1;
                    }
                    Ordering::Equal => {}
                }

                match self.planets[i].pos.z.cmp(&self.planets[j].pos.z) {
                    Ordering::Less => {
                        self.planets[i].vel.z += 1;
                        self.planets[j].vel.z -= 1;
                    }
                    Ordering::Greater => {
                        self.planets[i].vel.z -= 1;
                        self.planets[j].vel.z += 1;
                    }
                    Ordering::Equal => {}
                }
            }
        }

        // velocity
        for mut planet in &mut self.planets {
            planet.pos.x += planet.vel.x;
            planet.pos.y += planet.vel.y;
            planet.pos.z += planet.vel.z;
        }

        self.step_count += 1;
    }
}

struct Planet1d {
    pos: i32,
    vel: i32,
}

struct Universe1d {
    planets: Vec<Planet1d>,
    step_count: usize,
}

impl Universe1d {
    fn step(&mut self) {
        // gravity
        for i in 0..(self.planets.len() - 1) {
            for j in (i + 1)..self.planets.len() {
                match self.planets[i].pos.cmp(&self.planets[j].pos) {
                    Ordering::Less => {
                        self.planets[i].vel += 1;
                        self.planets[j].vel -= 1;
                    }
                    Ordering::Greater => {
                        self.planets[i].vel -= 1;
                        self.planets[j].vel += 1;
                    }
                    Ordering::Equal => {}
                }
            }
        }

        // velocity
        for mut planet in &mut self.planets {
            planet.pos += planet.vel;
        }

        self.step_count += 1;
    }

    fn get_state(&self) -> Vec<i32> {
        self.planets
            .iter()
            .flat_map(|p| vec![p.pos, p.vel])
            .collect()
    }
}
