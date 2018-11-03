extern crate strsim;

use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,damerau_levenshtein,
             normalized_damerau_levenshtein, jaro, jaro_winkler, StrSimError};

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
fn jaro_works() {
    assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
            0.001);
}

#[test]
fn jaro_winkler_works() {
    assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
            0.001);
}

#[test]
fn hamming_empty() {
    match hamming("", "") {
        Ok(distance) => { assert_eq!(0, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn hamming_same() {
    match hamming("hamming", "hamming") {
        Ok(distance) => { assert_eq!(0, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn hamming_diff() {
    match hamming("hamming", "hammers") {
        Ok(distance) => { assert_eq!(3, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn hamming_diff_multibyte() {
    match hamming("hamming", "h香mmüng") {
        Ok(distance) => { assert_eq!(2, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn hamming_unequal_length() {
    match hamming("ham", "hamming") {
        Ok(_) => { panic!(); },
        Err(why) => { assert_eq!(why, StrSimError::DifferentLengthArgs); }
    }
}

#[test]
fn hamming_names() {
    match hamming("Friedrich Nietzs", "Jean-Paul Sartre") {
        Ok(distance) => { assert_eq!(14, distance); },
        Err(why) => { panic!("{:?}", why); }
    }
}

#[test]
fn jaro_both_empty() {
   assert_eq!(1.0, jaro("", ""));
}

#[test]
fn jaro_first_empty() {
    assert_eq!(0.0, jaro("", "jaro"));
}

#[test]
fn jaro_second_empty() {
    assert_eq!(0.0, jaro("distance", ""));
}

#[test]
fn jaro_same() {
    assert_eq!(1.0, jaro("jaro", "jaro"));
}

#[test]
fn jaro_multibyte() {
    assert!((0.818 - jaro("testabctest", "testöঙ香test")).abs() < 0.001);
    assert!((0.818 - jaro("testöঙ香test", "testabctest")).abs() < 0.001);
}

#[test]
fn jaro_diff_short() {
    assert!((0.767 - jaro("dixon", "dicksonx")).abs() < 0.001);
}

#[test]
fn jaro_diff_one_character() {
    assert_eq!(0.0, jaro("a", "b"));
}

#[test]
fn jaro_diff_one_and_two() {
    assert!((0.83 - jaro("a", "ab")).abs() < 0.01);
}

#[test]
fn jaro_diff_two_and_one() {
    assert!((0.83 - jaro("ab", "a")).abs() < 0.01);
}

#[test]
fn jaro_diff_no_transposition() {
    assert!((0.822 - jaro("dwayne", "duane")).abs() < 0.001);
}

#[test]
fn jaro_diff_with_transposition() {
    assert!((0.944 - jaro("martha", "marhta")).abs() < 0.001);
}

#[test]
fn jaro_names() {
    assert!((0.392 - jaro("Friedrich Nietzsche",
                          "Jean-Paul Sartre")).abs() < 0.001);
}

#[test]
fn jaro_winkler_both_empty() {
    assert_eq!(1.0, jaro_winkler("", ""));
}

#[test]
fn jaro_winkler_first_empty() {
    assert_eq!(0.0, jaro_winkler("", "jaro-winkler"));
}

#[test]
fn jaro_winkler_second_empty() {
    assert_eq!(0.0, jaro_winkler("distance", ""));
}

#[test]
fn jaro_winkler_same() {
    assert_eq!(1.0, jaro_winkler("Jaro-Winkler", "Jaro-Winkler"));
}

#[test]
fn jaro_winkler_multibyte() {
    assert!((0.89 - jaro_winkler("testabctest", "testöঙ香test")).abs() <
            0.001);
    assert!((0.89 - jaro_winkler("testöঙ香test", "testabctest")).abs() <
            0.001);
}

#[test]
fn jaro_winkler_diff_short() {
    assert!((0.813 - jaro_winkler("dixon", "dicksonx")).abs() < 0.001);
    assert!((0.813 - jaro_winkler("dicksonx", "dixon")).abs() < 0.001);
}

#[test]
fn jaro_winkler_diff_one_character() {
    assert_eq!(0.0, jaro_winkler("a", "b"));
}

#[test]
fn jaro_winkler_diff_no_transposition() {
    assert!((0.840 - jaro_winkler("dwayne", "duane")).abs() < 0.001);
}

#[test]
fn jaro_winkler_diff_with_transposition() {
    assert!((0.961 - jaro_winkler("martha", "marhta")).abs() < 0.001);
}

#[test]
fn jaro_winkler_names() {
    assert!((0.562 - jaro_winkler("Friedrich Nietzsche",
                                  "Fran-Paul Sartre")).abs() < 0.001);
}

#[test]
fn jaro_winkler_long_prefix() {
    assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
            0.001);
}

#[test]
fn jaro_winkler_more_names() {
    assert!((0.868 - jaro_winkler("Thorkel", "Thorgier")).abs() < 0.001);
}

#[test]
fn jaro_winkler_length_of_one() {
    assert!((0.738 - jaro_winkler("Dinsdale", "D")).abs() < 0.001);
}

#[test]
fn jaro_winkler_very_long_prefix() {
    assert!((1.0 - jaro_winkler("thequickbrownfoxjumpedoverx",
                                "thequickbrownfoxjumpedovery")).abs() <
            0.001);
}

#[test]
fn levenshtein_empty() {
    assert_eq!(0, levenshtein("", ""));
}

#[test]
fn levenshtein_same() {
    assert_eq!(0, levenshtein("levenshtein", "levenshtein"));
}

#[test]
fn levenshtein_diff_short() {
    assert_eq!(3, levenshtein("kitten", "sitting"));
}

#[test]
fn levenshtein_diff_with_space() {
    assert_eq!(5, levenshtein("hello, world", "bye, world"));
}

#[test]
fn levenshtein_diff_multibyte() {
    assert_eq!(3, levenshtein("öঙ香", "abc"));
    assert_eq!(3, levenshtein("abc", "öঙ香"));
}

#[test]
fn levenshtein_diff_longer() {
    let a = "The quick brown fox jumped over the angry dog.";
    let b = "Lorem ipsum dolor sit amet, dicta latine an eam.";
    assert_eq!(37, levenshtein(a, b));
}

#[test]
fn levenshtein_first_empty() {
    assert_eq!(7, levenshtein("", "sitting"));
}

#[test]
fn levenshtein_second_empty() {
    assert_eq!(6, levenshtein("kitten", ""));
}

#[test]
fn normalized_levenshtein_diff_short() {
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
fn osa_distance_empty() {
    assert_eq!(0, osa_distance("", ""));
}

#[test]
fn osa_distance_same() {
    assert_eq!(0, osa_distance("damerau", "damerau"));
}

#[test]
fn osa_distance_first_empty() {
    assert_eq!(7, osa_distance("", "damerau"));
}

#[test]
fn osa_distance_second_empty() {
    assert_eq!(7, osa_distance("damerau", ""));
}

#[test]
fn osa_distance_diff() {
    assert_eq!(3, osa_distance("ca", "abc"));
}

#[test]
fn osa_distance_diff_short() {
    assert_eq!(3, osa_distance("damerau", "aderua"));
}

#[test]
fn osa_distance_diff_reversed() {
    assert_eq!(3, osa_distance("aderua", "damerau"));
}

#[test]
fn osa_distance_diff_multibyte() {
    assert_eq!(3, osa_distance("öঙ香", "abc"));
    assert_eq!(3, osa_distance("abc", "öঙ香"));
}

#[test]
fn osa_distance_diff_unequal_length() {
    assert_eq!(6, osa_distance("damerau", "aderuaxyz"));
}

#[test]
fn osa_distance_diff_unequal_length_reversed() {
    assert_eq!(6, osa_distance("aderuaxyz", "damerau"));
}

#[test]
fn osa_distance_diff_comedians() {
    assert_eq!(5, osa_distance("Stewart", "Colbert"));
}

#[test]
fn osa_distance_many_transpositions() {
    assert_eq!(4, osa_distance("abcdefghijkl", "bacedfgihjlk"));
}

#[test]
fn osa_distance_diff_longer() {
    let a = "The quick brown fox jumped over the angry dog.";
    let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
    assert_eq!(36, osa_distance(a, b));
}

#[test]
fn osa_distance_beginning_transposition() {
    assert_eq!(1, osa_distance("foobar", "ofobar"));
}

#[test]
fn osa_distance_end_transposition() {
    assert_eq!(1, osa_distance("specter", "spectre"));
}

#[test]
fn osa_distance_restricted_edit() {
    assert_eq!(4, osa_distance("a cat", "an abct"));
}

#[test]
fn damerau_levenshtein_empty() {
    assert_eq!(0, damerau_levenshtein("", ""));
}

#[test]
fn damerau_levenshtein_same() {
    assert_eq!(0, damerau_levenshtein("damerau", "damerau"));
}

#[test]
fn damerau_levenshtein_first_empty() {
    assert_eq!(7, damerau_levenshtein("", "damerau"));
}

#[test]
fn damerau_levenshtein_second_empty() {
    assert_eq!(7, damerau_levenshtein("damerau", ""));
}

#[test]
fn damerau_levenshtein_diff() {
    assert_eq!(2, damerau_levenshtein("ca", "abc"));
}

#[test]
fn damerau_levenshtein_diff_short() {
    assert_eq!(3, damerau_levenshtein("damerau", "aderua"));
}

#[test]
fn damerau_levenshtein_diff_reversed() {
    assert_eq!(3, damerau_levenshtein("aderua", "damerau"));
}

#[test]
fn damerau_levenshtein_diff_multibyte() {
    assert_eq!(3, damerau_levenshtein("öঙ香", "abc"));
    assert_eq!(3, damerau_levenshtein("abc", "öঙ香"));
}

#[test]
fn damerau_levenshtein_diff_unequal_length() {
    assert_eq!(6, damerau_levenshtein("damerau", "aderuaxyz"));
}

#[test]
fn damerau_levenshtein_diff_unequal_length_reversed() {
    assert_eq!(6, damerau_levenshtein("aderuaxyz", "damerau"));
}

#[test]
fn damerau_levenshtein_diff_comedians() {
    assert_eq!(5, damerau_levenshtein("Stewart", "Colbert"));
}

#[test]
fn damerau_levenshtein_many_transpositions() {
    assert_eq!(4, damerau_levenshtein("abcdefghijkl", "bacedfgihjlk"));
}

#[test]
fn damerau_levenshtein_diff_longer() {
    let a = "The quick brown fox jumped over the angry dog.";
    let b = "Lehem ipsum dolor sit amet, dicta latine an eam.";
    assert_eq!(36, damerau_levenshtein(a, b));
}

#[test]
fn damerau_levenshtein_beginning_transposition() {
    assert_eq!(1, damerau_levenshtein("foobar", "ofobar"));
}

#[test]
fn damerau_levenshtein_end_transposition() {
    assert_eq!(1, damerau_levenshtein("specter", "spectre"));
}

#[test]
fn damerau_levenshtein_unrestricted_edit() {
    assert_eq!(3, damerau_levenshtein("a cat", "an abct"));
}

#[test]
fn normalized_damerau_levenshtein_diff_short() {
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
