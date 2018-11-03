// Copyright 2015 Danny Guo
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

extern crate strsim;

mod shared;

use strsim::jaro;
use shared::assert_approx_eq_f64;

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
    assert_approx_eq_f64(0.818, jaro("testabctest", "testöঙ香test"), 0.001);
    assert_approx_eq_f64(0.818, jaro("testöঙ香test", "testabctest"), 0.001);
}

#[test]
fn diff_short() {
    assert_approx_eq_f64(0.767, jaro("dixon", "dicksonx"), 0.001);
}

#[test]
fn diff_one_character() {
    assert_eq!(0.0, jaro("a", "b"));
}

#[test]
fn diff_one_and_two() {
    assert_approx_eq_f64(0.83, jaro("a", "ab"), 0.01);
}

#[test]
fn diff_two_and_one() {
    assert_approx_eq_f64(0.83, jaro("ab", "a"), 0.01);
}

#[test]
fn diff_no_transposition() {
    assert_approx_eq_f64(0.822, jaro("dwayne", "duane"), 0.001);
}

#[test]
fn diff_with_transposition() {
    assert_approx_eq_f64(0.944, jaro("martha", "marhta"), 0.001);
}

#[test]
fn names() {
    assert_approx_eq_f64(0.392, jaro("Friedrich Nietzsche",
                                     "Jean-Paul Sartre"), 0.001);
}
