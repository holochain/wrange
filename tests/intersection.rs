use wrange::gfx::gfx;
use wrange::Wrange;

macro_rules! assert_intersection_single {
        ($a: expr, $b: expr, $e: expr $(,)?) => {
            assert_eq!(Wrange::<u8>::intersection(&$a, &$b).normalized().to_vec()[0], $e);
            assert_eq!(Wrange::<u8>::intersection(&$b, &$a).normalized().to_vec()[0], $e);
        };
    }

macro_rules! assert_intersection_double {
    ($a: expr, $b: expr, $e1: expr, $e2: expr $(,)?) => {
        assert_eq!(
            *Wrange::<u8>::intersection(&$a, &$b).normalized().to_vec(),
            vec![$e1, $e2]
        );
        assert_eq!(
            *Wrange::<u8>::intersection(&$b, &$a).normalized().to_vec(),
            vec![$e1, $e2]
        );
    };
}

#[test]
fn test_intersection_full_empty() {
    use Wrange::*;

    assert_intersection_single!(Full, Full, Full);
    assert_intersection_single!(Full, Empty, Empty);
    assert_intersection_single!(Empty, Full, Empty);
    assert_intersection_single!(Empty, Empty, Empty);
}

#[test]
fn test_intersection_convergent_convergent() {
    assert_intersection_single!(
        gfx("  o---------o   "),
        gfx("     o----o     "),
        gfx("     o----o     "),
    );

    assert_intersection_single!(
        gfx("  o-----o       "),
        gfx("     o----o     "),
        gfx("     o--o       "),
    );

    assert_intersection_single!(
        gfx("     o----o     "),
        gfx("  o-----o       "),
        gfx("     o--o       "),
    );

    assert_intersection_single!(
        gfx("  o----o       "),
        gfx("          o--o "),
        gfx("               "),
    );

    assert_intersection_single!(
        gfx("          o--o "),
        gfx("  o----o       "),
        gfx("               "),
    );

    assert_intersection_single!(
        gfx("  o----o       "),
        gfx("       o----o  "),
        gfx("       o       "),
    );

    assert_intersection_single!(
        gfx("  o----o       "),
        gfx("       x----o  "),
        gfx("               "),
    );

    assert_intersection_single!(
        gfx("  o----x       "),
        gfx("       x----o  "),
        gfx("               "),
    );
}

#[test]
fn test_intersection_divergent_divergent() {
    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("-----o   o------"),
        gfx("---o        o---"),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("-x       o------"),
        gfx("-x          o---"),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("-------o       o"),
        gfx("---o           o"),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("o              o"),
        gfx("o              o"),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("o              o"),
        gfx("o              o"),
    );

    assert_intersection_double!(
        gfx("----o    o------"),
        gfx("-----------o o--"),
        gfx("         o-o    "),
        gfx("----o        o--"),
    );

    assert_intersection_double!(
        gfx("------o    o----"),
        gfx("--o o-----------"),
        gfx("    o-o         "),
        gfx("--o        o----"),
    );

    assert_intersection_single!(
        gfx("----x    o------"),
        gfx("---------x o----"),
        gfx("----x      o----"),
    );

    assert_intersection_double!(
        gfx("----x    o------"),
        gfx("---------o o----"),
        gfx("         o      "),
        gfx("----x      o----"),
    );

    assert_intersection_single!(
        gfx("------o    o----"),
        gfx("--o   x---------"),
        gfx("--o        o----"),
    );

    assert_intersection_double!(
        gfx("------o    o----"),
        gfx("--o   o---------"),
        gfx("      o         "),
        gfx("--o        o----"),
    );

    assert_intersection_single!(
        gfx("x              o"),
        gfx("o              x"),
        gfx("x              x"),
    );

    assert_intersection_single!(
        gfx("x              o"),
        gfx("o              o"),
        gfx("x              o"),
    );

    assert_intersection_single!(
        gfx("-----------x    o"),
        gfx("o          o-----"),
        gfx("o               o"),
    );

    assert_intersection_double!(
        gfx("-----------o    o"),
        gfx("o          o-----"),
        gfx("           o     "),
        gfx("o               o"),
    );
}

#[test]
fn test_intersection_divergent_convergent() {
    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("     o----o     "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---x        x---"),
        gfx("     o----o     "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx(" o-----o        "),
        gfx(" o-o            "),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("        o-----o "),
        gfx("            o-o "),
    );

    assert_intersection_double!(
        gfx("----o      o----"),
        gfx(" o------------o "),
        gfx(" o--o           "),
        gfx("           o--o "),
    );

    assert_intersection_single!(
        gfx("---x x----------"),
        gfx("    o           "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("   x            "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("            x   "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---o        o---"),
        gfx("   x--------x   "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---x        x---"),
        gfx("   o--------o   "),
        gfx("                "),
    );

    assert_intersection_single!(
        gfx("---x        o---"),
        gfx("   o--------x   "),
        gfx("                "),
    );
}

#[test]
fn test_intersection_with_overlapping_endpoints() {
    assert_intersection_single!(gfx(" x "), gfx(" o "), gfx("   "),);
    assert_intersection_single!(gfx(" x "), gfx(" x "), gfx("   "),);
    assert_intersection_single!(gfx("---"), gfx(" x "), gfx("   "),);
    assert_intersection_single!(gfx("---"), gfx(" o "), gfx(" o "),);
    assert_intersection_single!(gfx("-x-"), gfx(" o "), gfx("   "),);
    assert_intersection_single!(gfx("-o-"), gfx(" o "), gfx(" o "),);
    assert_intersection_single!(gfx("-o-"), gfx(" x "), gfx("   "),);
}
