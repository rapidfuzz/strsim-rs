extern crate strsim;

use strsim::{hamming, StrSimError};

#[test]
fn empty() {
    match hamming("", "") {
        Ok(distance) => { assert_eq!(0, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn same() {
    match hamming("hamming", "hamming") {
        Ok(distance) => { assert_eq!(0, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn diff() {
    match hamming("hamming", "hammers") {
        Ok(distance) => { assert_eq!(3, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn diff_multibyte() {
    match hamming("hamming", "h香mmüng") {
        Ok(distance) => { assert_eq!(2, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn unequal_length() {
    match hamming("ham", "hamming") {
        Ok(_) => { panic!(); },
        Err(why) => { assert_eq!(why, StrSimError::DifferentLengthArgs); }
    }
}

#[test]
fn names() {
    match hamming("Friedrich Nietzs", "Jean-Paul Sartre") {
        Ok(distance) => { assert_eq!(14, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}
