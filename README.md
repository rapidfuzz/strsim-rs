# strsim-rs [![Build Status](https://travis-ci.org/dguo/strsim-rs.svg?branch=master)](https://travis-ci.org/dguo/strsim-rs)

Rust implementations of [string similarity metrics]. Best efforts will be made to stay up-to-date with Rust nightly. Currently includes:
  - [Hamming]
  - [Levenshtein]
  - [Jaro and Jaro-Winkler] - this implementation does not limit the common prefix length

### Installation

```toml
# Cargo.toml
[dependencies]
strsim = "0.1.0"
```

### Usage

```rust
extern crate strsim;

use strsim::{hamming, levenshtein, jaro, jaro_winkler};

fn main() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why)
    }
    
    assert_eq!(3, levenshtein("kitten", "sitting"));

    assert!(0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre") < 0.001);
    
    assert!(0.911 - jaro_winkler("cheeseburger", "cheese fries") < 0.001);
}
```

### Todo's

 - Implement [Damerau-Levenshtein]
 
### Version

0.1.0

### License

MIT

[string similarity metrics]:http://en.wikipedia.org/wiki/String_metric
[Damerau-Levenshtein]:http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
[Jaro and Jaro-Winkler]:http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
[Levenshtein]:http://en.wikipedia.org/wiki/Levenshtein_distance
[Hamming]:http://en.wikipedia.org/wiki/Hamming_distance
