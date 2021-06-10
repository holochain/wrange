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

    // pub fn normalized(self) -> Self {
    //     use Wrange::*;
    //     match self {
    //         (Convergent(a0, a1)
    //     }
    // }

    pub fn union(a: &Self, b: &Self) -> WrangeSet<T> {
        todo!()
    }

    pub fn intersection(a: &Self, b: &Self) -> WrangeSet<T> {
        use Wrange::*;
        match (a, b) {
            (Empty, _) | (_, Empty) => vec![Empty].into(),
            (Full, x) | (x, Full) => vec![x.clone()].into(),
            (Convergent(a0, a1), Convergent(b0, b1)) => {
                if a0 > b0 {
                    Self::intersection(b, a)
                } else if a1 < b0 {
                    vec![Empty].into()
                } else {
                    vec![Self::new(Bound::max(&a0, &b0), Bound::min(&a1, &b1))].into()
                }
            }
            (Divergent(a0, a1), Divergent(b0, b1)) => {
                vec![Self::new(Bound::max(&a0, &b0), Bound::min(&a1, &b1))].into()
            }
            (Convergent(_, _), Divergent(_, _)) => Self::intersection(b, a),
            (Divergent(a0, a1), Convergent(b0, b1)) => {
                // four cases:
                match (a1 >= b0, a0 <= b1) {
                    (false, false) => vec![Empty],
                    (true, false) => vec![Self::new(Bound::min(a1, b0), a1.clone())],
                    (false, true) => vec![Self::new(a0.clone(), Bound::max(a0, b1))],
                    (true, true) => vec![
                        Self::new(Bound::min(a1, b0), Bound::max(a1, b0)),
                        Self::new(Bound::min(a0, b1), Bound::max(a0, b1)),
                    ],
                }
                .into()
            }
        }
    }
}

fn normalize_pair<T>(a: Bound<T>, b: Bound<T>) -> (Bound<T>, Bound<T>)
where
    T: Clone + PartialEq + Eq,
{
    use Bound::*;
    match (&a, &b) {
        (Exclusive(x), Inclusive(y)) => {
            if x == y {
                (b.clone(), b)
            } else {
                (a, b)
            }
        }
        (Inclusive(x), Exclusive(y)) => {
            if x == y {
                (a.clone(), a)
            } else {
                (a, b)
            }
        }
        _ => (a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_gfx::gfx;

    macro_rules! assert_intersects_single {
        ($a: expr, $b: expr, $e: expr $(,)?) => {
            assert_eq!(Wrange::intersection(&$a, &$b).inner()[0], $e);
            assert_eq!(Wrange::intersection(&$b, &$a).inner()[0], $e);
        };
    }

    macro_rules! assert_intersects_double {
        ($a: expr, $b: expr, $e1: expr, $e2: expr $(,)?) => {
            assert_eq!(*Wrange::intersection(&$a, &$b).inner(), vec![$e1, $e2]);
            assert_eq!(*Wrange::intersection(&$b, &$a).inner(), vec![$e1, $e2]);
        };
    }

    #[test]
    fn test_intersections_convergent_convergent() {
        assert_intersects_single!(
            gfx("  o-----o       "),
            gfx("     o----o     "),
            gfx("     o--o       "),
        );

        assert_intersects_single!(
            gfx("     o----o     "),
            gfx("  o-----o       "),
            gfx("     o--o       "),
        );

        assert_intersects_single!(
            gfx("  o----o       "),
            gfx("          o--o "),
            gfx("               "),
        );

        assert_intersects_single!(
            gfx("          o--o "),
            gfx("  o----o       "),
            gfx("               "),
        );

        assert_intersects_single!(
            gfx("       o----o  "),
            gfx("  o----o       "),
            gfx("       o       "),
        );

        assert_intersects_single!(
            gfx("       x----o  "),
            gfx("  o----o       "),
            gfx("       o       "),
        );
    }

    #[test]
    fn test_intersections_divergent_convergent() {
        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("     o----o     "),
            gfx("                "),
        );

        assert_intersects_single!(
            gfx("---x        x---"),
            gfx("     o----o     "),
            gfx("                "),
        );

        assert_intersects_single!(
            gfx("---o        o---"),
            gfx(" o-----o        "),
            gfx(" o-o            "),
        );

        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("        o-----o "),
            gfx("            o-o "),
        );

        assert_intersects_double!(
            gfx("----o      o----"),
            gfx(" o------------o "),
            gfx(" o--o           "),
            gfx("           o--o "),
        );
    }

    #[test]
    fn test_intersections_with_overlapping_endpoints() {}
}
