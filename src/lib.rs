//! A crate to express "wrapping ranges".
//!
//! When the start bound is less than the end bound, it is a normal continuous Range.
//! When the start bound is greater than the end bound, it is treated as the union
//! of the ranges [MIN, end] + [start, MAX].

mod bound;
pub use bound::Bound;

mod wrange;
pub use wrange::Wrange;

mod wrange_set;
pub use wrange_set::WrangeSet;

#[cfg(test)]
mod test_gfx;
