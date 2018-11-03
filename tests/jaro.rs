extern crate strsim;

use strsim::jaro;

#[test]
fn both_empty() {
   assert_eq!(1.0, jaro("", ""));
}

#[test]
fn first_empty() {
    assert_eq!(0.0, jaro("", "jaro"));
}

#[test]
fn second_empty() {
    assert_eq!(0.0, jaro("distance", ""));
}

#[test]
fn same() {
    assert_eq!(1.0, jaro("jaro", "jaro"));
}

#[test]
fn multibyte() {
    assert!((0.818 - jaro("testabctest", "testöঙ香test")).abs() < 0.001);
    assert!((0.818 - jaro("testöঙ香test", "testabctest")).abs() < 0.001);
}

#[test]
fn diff_short() {
    assert!((0.767 - jaro("dixon", "dicksonx")).abs() < 0.001);
}

#[test]
fn diff_one_character() {
    assert_eq!(0.0, jaro("a", "b"));
}

#[test]
fn diff_one_and_two() {
    assert!((0.83 - jaro("a", "ab")).abs() < 0.01);
}

#[test]
fn diff_two_and_one() {
    assert!((0.83 - jaro("ab", "a")).abs() < 0.01);
}

#[test]
fn diff_no_transposition() {
    assert!((0.822 - jaro("dwayne", "duane")).abs() < 0.001);
}

#[test]
fn diff_with_transposition() {
    assert!((0.944 - jaro("martha", "marhta")).abs() < 0.001);
}

#[test]
fn names() {
    assert!((0.392 - jaro("Friedrich Nietzsche",
                          "Jean-Paul Sartre")).abs() < 0.001);
}
