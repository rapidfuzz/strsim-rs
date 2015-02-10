extern crate strsim;

use strsim::{hamming, levenshtein, jaro, jaro_winkler};

#[test]
fn hamming_works() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why)
    }
}

#[test]
fn levenshtein_works() {
    assert_eq!(3, levenshtein("kitten", "sitting"));
}

#[test]
fn jaro_works() {
    assert!(0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre") < 0.001);
}

#[test]
fn jaro_winkler_works() {
    assert!(0.911 - jaro_winkler("cheeseburger", "cheese fries") < 0.001);
}
