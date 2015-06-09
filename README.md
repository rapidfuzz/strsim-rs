# strsim-rs [![Build Status](https://travis-ci.org/dguo/strsim-rs.svg?branch=master)](https://travis-ci.org/dguo/strsim-rs)

Rust implementations of [string similarity metrics]. Should compile cleanly on both the nightly and beta versions of Rust. Includes:
  - [Hamming]
  - [Levenshtein] and [Damerau-Levenshtein]
  - [Jaro and Jaro-Winkler] - this implementation of Jaro-Winkler does not limit the common prefix length

### Installation

```toml
# Cargo.toml
[dependencies]
strsim = "0.4.0"
```

### Usage

```rust
extern crate strsim;

use strsim::{hamming, levenshtein, damerau_levenshtein, jaro, jaro_winkler, levenshtein_against_vec, damerau_levenshtein_against_vec, jaro_against_vec, jaro_winkler_against_vec};

fn main() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why)
    }

    assert_eq!(3, levenshtein("kitten", "sitting"));

    assert_eq!(1, damerau_levenshtein("specter", "spectre"));

    assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
            0.001);

    assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
            0.001);

    //
    // Vector to calculate distances against
    let v = vec!["test","test1","test12","test123","","tset"];

    // levenshtein
    let mut distances = levenshtein_against_vec("test",&v);
    let mut expected = vec![0,1,2,3,4,2];
    assert_eq!(distances,expected);

    // damereau_levenshtein
    distances = damerau_levenshtein_against_vec("test",&v);
    expected = vec![0,1,2,3,4,1];
    assert_eq!(distances,expected);

    // jaro
    distances = jaro_against_vec("test",&v);
    expected = vec![1.0, 0.933333, 0.888889, 0.857143, 0.0, 0.916667];
    let mut delta: f64 = res.iter().zip(expected.iter()).map(|(x,y)| (x-y).abs() as f64 ).fold(0.0, |x,y| x+y as f64);
    assert(true, (delta < 0.0001) ); 

    // jaro_winkler
    distances = jaro_winkler_against_vec("test",&v);
    expected = vec![1.0, 0.96, 0.933333, 0.914286, 0.0, 0.925];
    delta = res.iter().zip(expected.iter()).map(|(x,y)| (x-y).abs() as f64 ).fold(0.0, |x,y| x+y as f64);
    assert(true, (delta < 0.0001) ); 

}
```

### License

MIT

[string similarity metrics]:http://en.wikipedia.org/wiki/String_metric
[Damerau-Levenshtein]:http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
[Jaro and Jaro-Winkler]:http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
[Levenshtein]:http://en.wikipedia.org/wiki/Levenshtein_distance
[Hamming]:http://en.wikipedia.org/wiki/Hamming_distance
