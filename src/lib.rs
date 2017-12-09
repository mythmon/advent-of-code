#![feature(test)]
#![feature(conservative_impl_trait)]
extern crate test;
extern crate rand;

use std::cmp;

pub mod day8;
pub mod day9;

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
    use rand::{Rng, thread_rng};
    use test::{Bencher, black_box};

    use super::*;

    #[test]
    fn test_extremes() {
        let empty: Vec<u32> = vec![];
        assert_eq!(extremes(empty), None);
        assert_eq!(extremes(vec![1]), Some((1, 1)));
        assert_eq!(extremes(vec![0, 3, 1]), Some((0, 3)));
    }

    #[bench]
    fn bench_extremes(b: &mut Bencher) {
        let mut rng = thread_rng();
        let inp: Vec<u8> = rng.gen_iter().take(1000).collect();
        let min = inp.iter().min().unwrap();
        let max = inp.iter().max().unwrap();

        assert_eq!(Some((min, max)), extremes(&inp));

        b.iter(|| { black_box(extremes(&inp)); });
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
