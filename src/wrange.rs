use crate::{bound::Bounds, Bound, WrangeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Wrange<T>
where
    T: PartialOrd + Ord + Clone,
{
    Empty,
    Convergent(Bounds<T>),
    Divergent(Bounds<T>),
    Full,
}

impl<T> Wrange<T>
where
    T: PartialOrd + Ord + Clone,
{
    pub fn new(a: Bound<T>, b: Bound<T>) -> Self {
        if a > b {
            Self::Divergent(Bounds(a, b))
        } else {
            Self::Convergent(Bounds(a, b))
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

    /// Perform some sensible normalizations:
    /// - Two overlapping (colocated) endpoints with both inclusive and exclusive
    ///   representation are equivalent to two overlapping inclusive endpoints
    /// - A convergent range with overlapping exclusive endpoints is equivalent to Empty
    /// - A divergent range with overlapping inclusive endpoints is equivalent to Full
    ///
    /// Note that Wrange does not know about the min and max limits of the range,
    /// nor whether T is continuous or discrete, so this function *cannot* make determinations
    /// such as: "a convergent range with inclusive endpoints at MIN and MAX is equivalent to Full"
    pub fn normalized(self) -> Self {
        use Bound::*;
        use Wrange::*;
        match self {
            Convergent(p) => match p.normalized() {
                Bounds(Exclusive(x), Exclusive(y)) if x == y => Empty,
                p => Convergent(p),
            },
            Divergent(p) => match p.normalized() {
                Bounds(Inclusive(x), Inclusive(y)) if x == y => Full,
                p => Divergent(p),
            },
            Empty => Empty,
            Full => Full,
        }
    }

    pub fn union(a: &Self, b: &Self) -> WrangeSet<T> {
        todo!()
    }

    pub fn intersection(a: &Self, b: &Self) -> WrangeSet<T> {
        use Wrange::*;
        match (a, b) {
            (Empty, _) | (_, Empty) => vec![Empty].into(),
            (Full, x) | (x, Full) => vec![x.clone()].into(),
            (Convergent(Bounds(a0, a1)), Convergent(Bounds(b0, b1))) => {
                if a0 > b0 {
                    Self::intersection(b, a)
                } else if a1 < b0 {
                    vec![Empty].into()
                } else {
                    vec![Self::new(Bound::max(&a0, &b0), Bound::min(&a1, &b1))].into()
                }
            }
            (Divergent(Bounds(a0, a1)), Divergent(Bounds(b0, b1))) => {
                vec![Self::new(Bound::max(&a0, &b0), Bound::min(&a1, &b1))].into()
            }
            (Convergent(Bounds(_, _)), Divergent(Bounds(_, _))) => Self::intersection(b, a),
            (Divergent(Bounds(a0, a1)), Convergent(Bounds(b0, b1))) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_gfx::gfx;

    macro_rules! assert_intersects_single {
        ($a: expr, $b: expr, $e: expr $(,)?) => {
            assert_eq!(Wrange::<u8>::intersection(&$a, &$b).normalized().inner()[0], $e);
            assert_eq!(Wrange::<u8>::intersection(&$b, &$a).normalized().inner()[0], $e);
        };
    }

    macro_rules! assert_intersects_double {
        ($a: expr, $b: expr, $e1: expr, $e2: expr $(,)?) => {
            assert_eq!(
                *Wrange::<u8>::intersection(&$a, &$b).normalized().inner(),
                vec![$e1, $e2]
            );
            assert_eq!(
                *Wrange::<u8>::intersection(&$b, &$a).normalized().inner(),
                vec![$e1, $e2]
            );
        };
    }

    #[test]
    fn test_normalization() {
        use Bound::*;
        use Wrange::*;

        assert_eq!(
            Convergent(Bounds(Exclusive(0), Exclusive(0))).normalized(),
            Empty,
        );

        assert_eq!(
            Convergent(Bounds(Inclusive(0), Exclusive(0))).normalized(),
            Convergent(Bounds(Inclusive(0), Inclusive(0))),
        );

        assert_eq!(
            Divergent(Bounds(Exclusive(0), Exclusive(0))).normalized(),
            Divergent(Bounds(Exclusive(0), Exclusive(0))),
        );

        assert_eq!(
            Divergent(Bounds(Inclusive(0), Exclusive(0))).normalized(),
            Full,
        );
    }

    #[test]
    fn test_intersections_full_empty() {
        use Wrange::*;

        assert_intersects_single!(Full, Full, Full);
        assert_intersects_single!(Full, Empty, Empty);
        assert_intersects_single!(Empty, Full, Empty);
        assert_intersects_single!(Empty, Empty, Empty);
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

        assert_intersects_single!(
            gfx("       x----o  "),
            gfx("  o----x       "),
            gfx("               "),
        );
    }

    #[test]
    fn test_intersections_divergent_divergent() {
        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("-----o   o------"),
            gfx("---o        o---"),
        );

        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("-o       o------"),
            gfx("-o          o---"),
        );

        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("-------o       o"),
            gfx("---o           o"),
        );

        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("o              o"),
            gfx("o              o"),
        );

        assert_intersects_single!(
            gfx("---o        o---"),
            gfx("o              o"),
            gfx("o              o"),
        );

        assert_intersects_single!(
            gfx("x              o"),
            gfx("o              x"),
            gfx("o              o"),
        );

        assert_intersects_single!(
            gfx("x              o"),
            gfx("o              x"),
            gfx("o              o"),
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
    fn test_intersections_with_overlapping_endpoints() {
        assert_intersects_single!(gfx(" x "), gfx(" o "), gfx(" o "),);
        assert_intersects_single!(gfx(" x "), gfx(" x "), gfx("   "),);
        assert_intersects_single!(gfx("---"), gfx(" x "), gfx("   "),);
    }
}
