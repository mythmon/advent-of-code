use crate::twodee::{
    grid::{Bounds, Grid},
    Point,
    PointAxe,
};
use std::{collections::HashMap, fmt, hash::Hash, iter::Step};

#[derive(Debug, Clone)]
pub struct PatchedGrid<'a, G, C, I>
where
    I: PointAxe + Hash + fmt::Display + Step + Into<isize>,
    G: Grid<C, I> + Clone + fmt::Debug,
    C: fmt::Display,
{
    grid: &'a G,
    patches: HashMap<Point<I>, C>,
}

impl<'a, G, C, I> PatchedGrid<'a, G, C, I>
where
    I: PointAxe + Hash + fmt::Display + Step + Into<isize>,
    G: Grid<C, I> + Clone + fmt::Debug,
    C: fmt::Display,
{
    pub fn new(grid: &'a G) -> Self {
        Self {
            grid,
            patches: HashMap::default(),
        }
    }

    pub fn with_patch(mut self, p: Point<I>, v: C) -> Self {
        self.patches.insert(p, v);
        self
    }
}

impl<'a, G, C, I> Grid<C, I> for PatchedGrid<'a, G, C, I>
where
    G: Grid<C, I> + Clone + fmt::Debug,
    I: PointAxe + Hash + fmt::Display + Step + Into<isize>,
    C: fmt::Display + Clone,
{
    fn bounds(&self) -> Bounds<I> {
        self.grid.bounds()
    }

    fn get(&self, p: Point<I>) -> Option<C> {
        self.patches.get(&p)
            .map_or_else(|| self.grid.get(p), |patch| Some(patch.clone()))
    }
}

impl<'a, G, C, I> fmt::Display for PatchedGrid<'a, G, C, I>
where
    I: PointAxe + Hash + fmt::Display + Step + Into<isize>,
    G: Grid<C, I> + Clone + fmt::Debug,
    C: fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Grid::display(self, f)
    }
}
