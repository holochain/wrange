use crate::*;

const LEN: usize = 8;

fn char_to_bound((i, s): (usize, &str)) -> Bound<u8> {
    match s {
        "x" | "X" => Bound::Exclusive(i as u8),
        "o" | "O" => Bound::Inclusive(i as u8),
        _ => panic!("Invalid character: {}", s),
    }
}

pub fn gfx(s: &str) -> Wrange<u8> {
    assert_eq!(s.len(), LEN);
    let pat = ['o', 'O', 'x', 'X'];
    let mut endpoints = s.match_indices(&pat[..]);
    let p0 = endpoints.next().unwrap();
    match endpoints.next() {
        None => {
            let bound = char_to_bound(p0);
            Wrange::new(bound.clone(), bound)
        }
        Some(p1) => {
            assert!(endpoints.next().is_none(), "must be at most two endpoints");

            let middle = s
                .chars()
                .skip_while(|x| !pat.contains(x)) // scan to first marker
                .skip(1) // skip the marker
                .take_while(|x| !pat.contains(x));

            let outside = s
                .chars()
                .take_while(|x| !pat.contains(x)) // take the left side
                .chain(s.chars().rev().take_while(|x| !pat.contains(x))); // take the right side

            if middle.clone().all(|x| x == '-') && outside.clone().all(|x| x == ' ') {
                Wrange::Convergent(char_to_bound(p0), char_to_bound(p1))
            } else if middle.clone().all(|x| x == ' ') && outside.clone().all(|x| x == '-') {
                Wrange::Divergent(char_to_bound(p1), char_to_bound(p0))
            } else {
                panic!("Malformed graphical representation: |{}|", s);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn checks() {
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
