use std::{fmt, ops::Mul};
use std::{fmt::Display, iter::Step, ops::Index};

use crate::twodee::{grid::Grid, Bounds, Point, PointAxe};

#[derive(Debug, Clone, PartialEq)]
pub struct VecGrid<I, C> {
    cells: Vec<C>,
    bounds: Bounds<I>,
}

impl<I, C> VecGrid<I, C>
where
    I: PointAxe,
    <I as Mul<I>>::Output: Into<isize>,
    C: Default,
{
    pub fn new(bounds: Bounds<I>) -> Self {
        let capacity: isize = (bounds.width() * bounds.height()).into();
        let mut cells = Vec::with_capacity(capacity as usize);
        cells.resize_with(capacity as usize, Default::default);

        Self { cells, bounds }
    }
}

impl<I, C> VecGrid<I, C>
where
    I: PointAxe,
    <I as Mul<I>>::Output: Into<isize>,
{
    fn point_to_idx(&self, p: Point<I>) -> usize {
        let x = p.x - self.bounds.left;
        let y = p.y - self.bounds.top;
        let idx: isize = (x + y * self.bounds.width()).into();
        idx as usize
    }
}

impl<C, I> Grid<C, I> for VecGrid<I, C>
where
    C: Display,
    I: PointAxe + Display + Step + Into<isize>,
    [C]: Index<usize, Output = C>,
{
    fn bounds(&self) -> Bounds<I> {
        self.bounds
    }

    fn get(&self, p: Point<I>) -> Option<&C> {
        if !self.bounds.contains(p) {
            None
        } else {
            self.cells.get(self.point_to_idx(p))
        }
    }

    fn set(&mut self, p: Point<I>, v: C) {
        let idx = self.point_to_idx(p);
        self.cells[idx] = v;
    }
}

impl<I, C> fmt::Display for VecGrid<I, C>
where
    I: PointAxe + fmt::Display + Step + Into<isize>,
    C: fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f)
    }
}
