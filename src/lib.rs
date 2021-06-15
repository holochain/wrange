//! A crate to express "wrapping ranges".
//!
//! When the start bound is less than the end bound, it is a normal continuous Range.
//! When the start bound is greater than the end bound, it is treated as the union
//! of the ranges [MIN, end] + [start, MAX].

mod bound;
pub use bound::{Bound, Bounds};

mod wrange;
pub use crate::wrange::Wrange;

mod wrange_set;
pub use wrange_set::WrangeSet;

pub mod ascii;
