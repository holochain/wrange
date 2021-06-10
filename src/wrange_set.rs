use crate::Wrange;

#[derive(Debug, derive_more::From, derive_more::IntoIterator)]
pub struct WrangeSet<T>(Vec<Wrange<T>>)
where
    T: PartialOrd + Clone + std::fmt::Debug;

impl<T> WrangeSet<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    pub fn normalized(self) -> Self {
        Self(self.0.into_iter().map(|r| r.normalized()).collect())
    }

    pub fn inner(&self) -> &Vec<Wrange<T>> {
        &self.0
    }
}
