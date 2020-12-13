use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::{error::Error, fmt::Debug, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Copy, Clone, Debug)]
pub struct BusSpec {
    index: u64,
    schedule: u64,
}

fn parse_input_1(input: &str) -> Result<(u64, Vec<BusSpec>), Box<dyn Error>> {
    let mut lines = input.trimmed_lines();
    let now = lines
        .next()
        .ok_or_else(|| "No first line".to_string())?
        .parse()?;
    let bus_ids = parse_input_2(input)?;
    Ok((now, bus_ids))
}

fn parse_input_2(input: &str) -> Result<Vec<BusSpec>, Box<dyn Error>> {
    let mut lines = input.trimmed_lines();
    // the first line doesn't matter
    lines.next().unwrap();
    let bus_ids = lines
        .next()
        .ok_or_else(|| "No bus ids".to_string())?
        .split(',')
        // add indexes
        .enumerate()
        .map(|(index, p)| {
            // skip "x"
            if p == "x" {
                Ok(None)
            } else {
                // parse, and combine index with result
                p.parse()
                    .map(|p| Some(BusSpec { index: index as u64, schedule: p }))
                    .map_err(|err| format!("Couldn't parse {}: {}", p, err))
            }
        })
        // handle any errors
        .collect::<Result<Vec<_>, _>>()?
        // remove nones
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if let Some(l) = lines.next() {
        Err(format!("Extra data? `{:?}`", l).into())
    } else {
        Ok(bus_ids)
    }
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = (u64, Vec<BusSpec>);
    type Output = u64;

    fn name(&self) -> String {
        "2020-D13-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input_1)
            .transformed_case("Example", "939\n7,x,13,x,x,59,x,31,19", 295)?
            .transformed_case("Solution", include_str!("input"), 296)?
            .collect())
    }

    fn try_run_puzzle((now, bus_ids): Self::Input) -> Result<Self::Output, Self::Error> {
        bus_ids
            .into_iter()
            .map(|spec| (spec.schedule, now + (spec.schedule - now % spec.schedule)))
            .min_by_key(|(_id, next)| *next)
            .ok_or_else(|| "No answer found".into())
            .map(|(id, next)| id * (next - now))
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<BusSpec>;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D13-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input_2)
            .transformed_case("Example 1", "0\n7,13,x,x,59,x,31,19", 1_068_781)?
            .transformed_case("Example 2", "0\n17,x,13,19", 3_417)?
            .transformed_case("Example 3", "0\n67,7,59,61", 754_018)?
            .transformed_case("Example 4", "0\n67,x,7,59,61", 779_210)?
            .transformed_case("Example 5", "0\n67,7,x,59,61", 1_261_476)?
            .transformed_case("Example 6", "0\n1789,37,47,1889", 1_202_161_486)?
            .transformed_case("Solution", include_str!("input"), 535_296_695_251_210)?
            .collect())
    }

    fn try_run_puzzle(bus_specs: Self::Input) -> Result<Self::Output, Self::Error> {
        if bus_specs.is_empty() {
            return Err("No buses".into());
        }
        Ok(part2_helper(
            // ((bus_specs[0].index)..).step_by(bus_specs[0].schedule as usize),
            0..,
            &bus_specs,
        )?)
    }
}

fn part2_helper<I: Iterator<Item = u64> + Debug>(
    steps: I,
    bus_specs: &[BusSpec],
) -> Result<u64, String> {
    // Iterate through `steps` until the first item in the list matches twice
    // (or stop early if the first match also matches the rest of the list).
    // After the second time it matches, increase the step size to the
    // difference between the two, and recurse using the tail of the list.
    //
    // The difference between the two hits is the period at which that item and
    // all the previous ones in the call stack repeat. The answer for the rest
    // of the list will be a multiple of that period.
    //
    // This is a big speed up, since the step size grows multiplicatively as we
    // progress through the list.

    if bus_specs.is_empty() {
        return Err("No more buses (should this have returned previously?)".into());
    }

    let next = bus_specs[0];

    let mut first_hit: Option<u64> = None;

    let is_solution = |t: u64| -> bool {
        bus_specs
            .iter()
            .all(|spec| (t + spec.index) % spec.schedule == 0)
    };

    for t in steps {
        if (t + next.index) % next.schedule == 0 {
            if is_solution(t) {
                return Ok(t);
            } else {
                match first_hit {
                    None => {
                        first_hit = Some(t);
                    }

                    Some(first_hit) => {
                        return part2_helper(
                            (t..).step_by((t - first_hit) as usize),
                            &bus_specs[1..],
                        );
                    }
                }
            }
        }
    }

    Err("No answer found".into())
}
