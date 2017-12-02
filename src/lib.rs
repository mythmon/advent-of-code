#![feature(test)]
extern crate test;
extern crate rand;

/// Find the lowest and highest value in `items`.
pub fn extremes<T>(items: T) -> Option<(T::Item, T::Item)>
where
    T: IntoIterator,
    T::Item: Ord + Copy,
{
    let mut min = None;
    let mut max = None;

    for item in items.into_iter() {
        match min {
            Some(m) if m < item => {},
            _ => min = Some(item),
        }

        match max {
            Some(m) if m > item => {},
            _ => max = Some(item),
        }
    }

    match (min, max) {
        (Some(low), Some(high)) => Some((low, high)),
        _ => None,
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

        b.iter(|| {
            black_box(extremes(&inp));
        });
    }

}
