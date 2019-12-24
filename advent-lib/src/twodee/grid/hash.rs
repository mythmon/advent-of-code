use crate::twodee::{
    grid::{patch::PatchedGrid, Grid},
    Bounds,
    Point,
    PointAxe,
};
use std::{
    cmp,
    collections::{hash_map::Entry, HashMap},
    fmt,
    hash::Hash,
    iter::Step,
};

#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct HashGrid<C, I = isize>
where
    I: PointAxe + Hash + Eq,
{
    cells: HashMap<Point<I>, C>,
}

impl<C, I> Grid<C, I> for HashGrid<C, I>
where
    I: PointAxe + Hash + fmt::Display + Step + Into<isize> + cmp::Ord,
    C: Copy + fmt::Display,
{
    fn bounds(&self) -> Bounds<I> {
        self.cells.iter().map(|(k, _v)| *k).collect()
    }

    #[must_use]
    fn get(&self, p: Point<I>) -> Option<C> {
        self.cells.get(&p).copied()
    }
}

impl<C, I> HashGrid<C, I>
where
    I: PointAxe + Hash + fmt::Display + Step + Into<isize> + cmp::Ord,
    C: Copy + fmt::Debug + fmt::Display,
{
    pub fn set(&mut self, p: Point<I>, v: C) {
        self.cells.insert(p, v);
    }

    pub fn entry(&mut self, p: Point<I>) -> Entry<Point<I>, C> {
        self.cells.entry(p)
    }

    pub fn remove(&mut self, p: &Point<I>) -> Option<C> {
        self.cells.remove(p)
    }

    pub fn with_patch(&self, p: Point<I>, v: C) -> PatchedGrid<Self, C, I> {
        PatchedGrid::new(self).with_patch(p, v)
    }
}

impl<C> Default for HashGrid<C>
where
    C: Default + fmt::Debug + Hash + fmt::Display,
{
    fn default() -> Self {
        Self {
            cells: HashMap::default(),
        }
    }
}
