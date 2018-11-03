extern crate strsim;

use strsim::osa_distance;

#[test]
fn empty() {
    assert_eq!(0, osa_distance("", ""));
}

#[test]
fn same() {
    assert_eq!(0, osa_distance("damerau", "damerau"));
}

#[test]
fn first_empty() {
    assert_eq!(7, osa_distance("", "damerau"));
}

#[test]
fn second_empty() {
    assert_eq!(7, osa_distance("damerau", ""));
}

#[test]
fn diff() {
    assert_eq!(3, osa_distance("ca", "abc"));
}

#[test]
fn diff_short() {
    assert_eq!(3, osa_distance("damerau", "aderua"));
}

#[test]
fn diff_reversed() {
    assert_eq!(3, osa_distance("aderua", "damerau"));
}

#[test]
fn diff_multibyte() {
    assert_eq!(3, osa_distance("öঙ香", "abc"));
    assert_eq!(3, osa_distance("abc", "öঙ香"));
}

#[test]
fn diff_unequal_length() {
    assert_eq!(6, osa_distance("damerau", "aderuaxyz"));
}

#[test]
fn diff_unequal_length_reversed() {
    assert_eq!(6, osa_distance("aderuaxyz", "damerau"));
}

#[test]
fn diff_comedians() {
    assert_eq!(5, osa_distance("Stewart", "Colbert"));
}

#[test]
fn many_transpositions() {
    assert_eq!(4, osa_distance("abcdefghijkl", "bacedfgihjlk"));
}

#[test]
fn diff_longer() {
    let a = "The quick brown fox jumped over the angry dog.";
    let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
    assert_eq!(36, osa_distance(a, b));
}

#[test]
fn beginning_transposition() {
    assert_eq!(1, osa_distance("foobar", "ofobar"));
}

#[test]
fn end_transposition() {
    assert_eq!(1, osa_distance("specter", "spectre"));
}

#[test]
fn restricted_edit() {
    assert_eq!(4, osa_distance("a cat", "an abct"));
}
