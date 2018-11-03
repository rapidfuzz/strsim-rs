extern crate strsim;

use strsim::jaro_winkler;

#[test]
fn both_empty() {
    assert_eq!(1.0, jaro_winkler("", ""));
}

#[test]
fn first_empty() {
    assert_eq!(0.0, jaro_winkler("", "jaro-winkler"));
}

#[test]
fn second_empty() {
    assert_eq!(0.0, jaro_winkler("distance", ""));
}

#[test]
fn same() {
    assert_eq!(1.0, jaro_winkler("Jaro-Winkler", "Jaro-Winkler"));
}

#[test]
fn multibyte() {
    assert!((0.89 - jaro_winkler("testabctest", "testöঙ香test")).abs() <
            0.001);
    assert!((0.89 - jaro_winkler("testöঙ香test", "testabctest")).abs() <
            0.001);
}

#[test]
fn diff_short() {
    assert!((0.813 - jaro_winkler("dixon", "dicksonx")).abs() < 0.001);
    assert!((0.813 - jaro_winkler("dicksonx", "dixon")).abs() < 0.001);
}

#[test]
fn diff_one_character() {
    assert_eq!(0.0, jaro_winkler("a", "b"));
}

#[test]
fn diff_no_transposition() {
    assert!((0.840 - jaro_winkler("dwayne", "duane")).abs() < 0.001);
}

#[test]
fn diff_with_transposition() {
    assert!((0.961 - jaro_winkler("martha", "marhta")).abs() < 0.001);
}

#[test]
fn names() {
    assert!((0.562 - jaro_winkler("Friedrich Nietzsche",
                                  "Fran-Paul Sartre")).abs() < 0.001);
}

#[test]
fn long_prefix() {
    assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
            0.001);
}

#[test]
fn more_names() {
    assert!((0.868 - jaro_winkler("Thorkel", "Thorgier")).abs() < 0.001);
}

#[test]
fn length_of_one() {
    assert!((0.738 - jaro_winkler("Dinsdale", "D")).abs() < 0.001);
}

#[test]
fn very_long_prefix() {
    assert!((1.0 - jaro_winkler("thequickbrownfoxjumpedoverx",
                                "thequickbrownfoxjumpedovery")).abs() <
            0.001);
}
