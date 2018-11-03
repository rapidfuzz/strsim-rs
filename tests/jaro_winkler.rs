// Copyright 2015 Danny Guo
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

extern crate strsim;

mod shared;

use strsim::jaro_winkler;
use shared::assert_approx_eq_f64;

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
    assert_approx_eq_f64(0.89, jaro_winkler("testabctest", "testöঙ香test"),
                         0.001);
    assert_approx_eq_f64(0.89, jaro_winkler("testöঙ香test", "testabctest"),
                         0.001);
}

#[test]
fn diff_short() {
    assert_approx_eq_f64(0.813, jaro_winkler("dixon", "dicksonx"), 0.001);
    assert_approx_eq_f64(0.813, jaro_winkler("dicksonx", "dixon"), 0.001);
}

#[test]
fn diff_one_character() {
    assert_eq!(0.0, jaro_winkler("a", "b"));
}

#[test]
fn diff_no_transposition() {
    assert_approx_eq_f64(0.840, jaro_winkler("dwayne", "duane"), 0.001);
}

#[test]
fn diff_with_transposition() {
    assert_approx_eq_f64(0.961, jaro_winkler("martha", "marhta"), 0.001);
}

#[test]
fn names() {
    assert_approx_eq_f64(0.562, jaro_winkler("Friedrich Nietzsche",
                                             "Fran-Paul Sartre"), 0.001);
}

#[test]
fn long_prefix() {
    assert_approx_eq_f64(0.911, jaro_winkler("cheeseburger", "cheese fries"),
                         0.001);
}

#[test]
fn more_names() {
    assert_approx_eq_f64(0.868, jaro_winkler("Thorkel", "Thorgier"), 0.001);
}

#[test]
fn length_of_one() {
    assert_approx_eq_f64(0.738, jaro_winkler("Dinsdale", "D"), 0.001);
}

#[test]
fn very_long_prefix() {
    assert_approx_eq_f64(1.0, jaro_winkler("thequickbrownfoxjumpedoverx",
                                           "thequickbrownfoxjumpedovery"),
                         0.001);
}
