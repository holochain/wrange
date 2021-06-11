//! Allows easy construction of small ranges via ASCII art, useful for testing

use crate::*;

fn char_to_bound((i, s): (usize, &str)) -> Bound<u8> {
    match s {
        "x" => Bound::Exclusive(i as u8),
        "o" => Bound::Inclusive(i as u8),
        _ => panic!("Invalid character: {}", s),
    }
}

pub fn gfx(s: &str) -> Wrange<u8> {
    let s = s.to_lowercase();
    let pat_bound = ['o', 'x'];
    let mut endpoints = s.match_indices(&pat_bound[..]);
    let p0 = endpoints.next();
    let p1 = endpoints.next();
    match (p0, p1) {
        (None, None) => {
            if s.chars().all(|c| c == ' ') {
                Wrange::Empty
            } else if s.chars().all(|c| c == '-') {
                Wrange::Full
            } else {
                panic!("Malformed graphical representation: |{}|", s);
            }
        }
        (Some(p0), None) => {
            let bound = char_to_bound(p0);
            Wrange::new(bound.clone(), bound)
        }
        (Some(p0), Some(p1)) => {
            assert!(endpoints.next().is_none(), "must be at most two endpoints");

            let middle = s
                .chars()
                .skip_while(|x| !pat_bound.contains(x)) // scan to first marker
                .skip(1) // skip the marker
                .take_while(|x| !pat_bound.contains(x));

            let outside = s
                .chars()
                .take_while(|x| !pat_bound.contains(x)) // take the left side
                .chain(s.chars().rev().take_while(|x| !pat_bound.contains(x))); // take the right side

            if middle.clone().all(|x| x == '-') && outside.clone().all(|x| x == ' ') {
                Wrange::new(char_to_bound(p0), char_to_bound(p1))
            } else if middle.clone().all(|x| x == ' ') && outside.clone().all(|x| x == '-') {
                Wrange::new(char_to_bound(p1), char_to_bound(p0))
            } else {
                panic!("Malformed graphical representation: |{}|", s);
            }
        }
        (None, Some(_)) => unreachable!(),
    }
}

fn inclusion(full: &str, s: &str) -> bool {
    dbg!(s);
    if s.chars().all(|c| c == ' ') {
        false
    } else if s.chars().all(|c| c == '-') {
        true
    } else {
        panic!("Malformed graphical representation: |{}|", full);
    }
}

pub fn gfx2(s: &str) -> WrangeSet<u8> {
    let s = s.to_lowercase();
    let pat_bound = ['o', 'x'];
    let mut endpoints = s.match_indices(&pat_bound[..]);
    let (last, bounds) = endpoints.fold((0, vec![]), |(last, mut v), (i, c)| {
        v.push((i, c, inclusion(&s, &s[last + 1..i])));
        (i, v)
    });
    dbg!(last, bounds);
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn multi() {
        assert_eq!(gfx2("--x---x-"), vec![].into());
        assert_eq!(gfx2("-------x"), vec![].into());
        assert_eq!(gfx2("x-------"), vec![].into());

        // These are all equivalent to Full, but still valid
        // TODO: more intelligent equality checks
    }

    fn checks() {
        assert_eq!(gfx("        "), Wrange::Empty);
        assert_eq!(gfx("--------"), Wrange::Full);

        assert_eq!(gfx("  o     "), Wrange::new_inclusive(2, 2));
        assert_eq!(gfx("   x    "), Wrange::new_exclusive(3, 3));
        assert_eq!(gfx("o       "), Wrange::new_inclusive(0, 0));
        assert_eq!(gfx("       o"), Wrange::new_inclusive(7, 7));

        assert_eq!(gfx("-----x--"), Wrange::new_exclusive(5, 5));
        assert_eq!(gfx("-------x"), Wrange::new_exclusive(7, 7));
        assert_eq!(gfx("x-------"), Wrange::new_exclusive(0, 0));

        // These are all equivalent to Full, but still valid
        // TODO: more intelligent equality checks
        assert_eq!(gfx("o-------"), Wrange::new_inclusive(0, 0));
        assert_eq!(gfx("-------o"), Wrange::new_inclusive(7, 7));
        assert_eq!(gfx("o------o"), Wrange::new_inclusive(0, 7));

        assert_eq!(gfx("  o----o"), Wrange::new_inclusive(2, 7));
        assert_eq!(gfx("o----o  "), Wrange::new_inclusive(0, 5));
        assert_eq!(gfx("oo      "), Wrange::new_inclusive(0, 1));
        assert_eq!(gfx("      oo"), Wrange::new_inclusive(6, 7));

        assert_eq!(gfx("-o     o"), Wrange::new_inclusive(7, 1));
        assert_eq!(gfx("o     o-"), Wrange::new_inclusive(6, 0));
        assert_eq!(gfx("o      o"), Wrange::new_inclusive(7, 0));
        assert_eq!(gfx("x      x"), Wrange::new_exclusive(7, 0));
    }
}
