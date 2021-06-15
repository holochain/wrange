//! Allows easy construction of small ranges via ASCII art, useful for testing

use crate::*;

use itertools::Itertools;

/// Says whether a substring is all dashes or all spaces. Panics if neither.
fn dashes(full: &str, begin: usize, end: usize) -> bool {
    let s = &full[begin..end];
    if s.chars().all(|c| c == '-') {
        true
    } else if s.chars().all(|c| c == ' ') {
        false
    } else {
        panic!(
            "Malformed ascii. Must have all spaces or all dashes between positions {} and {}: |{}|",
            begin, end, full
        );
    }
}

/// Create a bound from a position and a character
fn bound((t, c): Endpoint) -> Bound<u8> {
    if c == "x" {
        Bound::Exclusive(t as u8)
    } else if c == "o" {
        Bound::Inclusive(t as u8)
    } else {
        unreachable!()
    }
}

type Endpoint<'a> = (usize, &'a str);

pub fn ascii(s: &str) -> WrangeSet<u8> {
    let s = s.to_lowercase();
    let pat_bound = ['o', 'x'];

    let intervals = s
        .match_indices(&pat_bound[..])
        .collect::<Vec<_>>()
        .into_iter()
        // find all N pairs of endpoints, where N is also the number of endpoints
        .circular_tuple_windows()
        // mark each interval as "on" or "off"
        .map(|(e0, e1)| {
            let lo = e0.0;
            let hi = e1.0;
            let on = if lo < hi {
                dashes(&s, lo + 1, hi)
            } else {
                // In this branch, either the endpoints are the same, or they are wrapping.
                // If they are the same, then there is only one endpoint in the string,
                // and then "on" means the entire space is covered,
                // and "off" means there is a single zero-length range.
                let dashes_end = dashes(&s, lo + 1, s.len());
                let dashes_start = dashes(&s, 0, hi);
                if lo == s.len() - 1 {
                    dashes_start
                } else if hi == 0 {
                    dashes_end
                } else if dashes_start && dashes_end {
                    true
                } else if !dashes_start && !dashes_end {
                    false
                } else {
                    panic!("Malformed ascii. The beginning and end of the string must match with respect to dashes and spaces: |{}|", s);
                }
            };
            (e0, e1, on)
        })
        .collect::<Vec<_>>();

    // fold over intervals, creating Wranges out of each one:
    if let Some((_, _, last_on)) = intervals.last().cloned() {
        let (_, wranges) = intervals.into_iter().fold(
            (last_on, vec![]),
            |(last_on, mut wranges), (e0, e1, on)| {
                if !last_on && !on {
                    // if the last interval was "off" and so is this one, then the beginning endpoint is a zero-length Wrange
                    wranges.push(Wrange::Convergent(Bounds(bound(e0), bound(e0))))
                } else if on {
                    // if this interval is "on", always create a Wrange.
                    // Note we have to be explicit about convergent/divergent
                    // in order to handle the case of the endpoints being the same.
                    if e0.0 < e1.0 {
                        wranges.push(Wrange::Convergent(Bounds(bound(e0), bound(e1))))
                    } else {
                        wranges.push(Wrange::Divergent(Bounds(bound(e0), bound(e1))))
                    }
                }
                (on, wranges)
            },
        );

        wranges.into()
    } else {
        if dashes(&s, 0, s.len()) {
            vec![Wrange::Full].into()
        } else {
            vec![Wrange::Empty].into()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn multi() {
        use Bound::*;

        assert_eq!(
            ascii("--x---x-"),
            vec![Wrange::new_exclusive(2, 6), Wrange::new_exclusive(6, 2)].into()
        );
        assert_eq!(
            ascii("-------x"),
            vec![Wrange::Divergent(Bounds(Exclusive(7), Exclusive(7)))].into()
        );
        assert_eq!(
            ascii("x-------"),
            vec![Wrange::Divergent(Bounds(Exclusive(0), Exclusive(0)))].into()
        );
        assert_eq!(
            ascii("x------x"),
            vec![
                Wrange::Convergent(Bounds(Exclusive(0), Exclusive(7))),
                Wrange::Divergent(Bounds(Exclusive(7), Exclusive(0)))
            ]
            .into()
        );
        assert_eq!(
            ascii("       x"),
            vec![Wrange::Convergent(Bounds(Exclusive(7), Exclusive(7)))].into()
        );
        assert_eq!(
            ascii("x       "),
            vec![Wrange::Convergent(Bounds(Exclusive(0), Exclusive(0)))].into()
        );
        assert_eq!(
            ascii("x x x x "),
            vec![
                Wrange::Convergent(Bounds(Exclusive(0), Exclusive(0))),
                Wrange::Convergent(Bounds(Exclusive(2), Exclusive(2))),
                Wrange::Convergent(Bounds(Exclusive(4), Exclusive(4))),
                Wrange::Convergent(Bounds(Exclusive(6), Exclusive(6)))
            ]
            .into()
        );
        assert_eq!(
            ascii("xxx x-x "),
            vec![
                Wrange::Convergent(Bounds(Exclusive(0), Exclusive(1))),
                Wrange::Convergent(Bounds(Exclusive(1), Exclusive(2))),
                Wrange::Convergent(Bounds(Exclusive(4), Exclusive(6))),
            ]
            .into()
        );
        assert_eq!(
            ascii("xxx x-x"),
            vec![
                Wrange::Convergent(Bounds(Exclusive(0), Exclusive(1))),
                Wrange::Convergent(Bounds(Exclusive(1), Exclusive(2))),
                Wrange::Convergent(Bounds(Exclusive(4), Exclusive(6))),
                Wrange::Divergent(Bounds(Exclusive(6), Exclusive(0))),
            ]
            .into()
        );
        assert_eq!(
            ascii("o--xo ox o--"),
            vec![
                Wrange::new(Inclusive(0), Exclusive(3)),
                Wrange::new(Exclusive(3), Inclusive(4)),
                Wrange::new(Inclusive(6), Exclusive(7)),
                Wrange::new(Inclusive(9), Inclusive(0)),
            ]
            .into()
        );

        // These are all equivalent to Full, but still valid
        // TODO: more intelligent equality checks
    }

    #[test]
    fn single_checks() {
        use Bound::*;
        use Wrange::*;
        assert_eq!(ascii("        ").to_vec()[0], Wrange::Empty);
        assert_eq!(ascii("--------").to_vec()[0], Wrange::Full);

        assert_eq!(ascii("  o     ").to_vec()[0], Wrange::new_inclusive(2, 2));
        assert_eq!(ascii("   x    ").to_vec()[0], Wrange::new_exclusive(3, 3));
        assert_eq!(ascii("o       ").to_vec()[0], Wrange::new_inclusive(0, 0));
        assert_eq!(ascii("       o").to_vec()[0], Wrange::new_inclusive(7, 7));

        assert_eq!(
            ascii("-----x--").to_vec()[0],
            Wrange::Divergent(Bounds(Exclusive(5), Exclusive(5)))
        );
        assert_eq!(
            ascii("-------x").to_vec()[0],
            Wrange::Divergent(Bounds(Exclusive(7), Exclusive(7)))
        );
        assert_eq!(
            ascii("x-------").to_vec()[0],
            Wrange::Divergent(Bounds(Exclusive(0), Exclusive(0)))
        );

        assert_eq!(ascii("  o----o").to_vec()[0], Wrange::new_inclusive(2, 7));
        assert_eq!(ascii("o----o  ").to_vec()[0], Wrange::new_inclusive(0, 5));
        assert_eq!(ascii("oo      ").to_vec()[0], Wrange::new_inclusive(0, 1));
        assert_eq!(ascii("      oo").to_vec()[0], Wrange::new_inclusive(6, 7));

        assert_eq!(ascii("-o     o").to_vec()[0], Wrange::new_inclusive(7, 1));
        assert_eq!(ascii("o     o-").to_vec()[0], Wrange::new_inclusive(6, 0));
        assert_eq!(ascii("o      o").to_vec()[0], Wrange::new_inclusive(7, 0));
        assert_eq!(ascii("x      x").to_vec()[0], Wrange::new_exclusive(7, 0));
    }
}
