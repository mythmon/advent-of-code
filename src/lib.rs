#![deny(clippy::all)]
#![feature(slice_patterns, range_contains, associated_type_defaults, inner_deref)]

use std::cmp;

pub mod cases;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

/// Find the lowest and highest value in `items`.
pub fn extremes<T>(items: T) -> Option<(T::Item, T::Item)>
where
    T: IntoIterator,
    T::Item: Ord + Copy,
{
    let mut items = items.into_iter();
    if let Some(first) = items.next() {
        let mut min = first;
        let mut max = first;

        for item in items {
            min = cmp::min(min, item);
            max = cmp::max(max, item);
        }

        Some((min, max))
    } else {
        None
    }
}

pub fn odds() -> impl Iterator<Item = u32> {
    (1..).filter(|n| n % 2 == 1)
}

pub fn evens() -> impl Iterator<Item = u32> {
    (0..).filter(|n| n % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extremes() {
        let empty: Vec<u32> = vec![];
        assert_eq!(extremes(empty), None);
        assert_eq!(extremes(vec![1]), Some((1, 1)));
        assert_eq!(extremes(vec![0, 3, 1]), Some((0, 3)));
    }

    #[test]
    fn test_odds() {
        let xs: Vec<u32> = odds().take(5).collect();
        assert_eq!(xs, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_evens() {
        let xs: Vec<u32> = evens().take(5).collect();
        assert_eq!(xs, vec![0, 2, 4, 6, 8]);
    }
}
