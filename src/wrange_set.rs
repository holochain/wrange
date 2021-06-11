use std::collections::BTreeSet;

use crate::Wrange;

#[derive(PartialEq, Eq, Debug, derive_more::From, derive_more::IntoIterator)]
pub struct WrangeSet<T>(BTreeSet<Wrange<T>>)
where
    T: PartialOrd + Ord + Clone + std::fmt::Debug;

impl<T> WrangeSet<T>
where
    T: PartialOrd + Ord + Clone + std::fmt::Debug,
{
    pub fn normalized(self) -> Self {
        Self(self.0.into_iter().map(|r| r.normalized()).collect())
    }

    pub fn to_vec(&self) -> Vec<Wrange<T>> {
        self.0.clone().into_iter().collect()
    }
}

impl<T> From<Vec<Wrange<T>>> for WrangeSet<T>
where
    T: PartialOrd + Ord + Clone + std::fmt::Debug,
{
    fn from(v: Vec<Wrange<T>>) -> Self {
        Self(v.into_iter().collect())
    }
}
