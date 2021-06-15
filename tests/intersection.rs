use wrange::ascii::ascii;
use wrange::{Wrange, WrangeSet};

macro_rules! assert_intersection {
        ($a: expr, $b: expr, $e: expr $(,)?) => {
            assert_eq!(WrangeSet::<u8>::intersection(&$a, &$b).normalized(), $e);
            assert_eq!(WrangeSet::<u8>::intersection(&$b, &$a).normalized(), $e);
        };
    }

macro_rules! assert_intersection_double {
    ($a: expr, $b: expr, $e1: expr, $e2: expr $(,)?) => {
        assert_eq!(
            WrangeSet::<u8>::intersection(&$a, &$b).normalized(),
            vec![$e1.to_vec()[0].clone(), $e2.to_vec()[0].clone()].into()
        );
        assert_eq!(
            WrangeSet::<u8>::intersection(&$b, &$a).normalized(),
            vec![$e1.to_vec()[0].clone(), $e2.to_vec()[0].clone()].into()
        );
    };
}

#[test]
fn test_intersection_full_empty() {
    use Wrange::*;

    assert_intersection!(Full.into(), Full.into(), Full.into());
    assert_intersection!(Full.into(), Empty.into(), Empty.into());
    assert_intersection!(Empty.into(), Full.into(), Empty.into());
    assert_intersection!(Empty.into(), Empty.into(), Empty.into());
}

#[test]
fn test_intersection_convergent_convergent() {
    assert_intersection!(
        ascii("  o---------o   "),
        ascii("     o----o     "),
        ascii("     o----o     "),
    );

    assert_intersection!(
        ascii("  o-----o       "),
        ascii("     o----o     "),
        ascii("     o--o       "),
    );

    assert_intersection!(
        ascii("     o----o     "),
        ascii("  o-----o       "),
        ascii("     o--o       "),
    );

    assert_intersection!(
        ascii("  o----o       "),
        ascii("          o--o "),
        ascii("               "),
    );

    assert_intersection!(
        ascii("          o--o "),
        ascii("  o----o       "),
        ascii("               "),
    );

    assert_intersection!(
        ascii("  o----o       "),
        ascii("       o----o  "),
        ascii("       o       "),
    );

    assert_intersection!(
        ascii("  o----o       "),
        ascii("       x----o  "),
        ascii("               "),
    );

    assert_intersection!(
        ascii("  o----x       "),
        ascii("       x----o  "),
        ascii("               "),
    );
}

#[test]
fn test_intersection_divergent_divergent() {
    assert_intersection!(
        ascii("---o        o---"),
        ascii("-----o   o------"),
        ascii("---o        o---"),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("-x       o------"),
        ascii("-x          o---"),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("-------o       o"),
        ascii("---o           o"),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("o              o"),
        ascii("o              o"),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("o              o"),
        ascii("o              o"),
    );

    assert_intersection_double!(
        ascii("----o    o------"),
        ascii("-----------o o--"),
        ascii("         o-o    "),
        ascii("----o        o--"),
    );

    assert_intersection_double!(
        ascii("------o    o----"),
        ascii("--o o-----------"),
        ascii("    o-o         "),
        ascii("--o        o----"),
    );

    assert_intersection!(
        ascii("----x    o------"),
        ascii("---------x o----"),
        ascii("----x      o----"),
    );

    assert_intersection_double!(
        ascii("----x    o------"),
        ascii("---------o o----"),
        ascii("         o      "),
        ascii("----x      o----"),
    );

    assert_intersection!(
        ascii("------o    o----"),
        ascii("--o   x---------"),
        ascii("--o        o----"),
    );

    assert_intersection_double!(
        ascii("------o    o----"),
        ascii("--o   o---------"),
        ascii("      o         "),
        ascii("--o        o----"),
    );

    assert_intersection!(
        ascii("x              o"),
        ascii("o              x"),
        ascii("x              x"),
    );

    assert_intersection!(
        ascii("x              o"),
        ascii("o              o"),
        ascii("x              o"),
    );

    assert_intersection!(
        ascii("-----------x    o"),
        ascii("o          o-----"),
        ascii("o               o"),
    );

    assert_intersection_double!(
        ascii("-----------o    o"),
        ascii("o          o-----"),
        ascii("           o     "),
        ascii("o               o"),
    );
}

#[test]
fn test_intersection_divergent_convergent() {
    assert_intersection!(
        ascii("---o        o---"),
        ascii("     o----o     "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---x        x---"),
        ascii("     o----o     "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii(" o-----o        "),
        ascii(" o-o            "),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("        o-----o "),
        ascii("            o-o "),
    );

    assert_intersection_double!(
        ascii("----o      o----"),
        ascii(" o------------o "),
        ascii(" o--o           "),
        ascii("           o--o "),
    );

    assert_intersection!(
        ascii("---x x----------"),
        ascii("    o           "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("   x            "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("            x   "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---o        o---"),
        ascii("   x--------x   "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---x        x---"),
        ascii("   o--------o   "),
        ascii("                "),
    );

    assert_intersection!(
        ascii("---x        o---"),
        ascii("   o--------x   "),
        ascii("                "),
    );
}

#[test]
fn test_intersection_with_overlapping_endpoints() {
    assert_intersection!(ascii(" x "), ascii(" o "), ascii("   "),);
    assert_intersection!(ascii(" x "), ascii(" x "), ascii("   "),);
    assert_intersection!(ascii("---"), ascii(" x "), ascii("   "),);
    assert_intersection!(ascii("---"), ascii(" o "), ascii(" o "),);
    assert_intersection!(ascii("-x-"), ascii(" o "), ascii("   "),);
    assert_intersection!(ascii("-o-"), ascii(" o "), ascii(" o "),);
    assert_intersection!(ascii("-o-"), ascii(" x "), ascii("   "),);
}
