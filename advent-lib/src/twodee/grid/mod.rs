use super::{Bounds, Point, PointAxe};
use std::{
    fmt,
    iter::{self, Step},
};

mod astar;
mod hash;
mod patch;

pub use hash::HashGrid;

pub trait Grid<C, I>
where
    I: PointAxe + fmt::Display + Step + Into<isize>,
    C: fmt::Display,
{
    fn bounds(&self) -> Bounds<I>;
    fn get(&self, p: Point<I>) -> Option<C>;

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
}
