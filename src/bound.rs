#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bound<T> {
    Exclusive(T),
    Inclusive(T),
}

impl<T> Bound<T>
where
    T: PartialOrd + Ord + Clone,
{
    pub fn min(a: &Self, b: &Self) -> Self {
        if a.inner() == b.inner() {
            match (a, b) {
                (Bound::Inclusive(_), Bound::Exclusive(_)) => a,
                (Bound::Exclusive(_), Bound::Inclusive(_)) => b,
                _ => a,
            }
        } else if a < b {
            a
        } else {
            b
        }
        .to_owned()
    }

    pub fn max(a: &Self, b: &Self) -> Self {
        if a.inner() == b.inner() {
            match (a, b) {
                (Bound::Inclusive(_), Bound::Exclusive(_)) => a,
                (Bound::Exclusive(_), Bound::Inclusive(_)) => b,
                _ => a,
            }
        } else if a > b {
            a
        } else {
            b
        }
        .to_owned()
    }

    fn inner(&self) -> &T {
        match self {
            Bound::Inclusive(ref t) => t,
            Bound::Exclusive(ref t) => t,
        }
    }
}
