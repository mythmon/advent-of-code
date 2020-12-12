use super::{Bounds, Point, PointAxe};
use std::{
    fmt::{self, Debug},
    iter::{self, Step},
    marker::PhantomData,
    ops::AddAssign,
};

mod astar;
mod hash;
mod patch;
mod vec;

use fmt::Display;
pub use hash::HashGrid;
pub use vec::VecGrid;

pub trait Grid<C, I>
where
    I: PointAxe + fmt::Display + Step + Into<isize>,
    C: fmt::Display,
{
    fn bounds(&self) -> Bounds<I>;
    fn get(&self, p: Point<I>) -> Option<&C>;
    fn set(&mut self, p: Point<I>, v: C);

    /// # Errors
    /// Will return an error if any of called formatters do.
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bounds = self.bounds();

        writeln!(f, "{}", bounds.top_left())?;

        for y in (bounds.top)..=(bounds.bottom) {
            for x in (bounds.left)..=(bounds.right) {
                if let Some(cell) = self.get(Point::new(x, y)) {
                    write!(f, "{}", cell)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        let padding_mag: isize = (bounds.right - bounds.left).into();
        write!(
            f,
            "{}{}",
            iter::repeat(" ")
                .take(padding_mag.abs() as usize)
                .collect::<String>(),
            bounds.bottom_right()
        )?;

        Ok(())
    }

    fn iter_coords(&self) -> CoordinateIterator<I> {
        CoordinateIterator::new(self.bounds())
    }

    fn iter_values(&self) -> ValueIterator<'_, Self, I, C>
    where
        Self: Sized,
    {
        ValueIterator::new(self)
    }
}

#[derive(Clone, Debug)]
pub struct CoordinateIterator<I>
where
    I: PointAxe,
{
    cursor: Point<I>,
    bounds: Bounds<I>,
}

impl<I> CoordinateIterator<I>
where
    I: PointAxe,
{
    pub fn new(bounds: Bounds<I>) -> Self {
        Self {
            cursor: bounds.top_left(),
            bounds,
        }
    }
}

impl<I> Iterator for CoordinateIterator<I>
where
    I: PointAxe,
    I: AddAssign,
{
    type Item = Point<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.cursor.clone();

        if self.bounds.contains(p) {
            self.cursor.x += I::one();
            if !self.bounds.contains(self.cursor) {
                self.cursor.x = self.bounds.left;
                self.cursor.y += I::one();
            }

            Some(p)
        } else {
            None
        }
    }
}

pub struct ValueIterator<'a, G, I, C>
where
    G: Grid<C, I>,
    C: Display,
    I: Debug + Display + PointAxe + Step + Into<isize>,
{
    coord_iter: CoordinateIterator<I>,
    grid: &'a G,
    phantom: PhantomData<C>,
}

impl<'a, G, I, C> ValueIterator<'a, G, I, C>
where
    G: Grid<C, I>,
    C: Display,
    I: Debug + Display + PointAxe + Step + Into<isize>,
{
    pub fn new(grid: &'a G) -> Self {
        Self {
            grid,
            coord_iter: grid.iter_coords(),
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, G, I, C> Iterator for ValueIterator<'a, G, I, C>
where
    G: Grid<C, I>,
    C: Display + 'a,
    I: Debug + Display + PointAxe + Step + Into<isize> + AddAssign,
{
    type Item = &'a C;

    fn next(&mut self) -> Option<Self::Item> {
        self.coord_iter.next().and_then(|p| self.grid.get(p))
    }
}

// impl<'a, G, C, I> Iterator for GridValueIterator<'a, G, I, C>
// where
//     G: Grid<C, I>,
// {
//     type Item = C;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
