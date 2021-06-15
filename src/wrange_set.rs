use std::collections::HashSet;
use std::hash::Hash;

use crate::Wrange;

#[derive(PartialEq, Eq, Debug, derive_more::From, derive_more::IntoIterator)]
pub struct WrangeSet<T>(HashSet<Wrange<T>>)
where
    T: PartialOrd + Ord + Hash + Clone + std::fmt::Debug;

impl<T> WrangeSet<T>
where
    T: PartialOrd + Ord + Hash + Clone + std::fmt::Debug,
{
    pub fn normalized(self) -> Self {
        Self(self.0.into_iter().map(|r| r.normalized()).collect())
    }

    pub fn to_vec(&self) -> Vec<Wrange<T>> {
        self.0.clone().into_iter().collect()
    }

    pub fn union(a: &Self, b: &Self) -> Self {
        Self(a.0.iter().chain(b.0.iter()).cloned().collect())
    }

    pub fn intersection(a: &Self, b: &Self) -> Self {
        let mut sets =
            a.0.iter()
                .flat_map(|a| b.0.iter().map(move |b| Wrange::intersection(a, b)));
        if let Some(first) = sets.next() {
            sets.fold(first, |a, b| Self::union(&a, &b))
        } else {
            vec![Wrange::Empty].into()
        }
    }
}

impl<T> From<Vec<Wrange<T>>> for WrangeSet<T>
where
    T: PartialOrd + Ord + Hash + Clone + std::fmt::Debug,
{
    fn from(v: Vec<Wrange<T>>) -> Self {
        Self(v.into_iter().collect())
    }
}

impl<T> From<Wrange<T>> for WrangeSet<T>
where
    T: PartialOrd + Ord + Hash + Clone + std::fmt::Debug,
{
    fn from(r: Wrange<T>) -> Self {
        vec![r].into()
    }
}
