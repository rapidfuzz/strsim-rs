extern crate strsim;

use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,damerau_levenshtein,
             normalized_damerau_levenshtein, jaro, jaro_winkler};

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
fn normalized_levenshtein_works() {
    assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
}

#[test]
fn normalized_levenshtein_for_empty_strings() {
    assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
}

#[test]
fn normalized_levenshtein_first_empty() {
    assert!(normalized_levenshtein("", "second").abs() < 0.00001);
}

#[test]
fn normalized_levenshtein_second_empty() {
    assert!(normalized_levenshtein("first", "").abs() < 0.00001);
}

#[test]
fn normalized_levenshtein_identical_strings() {
    assert!((normalized_levenshtein("identical", "identical") - 1.0).abs() < 0.00001);
}

#[test]
fn osa_distance_works() {
    assert_eq!(3, osa_distance("ac", "cba"));
}

#[test]
fn damerau_levenshtein_works() {
    assert_eq!(2, damerau_levenshtein("ac", "cba"));
}

#[test]
fn normalized_damerau_levenshtein_works() {
    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
}

#[test]
fn normalized_damerau_levenshtein_for_empty_strings() {
    assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
}

#[test]
fn normalized_damerau_levenshtein_first_empty() {
    assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
}

#[test]
fn normalized_damerau_levenshtein_second_empty() {
    assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
}

#[test]
fn normalized_damerau_levenshtein_identical_strings() {
    assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
}

#[test]
fn jaro_works() {
    assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
            0.001);
}

#[test]
fn jaro_winkler_works() {
    assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
            0.001);
}
