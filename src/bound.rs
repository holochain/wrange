use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub enum Bound<T> {
    Exclusive(T),
    Inclusive(T),
}

impl<T> PartialOrd for Bound<T>
where
    T: PartialOrd + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Bound::*;
        match self.inner().partial_cmp(other.inner()) {
            Some(Ordering::Equal) => match (self, other) {
                (Exclusive(_), Exclusive(_)) | (Inclusive(_), Inclusive(_)) => {
                    Some(Ordering::Equal)
                }
                _ => None,
            },
            o => o,
        }
    }
}

impl<T> PartialEq for Bound<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        use Bound::*;
        match (self, other) {
            (Exclusive(x), Exclusive(y)) | (Inclusive(x), Inclusive(y)) => x == y,
            _ => false,
        }
    }
}

impl<T> Eq for Bound<T> where T: PartialEq + Clone {}

impl<T> Bound<T>
where
    T: PartialOrd + Clone,
{
    pub fn intersection_min(a: &Self, b: &Self) -> Self {
        if a.inner() == b.inner() {
            match (a, b) {
                (Bound::Inclusive(_), Bound::Exclusive(_)) => b,
                (Bound::Exclusive(_), Bound::Inclusive(_)) => a,
                _ => a,
            }
        } else if a < b {
            a
        } else {
            b
        }
        .to_owned()
    }

    pub fn intersection_max(a: &Self, b: &Self) -> Self {
        if a.inner() == b.inner() {
            match (a, b) {
                (Bound::Inclusive(_), Bound::Exclusive(_)) => b,
                (Bound::Exclusive(_), Bound::Inclusive(_)) => a,
                _ => a,
            }
        } else if a > b {
            a
        } else {
            b
        }
        .to_owned()
    }

    pub fn union_min(a: &Self, b: &Self) -> Self {
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

    pub fn union_max(a: &Self, b: &Self) -> Self {
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
pub struct Bounds<T>(pub Bound<T>, pub Bound<T>)
where
    T: Clone + PartialEq + PartialOrd;

impl<T> Bounds<T>
where
    T: Clone + PartialEq + PartialOrd,
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
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_order() {
        use Bound::*;

        let e3 = Exclusive(3);
        let e7 = Exclusive(7);
        let i3 = Inclusive(3);
        let i7 = Inclusive(7);

        assert_eq!(i3.partial_cmp(&i3), Some(Ordering::Equal));
        assert_eq!(i3.partial_cmp(&e3), None);
        assert_eq!(e3.partial_cmp(&i3), None);

        assert_eq!(e3.partial_cmp(&i7), Some(Ordering::Less));
        assert_eq!(i3.partial_cmp(&e7), Some(Ordering::Less));

        assert_eq!(e7.partial_cmp(&i3), Some(Ordering::Greater));
        assert_eq!(i7.partial_cmp(&e3), Some(Ordering::Greater));

        assert!(i3 != e3);
        assert!(!(e3 < i3));
        assert!(!(i3 < e3));
        assert!(!(e3 > i3));
        assert!(!(i3 > e3));

        assert!(e3 <= i7);
        assert!(i3 <= e7);
        assert!(i3 != e7);
    }

    #[test]
    fn test_minmax() {
        use Bound::*;

        let e3 = Exclusive(3);
        let e7 = Exclusive(7);
        let i3 = Inclusive(3);
        let i7 = Inclusive(7);

        // Inclusive always wins tie-breakers in unions
        assert_eq!(Bound::union_min(&i3, &e3), i3);
        assert_eq!(Bound::union_min(&e3, &i3), i3);

        // Exclusive always wins tie-breakers in intersections
        assert_eq!(Bound::intersection_min(&i3, &e3), e3);
        assert_eq!(Bound::intersection_min(&e3, &i3), e3);

        // Otherwise, the inner value dictates the order
        assert_eq!(Bound::union_min(&i3, &i7), i3);
        assert_eq!(Bound::union_min(&e3, &i7), e3);
        assert_eq!(Bound::intersection_min(&i3, &e7), i3);
        assert_eq!(Bound::intersection_min(&e3, &e7), e3);
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
