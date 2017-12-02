#![feature(test)]
extern crate test;
extern crate rand;

use std::cmp;

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

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use rand::{thread_rng, Rng};

    use super::extremes;

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

        b.iter(|| {
            black_box(extremes(&inp));
        });
    }

}
