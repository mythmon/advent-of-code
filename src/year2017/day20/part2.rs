use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug)]
pub struct Day20Part2;

impl PuzzleRunner for Day20Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D20-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\np=<-4,0,0>, v=< 2,0,0>, a=< \
                 0,0,0>\np=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\np=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>",
                1_usize,
            )
            .case("Solution", include_str!("input"), 504_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut particles: Vec<Particle> = input.lines().map(|l| l.parse().unwrap()).collect();

        let escape_distance = 1_000_000;
        let mut num_escaped = 0;
        while particles.len() > 1 {
            let mut positions: HashMap<Vec3, Vec<Particle>> = HashMap::new();
            for p in &particles {
                let entry = positions.entry(p.p).or_insert_with(Vec::new);
                entry.push(*p);
            }
            let collided: HashSet<Particle> = positions
                .values()
                .filter(|entries| entries.len() > 1)
                .flat_map(|es| es.clone())
                .collect();
            particles.retain(|p| !collided.contains(p));

            for p in &mut particles {
                p.tick();
            }

            let (keep, escaped) = particles
                .into_iter()
                .partition(|p| p.p.manhattan() < escape_distance);
            particles = keep;
            num_escaped += escaped.len();
        }

        num_escaped + particles.len()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

impl Particle {
    fn tick(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

impl FromStr for Particle {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // The groups after the comma split look like this
        //    p=<-4897,3080,2133>, v=<-58,-15,-78>, a=<17,-7,0>
        //    -0------|-1--|-2---|-3-----|-4-|-5--|-6----|7-|8-
        let parts: Vec<&str> = input.split(',').collect();
        assert_eq!(parts.len(), 9);

        Ok(Self {
            p: (
                parts[0][3..].trim().parse()?,
                parts[1].trim().parse()?,
                parts[2][..(parts[2].len() - 1)].trim().parse()?,
            )
                .into(),
            v: (
                parts[3][4..].trim().parse()?,
                parts[4].trim().parse()?,
                parts[5][..(parts[5].len() - 1)].trim().parse()?,
            )
                .into(),
            a: (
                parts[6][4..].trim().parse()?,
                parts[7].trim().parse()?,
                parts[8][..(parts[8].len() - 1)].trim().parse()?,
            )
                .into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl From<(i64, i64, i64)> for Vec3 {
    fn from(t: (i64, i64, i64)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[test]
fn test_particle_from_str() {
    let s = "p=<-4897,3080,2133>, v=<-58,-15,-78>, a=<17,-7,0>";
    let p: Particle = s.parse().unwrap();
    assert_eq!(p.p, (-4897, 3080, 2133).into());
    assert_eq!(p.v, (-58, -15, -78).into());
    assert_eq!(p.a, (17, -7, 0).into());
}
