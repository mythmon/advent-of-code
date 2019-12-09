use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use itertools::Itertools;
use lalrpop_util::lalrpop_mod;
use std::{collections::HashMap, iter::Iterator, str::FromStr};

#[cfg(windows)]
lalrpop_mod!(parser, "\\day04\\parser.rs");
#[cfg(unix)]
lalrpop_mod!(parser);

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2018-D04-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 3_212_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let log: Result<Vec<SleepLogLine>, String> =
            input.trimmed_lines().map(str::parse).collect();
        let mut log = log.expect("Couldn't parse input");
        log.sort_by_key(|log| log.datetime);

        let mut latest_guard_id = match log[0].event {
            LogEvent::BeginsShift { id } => id,
            _ => panic!("Input is invalid: did not start with a start of shift"),
        };
        for mut line in &mut log {
            line.guard_id = match line.event {
                LogEvent::BeginsShift { id } => {
                    latest_guard_id = id;
                    Some(id)
                }
                _ => Some(latest_guard_id),
            }
        }

        // which guard spent the most minutes asleep:
        let mut guard_asleep_total = HashMap::new();
        let sleep_wake_cycles = log
            .iter()
            .filter(|l| match l.event {
                LogEvent::BeginsShift { .. } => false,
                _ => true,
            })
            .tuples();
        for (asleep, awake) in sleep_wake_cycles {
            assert_eq!(awake.guard_id, asleep.guard_id);
            assert!(awake.datetime > asleep.datetime);
            assert!(awake.datetime.year == asleep.datetime.year);
            assert!(awake.datetime.month == asleep.datetime.month);
            assert!(awake.datetime.hour == asleep.datetime.hour);
            match (awake.event, asleep.event) {
                (LogEvent::WakesUp, LogEvent::FallsAsleep) => (),
                _ => panic!("Unexpected input"),
            };
            *guard_asleep_total
                .entry(awake.guard_id.unwrap())
                .or_insert(0) += u32::from(awake.datetime.minute - asleep.datetime.minute);
        }

        let guard_most_asleep: u32 = guard_asleep_total
            .into_iter()
            .max_by_key(|(_guard_id, minutes_asleep)| *minutes_asleep)
            .unwrap()
            .0;

        // what minute was that guard most often asleep?
        let guard_sleep_wake_cycles = log
            .iter()
            .filter(|l| {
                l.guard_id == Some(guard_most_asleep)
                    && match l.event {
                        LogEvent::BeginsShift { .. } => false,
                        _ => true,
                    }
            })
            .tuples();
        let mut asleep_minutes = [0_u32; 60];
        for (asleep, awake) in guard_sleep_wake_cycles {
            for idx in asleep.datetime.minute..awake.datetime.minute {
                asleep_minutes[idx as usize] += 1;
            }
        }
        let most_asleep_minute = asleep_minutes
            .iter()
            .enumerate()
            .max_by_key(|(_idx, asleep)| *asleep)
            .unwrap()
            .0 as u32;

        guard_most_asleep * most_asleep_minute
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2018-D04-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 4_966_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let log: Result<Vec<SleepLogLine>, String> =
            input.trimmed_lines().map(str::parse).collect();
        let mut log = log.expect("Couldn't parse input");
        log.sort_by_key(|log| log.datetime);

        let mut latest_guard_id = match log[0].event {
            LogEvent::BeginsShift { id } => id,
            _ => panic!("Input is invalid: did not start with a start of shift"),
        };
        for mut line in &mut log {
            line.guard_id = match line.event {
                LogEvent::BeginsShift { id } => {
                    latest_guard_id = id;
                    Some(id)
                }
                _ => Some(latest_guard_id),
            }
        }

        // which guard spent the most minutes asleep:
        let mut guard_asleep_at_minute_counts = HashMap::new();
        let sleep_wake_cycles = log
            .iter()
            .filter(|l| match l.event {
                LogEvent::BeginsShift { .. } => false,
                _ => true,
            })
            .tuples();
        for (asleep, awake) in sleep_wake_cycles {
            for minute in asleep.datetime.minute..awake.datetime.minute {
                *guard_asleep_at_minute_counts
                    .entry((asleep.guard_id.unwrap(), minute))
                    .or_insert(0) += 1;
            }
        }
        let (guard_id, most_asleep_minute) = guard_asleep_at_minute_counts
            .iter()
            .max_by_key(|((_guard_id, _minute), times_asleep)| *times_asleep)
            .unwrap()
            .0;

        *guard_id * u32::from(*most_asleep_minute)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct DateTime {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum LogEvent {
    BeginsShift { id: u32 },
    FallsAsleep,
    WakesUp,
}

#[derive(Clone, Debug)]
pub struct SleepLogLine {
    datetime: DateTime,
    event: LogEvent,
    guard_id: Option<u32>,
}

impl FromStr for SleepLogLine {
    // TODO better error handling
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::SleepLogLineParser::new()
            .parse(s)
            .map_err(|e| format!("Error parsing {}: {}", s, e))
    }
}
