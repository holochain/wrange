#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn inner(&self) -> &T {
        match self {
            Bound::Inclusive(ref t) => t,
            Bound::Exclusive(ref t) => t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
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
}
