use crate::twodee::{
    grid::{Grid, HashGrid},
    Point,
    PointAxe,
};
use num_traits::{bounds::Bounded, sign::Signed};
use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    iter,
};

impl<'a, C, I> HashGrid<C, I>
where
    Self: 'a + Grid<C, I>,
    C: Default + fmt::Debug + Hash + Eq + Copy + fmt::Display,
    I: PointAxe + Hash + Ord + Signed + Bounded + fmt::Display + iter::Step + Into<isize>,
{
    pub fn astar(
        &self,
        start: Point<I>,
        goal: Point<I>,
        walkable: &HashSet<C>,
    ) -> Option<Vec<Point<I>>> {
        // Set of discovered nodes that may need to be expanded or re-expanded.
        let mut open_set: HashSet<Point<I>> = HashSet::default();
        open_set.insert(start);

        // For node `n`, `cameFrom[n]` is the node immediately before `n` on the
        // cheapest known path from `start` to `n`.
        let mut came_from: HashMap<Point<I>, Point<I>> = HashMap::default();

        // For node `n`, `gScore[n]` is the cost of the cheapest known path from
        // `start` to `n`. It should default to the highest possible cost.
        let mut g_score: HashMap<Point<I>, I> = HashMap::default();
        g_score.insert(start, I::zero());

        // For node `n`, `fScore[n]` is `gScore[n] + h(n)
        // `start` to `n`. It should default to the highest possible cost.
        let mut f_score: HashMap<Point<I>, I> = HashMap::new();
        f_score.insert(start, start.manhattan_distance(goal));

        // A stand-in for "Infinity" in the usual algorithm
        let max_distance = I::max_value();

        while !open_set.is_empty() {
            let current: Point<I> = {
                let (rv, _) = open_set
                    .iter()
                    .map(|p| (*p, f_score.get(p).unwrap_or(&max_distance)))
                    .min_by_key(|(_p, f)| *f)
                    .unwrap();
                open_set.remove(&rv);
                rv
            };

            if current == goal {
                return Some(reconstruct_path(&came_from, current));
            }
            // println!(
            //     "      Considering {:?} (f={:?})",
            //     &current,
            //     f_score.get(&current),
            // );
            for &neighbor in &current.neighbors4() {
                // print!("        Scoring neighbor at {:?}", &neighbor);
                let neighbor_cell: C = match self.get(neighbor) {
                    Some(c) => c,
                    None => continue,
                };
                if !walkable.contains(&neighbor_cell) {
                    // println!(" not walkable");
                    continue;
                }
                let tentative_g_score = g_score[&current] + I::one();
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&max_distance) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(
                        neighbor,
                        tentative_g_score + neighbor.manhattan_distance(goal),
                    );
                    // println!(" improved to f={:?}", f_score.get(&neighbor).unwrap());
                    open_set.insert(neighbor);
                } else {
                    // println!(" (not a better route)");
                }
            }
        }

        None
    }
}

fn reconstruct_path<I>(came_from: &HashMap<Point<I>, Point<I>>, current: Point<I>) -> Vec<Point<I>>
where
    I: PointAxe + Hash + Ord,
{
    let mut rv = vec![current];
    let mut prev = current;
    while let Some(next) = came_from.get(&prev) {
        rv.push(*next);
        prev = *next;
    }
    rv.reverse();
    rv
}
