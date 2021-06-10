use crate::{Bound, WrangeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Wrange<T>
where
    T: PartialOrd + Ord + Clone,
{
    Empty,
    Convergent(Bound<T>, Bound<T>),
    Divergent(Bound<T>, Bound<T>),
    Full,
}

impl<T> Wrange<T>
where
    T: PartialOrd + Ord + Clone,
{
    pub fn new(a: Bound<T>, b: Bound<T>) -> Self {
        if a > b {
            Self::Divergent(a, b)
        } else {
            Self::Convergent(a, b)
        }
    }

    pub fn new_empty() -> Self {
        Self::Empty
    }

    pub fn new_full() -> Self {
        Self::Full
    }

    pub fn new_inclusive(a: T, b: T) -> Self {
        Self::new(Bound::Inclusive(a), Bound::Inclusive(b))
    }

    pub fn new_exclusive(a: T, b: T) -> Self {
        Self::new(Bound::Exclusive(a), Bound::Exclusive(b))
    }

    pub fn union(a: &Self, b: &Self) -> WrangeSet<T> {
        todo!()
    }

    pub fn intersection(a: &Self, b: &Self) -> WrangeSet<T> {
        use Wrange::*;
        match (a, b) {
            (Empty, _) | (_, Empty) => vec![Empty].into(),
            (Full, x) | (x, Full) => vec![x.clone()].into(),
            (Convergent(a0, a1), Convergent(b0, b1)) => {
                vec![Self::new(Bound::min(&a0, &b0), Bound::max(&a1, &b1))].into()
            }
            (Divergent(a0, a1), Divergent(b0, b1)) => {
                vec![Self::new(Bound::max(&a0, &b0), Bound::min(&a1, &b1))].into()
            }
            (Convergent(_, _), Divergent(_, _)) => Self::intersection(b, a),
            (Divergent(a0, a1), Convergent(b0, b1)) => {
                // four cases:
                match (a1 >= b0, a0 <= b1) {
                    (false, false) => vec![Empty],
                    (true, false) => vec![Self::new(Bound::max(a1, b0), b1.clone())],
                    (false, true) => vec![Self::new(b0.clone(), Bound::min(a0, b1))],
                    (true, true) => vec![
                        Self::new(Bound::max(a1, b0), b1.clone()),
                        Self::new(b0.clone(), Bound::min(a0, b1)),
                    ],
                }
                .into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_inclusive() {
        assert_eq!(
            Wrange::intersection(
                &Wrange::new_inclusive(0, 127),
                &Wrange::new_inclusive(255, 127),
            )
            .inner()[0],
            Wrange::new_inclusive(255, 0)
        );
    }
}
