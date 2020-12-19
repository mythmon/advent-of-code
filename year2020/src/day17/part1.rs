use advent_lib::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
    twodee::PointAxe,
};
use std::{
    collections::HashMap,
    error::Error,
    hash::Hash,
    iter::{Iterator, Step},
    ops::Neg,
};

fn parse_input<P>(input: &str) -> Result<HashGrid3<P>, Box<dyn Error>>
where
    P: PointAxe + Hash + From<i32> + Ord,
{
    Ok(HashGrid3::from_2d_slice(input)?)
}

#[derive(Clone, Debug, Default)]
pub struct HashGrid3<P: PointAxe> {
    cells: HashMap<Point3<P>, bool>,
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, Default)]
pub struct Point3<P> {
    x: P,
    y: P,
    z: P,
}

impl<P> Point3<P> {
    fn new<IX, IY, IZ>(x: IX, y: IY, z: IZ) -> Self
    where
        IX: Into<P>,
        IY: Into<P>,
        IZ: Into<P>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl<P> Point3<P>
where
    P: PointAxe + Neg<Output = P> + Step,
{
    fn neighbors26(&self) -> Vec<Point3<P>> {
        let zero = P::zero();
        let one = P::one();

        let mut rv = Vec::with_capacity(26);
        for dx in -one..=one {
            for dy in -one..=one {
                for dz in -one..=one {
                    if dx == zero && dy == zero && dz == zero {
                        continue;
                    }
                    rv.push(Point3::new(self.x + dx, self.y + dy, self.z + dz));
                }
            }
        }
        rv
    }
}

impl<P> HashGrid3<P>
where
    P: PointAxe + Hash + From<i32> + Ord,
{
    fn from_2d_slice(s: &str) -> Result<Self, String> {
        let mut cells = HashMap::new();
        for (x, line) in s.trimmed_lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                cells.insert(
                    Point3::new(x as i32, y as i32, P::zero()),
                    match c {
                        '.' => Ok(false),
                        '#' => Ok(true),
                        x => Err(format!("Invalid cell {}", x)),
                    }?,
                );
            }
        }
        Ok(Self { cells })
    }

    fn bounds(&self) -> Option<(Point3<P>, Point3<P>)> {
        let mut mins: Option<Point3<P>> = None;
        let mut maxs: Option<Point3<P>> = None;

        for p in self.cells.keys() {
            mins = Some(if let Some(Point3 { x, y, z }) = mins {
                Point3::new(x.min(p.x), y.min(p.y), z.min(p.z))
            } else {
                *p
            });
            maxs = Some(if let Some(Point3 { x, y, z }) = maxs {
                Point3::new(x.max(p.x), y.max(p.y), z.max(p.z))
            } else {
                *p
            });
        }

        match (mins, maxs) {
            (Some(min), Some(max)) => Some((min, max)),
            (None, None) => None,
            _ => panic!("Bug"),
        }
    }
}

#[derive(Debug)]
pub struct Runner;

impl PuzzleRunner for Runner {
    type Input = HashGrid3<i32>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D17-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case("Example", ".#.\n..#\n###", 112)?
            .transformed_case("Solution", include_str!("input"), 276)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut current = input;
        let mut next = HashGrid3::<i32>::default();

        for _ in 0..6 {
            let (min, max) = current.bounds().ok_or("Empty grid")?;
            for x in (min.x - 1)..=(max.x + 1) {
                for y in (min.y - 1)..=(max.y + 1) {
                    for z in (min.z - 1)..=(max.z + 1) {
                        let p = Point3::new(x, y, z);
                        let neighbor_count = p
                            .neighbors26()
                            .iter()
                            .filter(|n| *current.cells.get(n).unwrap_or(&false))
                            .count();
                        let c = *current.cells.get(&p).unwrap_or(&false);
                        next.cells.insert(
                            p,
                            match (c, neighbor_count) {
                                (true, n) => n == 2 || n == 3,
                                (false, n) => n == 3,
                            },
                        );
                    }
                }
            }
            current = next;
            next = HashGrid3::default();
        }

        Ok(current.cells.into_values().filter(|c| *c).count())
    }
}
