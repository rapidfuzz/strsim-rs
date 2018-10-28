# strsim-rs [![Crates.io](https://img.shields.io/crates/v/strsim.svg)](https://crates.io/crates/strsim) [![Crates.io](https://img.shields.io/crates/l/strsim.svg?maxAge=2592000)](https://github.com/dguo/strsim-rs/blob/master/LICENSE) [![build status](https://travis-ci.org/dguo/strsim-rs.svg?branch=master)](https://travis-ci.org/dguo/strsim-rs)

[Rust](https://www.rust-lang.org) implementations of [string similarity metrics]:
  - [Hamming]
  - [Levenshtein] - distance & normalized
  - [Optimal string alignment]
  - [Damerau-Levenshtein] - distance & normalized
  - [Jaro and Jaro-Winkler] - this implementation of Jaro-Winkler does not limit the common prefix length

The normalized versions return values between `0.0` and `1.0`, where `1.0` means
an exact match.

## Installation

`strsim` is available on [crates.io](https://crates.io/crates/strsim). Add it to
your `Cargo.toml`:
```toml
[dependencies]
strsim = "0.8.0"
```

## Usage

Go to [Docs.rs](https://docs.rs/strsim/) for the full documentation. You can
also clone the repo, and run `$ cargo doc --open`.

### Examples

```rust
extern crate strsim;

use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler};

fn main() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why)
    }

    assert_eq!(levenshtein("kitten", "sitting"), 3);

    assert!((normalized_levenshtein("kitten", "sitting") - 0.571).abs() < 0.001);

    assert_eq!(osa_distance("ac", "cba"), 3);

    assert_eq!(damerau_levenshtein("ac", "cba"), 2);

    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.272).abs() <
            0.001);

    assert!((jaro("Friedrich Nietzsche", "Jean-Paul Sartre") - 0.392).abs() <
            0.001);

    assert!((jaro_winkler("cheeseburger", "cheese fries") - 0.911).abs() <
            0.001);
}
```

## Contributing

If you don't want to install Rust itself, you can run `$ ./dev` for a
development CLI if you have [Docker] installed.

Benchmarks require a Nightly toolchain. Run `$ cargo +nightly bench`.

## License

[MIT](https://github.com/dguo/strsim-rs/blob/master/LICENSE)

[string similarity metrics]:http://en.wikipedia.org/wiki/String_metric
[Damerau-Levenshtein]:http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
[Jaro and Jaro-Winkler]:http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
[Levenshtein]:http://en.wikipedia.org/wiki/Levenshtein_distance
[Hamming]:http://en.wikipedia.org/wiki/Hamming_distance
[Optimal string alignment]:https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Optimal_string_alignment_distance
[Docker]:https://docs.docker.com/engine/installation/
