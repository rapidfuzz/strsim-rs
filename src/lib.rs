// Copyright 2015 Danny Guo
// Copyright 2016 Titus Wormer <tituswormer@gmail.com>
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

//! This library implements [string similarity metrics](http://en.wikipedia.org/wiki/String_metric).

use std::char;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum StrSimError {
    DifferentLengthArgs,
}

impl Display for StrSimError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };

        write!(fmt, "{}", text)
    }
}

impl Error for StrSimError {}

pub type HammingResult = Result<usize, StrSimError>;

/// Calculate a “[Hamming](http://en.wikipedia.org/wiki/Hamming_distance)” metric.
///
/// Calculates the number of positions in the two strings where the characters
/// differ. Returns an error if the strings have different char counts.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::hamming;
///
/// assert_eq!(Ok(3), hamming("hamming", "hammers"));
/// ```
pub fn hamming(a: &str, b: &str) -> HammingResult {
    let (mut ita, mut itb, mut count) = (a.chars(), b.chars(), 0);
    loop {
        match (ita.next(), itb.next()) {
            (Some(x), Some(y)) => if x != y { count += 1 },
            (None, None) => return Ok(count),
            _ => return Err(StrSimError::DifferentLengthArgs),
        }
    }
}

/// Calculate a “[Jaro](http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)”
/// metric.
///
/// Calculates the “Jaro” similarity between two strings. The returned value
/// is between `0.0` and `1.0` (higher value means more similar).
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::jaro;
///
/// assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
///         0.001);
/// ```
pub fn jaro(a: &str, b: &str) -> f64 {
    if a == b { return 1.0; }

    let a_len = a.chars().count();
    let b_len = b.chars().count();

    // The check for lengths of one here is to prevent integer overflow when
    // calculating the search range.
    if a_len == 0 || b_len == 0 || (a_len == 1 && b_len == 1) {
        return 0.0;
    }

    let search_range = (max(a_len, b_len) / 2) - 1;

    let mut b_consumed = Vec::with_capacity(b_len);
    for _ in 0..b_len {
        b_consumed.push(false);
    }
    let mut matches = 0.0;

    let mut transpositions = 0.0;
    let mut b_match_index = 0;

    for (i, a_char) in a.chars().enumerate() {
        let min_bound =
            // prevent integer wrapping
            if i > search_range {
                max(0, i - search_range)
            } else {
                0
            };

        let max_bound = min(b_len - 1, i + search_range);

        if min_bound > max_bound {
            continue;
        }

        for (j, b_char) in b.chars().enumerate() {
            if min_bound <= j && j <= max_bound && a_char == b_char &&
               !b_consumed[j] {
                b_consumed[j] = true;
                matches += 1.0;

                if j < b_match_index {
                    transpositions += 1.0;
                }
                b_match_index = j;

                break;
            }
        }
    }

    if matches == 0.0 {
        0.0
    } else {
        (1.0 / 3.0) * ((matches / a_len as f64) +
                       (matches / b_len as f64) +
                       ((matches - transpositions) / matches))
    }
}

/// Calculate a “[Jaro Winkler](http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)”
/// metric.
///
/// Like “Jaro” but gives a boost to strings that have a common prefix.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::jaro_winkler;
///
/// assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
///         0.001);
/// ```
pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    let jaro_distance = jaro(a, b);

    // Don't limit the length of the common prefix
    let prefix_length = a.chars()
                         .zip(b.chars())
                         .take_while(|&(a_char, b_char)| a_char == b_char)
                         .count();

    let jaro_winkler_distance =
        jaro_distance + (0.1 * prefix_length as f64 * (1.0 - jaro_distance));

    if jaro_winkler_distance <= 1.0 {
        jaro_winkler_distance
    } else {
        1.0
    }
}

/// Calculate a “[Levenshtein](http://en.wikipedia.org/wiki/Levenshtein_distance)”
/// metric.
///
/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one string into the other.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
    if a == b { return 0; }

    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 { return b_len; }
    if b_len == 0 { return a_len; }

    let mut cache: Vec<usize> = (1..b_len+1).collect();

    let mut result = 0;
    let mut distance_a;
    let mut distance_b;

    for (i, a_char) in a.chars().enumerate() {
        result = i;
        distance_b = i;

        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            distance_a = distance_b + cost;
            distance_b = cache[j];
            result = min(result + 1, min(distance_a, distance_b + 1));
            cache[j] = result;
        }
    }

    result
}

/// Calculate a “normalized [Levenshtein](http://en.wikipedia.org/wiki/Levenshtein_distance)”
/// metric.
///
/// Calculates a normalized score of the “Levenshtein” algorithm between `0.0`
/// and `1.0` (inclusive), where `1.0` means the strings are the same.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::normalized_levenshtein;
///
/// assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
/// assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
/// assert!(normalized_levenshtein("", "second").abs() < 0.00001);
/// assert!(normalized_levenshtein("first", "").abs() < 0.00001);
/// assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
/// ```
pub fn normalized_levenshtein(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64)
}

/// Calculate a “[Optimal string alignment](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Optimal_string_alignment_distance)”
/// metric.
///
/// Like “Levenshtein” but allows for adjacent transpositions. Each substring
/// can only be edited once.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::osa_distance;
///
/// assert_eq!(3, osa_distance("ab", "bca"));
/// ```
pub fn osa_distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();
    if a == b { return 0; }
    else if a_len == 0 { return b_len; }
    else if b_len == 0 { return a_len; }

    let mut prev_two_distances: Vec<usize> = Vec::with_capacity(b_len + 1);
    let mut prev_distances: Vec<usize> = Vec::with_capacity(b_len + 1);
    let mut curr_distances: Vec<usize> = Vec::with_capacity(b_len + 1);

    let mut prev_a_char = char::MAX;
    let mut prev_b_char = char::MAX;

    for i in 0..(b_len + 1) {
        prev_two_distances.push(i);
        prev_distances.push(i);
        curr_distances.push(0);
    }

    for (i, a_char) in a.chars().enumerate() {
        curr_distances[0] = i + 1;

        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            curr_distances[j + 1] = min(curr_distances[j] + 1,
                                        min(prev_distances[j + 1] + 1,
                                            prev_distances[j] + cost));
            if i > 0 && j > 0 && a_char != b_char &&
               a_char == prev_b_char && b_char == prev_a_char {
                curr_distances[j + 1] = min(curr_distances[j + 1],
                                            prev_two_distances[j - 1] + 1);
            }

            prev_b_char = b_char;
        }

        prev_two_distances.clone_from(&prev_distances);
        prev_distances.clone_from(&curr_distances);
        prev_a_char = a_char;
    }

    curr_distances[b_len]
}

/// Calculate a “[Damerau-Levenshtein](http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)”
/// metric.
///
/// Like “optimal string alignment”, but substrings can be edited an unlimited
/// number of times, and the triangle inequality holds.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::damerau_levenshtein;
///
/// assert_eq!(2, damerau_levenshtein("ab", "bca"));
/// ```
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    if a == b { return 0; }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 { return b_len; }
    if b_len == 0 { return a_len; }

    let mut distances = vec![vec![0; b_len + 2]; a_len + 2];
    let max_distance = a_len + b_len;
    distances[0][0] = max_distance;

    for i in 0..(a_len + 1) {
        distances[i + 1][0] = max_distance;
        distances[i + 1][1] = i;
    }

    for j in 0..(b_len + 1) {
        distances[0][j + 1] = max_distance;
        distances[1][j + 1] = j;
    }

    let mut chars: HashMap<char, usize> = HashMap::new();

    for i in 1..(a_len + 1) {
        let mut db = 0;

        for j in 1..(b_len + 1) {
            let k = match chars.get(&b_chars[j - 1]) {
                Some(value) => value.clone(),
                None => 0
            };

            let l = db;

            let mut cost = 1;
            if a_chars[i - 1] == b_chars[j - 1] {
                cost = 0;
                db = j;
            }

            let substitution_cost = distances[i][j] + cost;
            let insertion_cost = distances[i][j + 1] + 1;
            let deletion_cost = distances[i + 1][j] + 1;
            let transposition_cost = distances[k][l] + (i - k - 1) + 1 +
                                     (j - l - 1);

            distances[i + 1][j + 1] = min(substitution_cost,
                                      min(insertion_cost,
                                      min(deletion_cost,
                                          transposition_cost)));
        }

        chars.insert(a_chars[i - 1], i);
    }

    distances[a_len + 1][b_len + 1]
}

/// Calculate a “normalized [Damerau-Levenshtein](http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)”
/// metric.
///
/// Calculates a normalized score of the “Damerau–Levenshtein” algorithm between
/// `0.0` and `1.0` (inclusive), where `1.0` means the strings are the same.
///
/// Note: This implementation is based on unicode “scalar values”, not “grapheme
/// clusters”.
///
/// ```
/// use strsim::normalized_damerau_levenshtein;
///
/// assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
/// assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
/// assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
/// assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
/// assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
/// ```
pub fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    1.0 - (damerau_levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64)
}
