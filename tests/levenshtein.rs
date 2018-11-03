// Copyright 2015 Danny Guo
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

extern crate strsim;

mod standard {
    use strsim::levenshtein;

    #[test]
    fn empty() {
        assert_eq!(0, levenshtein("", ""));
    }

    #[test]
    fn same() {
        assert_eq!(0, levenshtein("levenshtein", "levenshtein"));
    }

    #[test]
    fn diff_short() {
        assert_eq!(3, levenshtein("kitten", "sitting"));
    }

    #[test]
    fn diff_with_space() {
        assert_eq!(5, levenshtein("hello, world", "bye, world"));
    }

    #[test]
    fn diff_multibyte() {
        assert_eq!(3, levenshtein("öঙ香", "abc"));
        assert_eq!(3, levenshtein("abc", "öঙ香"));
    }

    #[test]
    fn diff_longer() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lorem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(37, levenshtein(a, b));
    }

    #[test]
    fn first_empty() {
        assert_eq!(7, levenshtein("", "sitting"));
    }

    #[test]
    fn second_empty() {
        assert_eq!(6, levenshtein("kitten", ""));
    }
}

mod normalized {
    use strsim::normalized_levenshtein;

    #[test]
    fn diff_short() {
        assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
    }

    #[test]
    fn for_empty_strings() {
        assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
    }

    #[test]
    fn first_empty() {
        assert!(normalized_levenshtein("", "second").abs() < 0.00001);
    }

    #[test]
    fn second_empty() {
        assert!(normalized_levenshtein("first", "").abs() < 0.00001);
    }

    #[test]
    fn identical_strings() {
        assert!((normalized_levenshtein("identical", "identical") - 1.0).abs() < 0.00001);
    }
}
