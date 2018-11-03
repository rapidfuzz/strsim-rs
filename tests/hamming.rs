// Copyright 2015 Danny Guo
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

extern crate strsim;

use strsim::{hamming, StrSimError};

#[test]
fn empty() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn same() {
    assert_eq!(Ok(0), hamming("hamming", "hamming"));
}

#[test]
fn diff() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn diff_multibyte() {
    assert_eq!(Ok(2), hamming("hamming", "h香mmüng"));
}

#[test]
fn unequal_length() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("ham", "hamming"));
}

#[test]
fn names() {
    assert_eq!(Ok(14), hamming("Friedrich Nietzs", "Jean-Paul Sartre"));
}
