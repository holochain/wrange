use crate::Wrange;

#[derive(derive_more::From, derive_more::IntoIterator)]
pub struct WrangeSet<T>(Vec<Wrange<T>>)
where
    T: PartialOrd + Ord + Clone;

impl<T> WrangeSet<T>
where
    T: PartialOrd + Ord + Clone,
{
    pub fn inner(&self) -> &Vec<Wrange<T>> {
        &self.0
    }
}
