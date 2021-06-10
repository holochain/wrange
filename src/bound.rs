#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bound<T> {
    Exclusive(T),
    Inclusive(T),
}

impl<T> PartialOrd for Bound<T>
where
    T: PartialOrd + Ord + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Bound<T>
where
    T: PartialOrd + Ord + Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner().cmp(other.inner())
    }
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

    pub(crate) fn inner(&self) -> &T {
        match self {
            Bound::Inclusive(ref t) => t,
            Bound::Exclusive(ref t) => t,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Constructor)]
pub struct Bounds<T>(pub(crate) Bound<T>, pub(crate) Bound<T>);

impl<T> Bounds<T>
where
    T: Clone + PartialEq + Eq + PartialOrd + Ord,
{
    /// Perform some sensible normalization:
    /// Two overlapping (colocated) endpoints with both inclusive and exclusive
    /// representation are equivalent to two overlapping inclusive endpoints
    pub fn normalized(self) -> Self {
        use Bound::*;
        match self {
            Bounds(Exclusive(ref x), ref i @ Inclusive(_)) => {
                if x == i.inner() {
                    Bounds(i.clone(), i.clone())
                } else {
                    self
                }
            }
            Bounds(ref i @ Inclusive(_), Exclusive(ref y)) => {
                if i.inner() == y {
                    Bounds(i.clone(), i.clone())
                } else {
                    self
                }
            }
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax() {
        use Bound::*;

        let e3 = Exclusive(3);
        let e7 = Exclusive(7);
        let i3 = Inclusive(3);
        let i7 = Inclusive(7);

        // Inclusive always wins tie-breakers
        assert_eq!(Bound::min(&i3, &e3), i3);
        assert_eq!(Bound::min(&e3, &i3), i3);

        // Otherwise, the inner value dictates the order
        assert_eq!(Bound::min(&i3, &i7), i3);
        assert_eq!(Bound::min(&e3, &i7), e3);
        assert_eq!(Bound::min(&i3, &e7), i3);
        assert_eq!(Bound::min(&e3, &e7), e3);
    }

    #[test]
    fn test_normalization() {
        use Bound::*;
        let e3 = Exclusive(3);
        let e7 = Exclusive(7);
        let i3 = Inclusive(3);
        let i7 = Inclusive(7);

        assert_eq!(Bounds(e3, i3).normalized(), Bounds(i3, i3));
        assert_eq!(Bounds(i3, e3).normalized(), Bounds(i3, i3));
        assert_eq!(Bounds(i3, e7).normalized(), Bounds(i3, e7));
        assert_eq!(Bounds(e3, i7).normalized(), Bounds(e3, i7));
    }
}
