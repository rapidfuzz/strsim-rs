//! Benchmarks for strsim using Criterion.

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
extern crate strsim;

use std::hint::black_box;

fn run_benchmark<F>(c: &mut Criterion, name: &str, f: F)
where
    F: Fn() + 'static,
{
    c.bench_function(name, |b| b.iter(|| black_box(f())));
}

fn run_benchmark_setup<F, S, I, O>(c: &mut Criterion, name: &str, mut setup: S, mut f: F)
where
    S: FnMut() -> I,
    F: FnMut(I) -> O,
{
    c.bench_function(name, |b| {
        b.iter_batched(
            || setup(),
            |input| black_box(f(input)),
            criterion::BatchSize::SmallInput,
        )
    });
}

/* -------------------------------------------------------------------------- */
/*  Hamming distance                                                          */
/* -------------------------------------------------------------------------- */
fn bench_hamming(c: &mut Criterion) {
    let a = "ACAAGATGCCATTGTCCCCCGGCCTCCTGCTGCTGCTGCTCTCCGGGG";
    let b = "CCTGGAGGGTGGCCCCACCGGCCGAGACAGCGAGCATATGCAGGAAGC";

    run_benchmark(c, "hamming", || {
        // `unwrap` is kept because the original benchmark did it.
        strsim::hamming(a, b).unwrap();
    });
}

/* -------------------------------------------------------------------------- */
/*  Jaro                                                                     */
/* -------------------------------------------------------------------------- */
fn bench_jaro(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "jaro", || {
        strsim::jaro(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  Jaro‑Winkler                                                             */
/* -------------------------------------------------------------------------- */
fn bench_jaro_winkler(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "jaro_winkler", || {
        strsim::jaro_winkler(a, b);
    });
}

fn bench_jaro_longstring(c: &mut Criterion) {
    let a = "abcd".repeat(3000);
    let b = "abce".repeat(3000);
    run_benchmark_setup(
        c,
        "jaro_longstring",
        || (a.clone(), b.clone()),
        |(a, b)| {
            strsim::jaro(&a, &b);
        },
    );
}

/* -------------------------------------------------------------------------- */
/*  Levenshtein                                                              */
/* -------------------------------------------------------------------------- */
fn bench_levenshtein(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "levenshtein", || {
        strsim::levenshtein(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  Levenshtein on `u8` slices                                               */
/* -------------------------------------------------------------------------- */
fn bench_levenshtein_on_u8(c: &mut Criterion) {
    run_benchmark(c, "levenshtein_u8", || {
        strsim::generic_levenshtein(&vec![0u8; 30], &vec![7u8; 31]);
    });
}

/* -------------------------------------------------------------------------- */
/*  Normalized Levenshtein                                                   */
/* -------------------------------------------------------------------------- */
fn bench_normalized_levenshtein(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "normalized_levenshtein", || {
        strsim::normalized_levenshtein(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  OSA distance                                                             */
/* -------------------------------------------------------------------------- */
fn bench_osa_distance(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "osa_distance", || {
        strsim::osa_distance(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  Damerau‑Levenshtein                                                      */
/* -------------------------------------------------------------------------- */
fn bench_damerau_levenshtein(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "damerau_levenshtein", || {
        strsim::damerau_levenshtein(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  Normalized Damerau‑Levenshtein                                            */
/* -------------------------------------------------------------------------- */
fn bench_normalized_damerau_levenshtein(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "normalized_damerau_levenshtein", || {
        strsim::normalized_damerau_levenshtein(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  Sørensen‑Dice                                                            */
/* -------------------------------------------------------------------------- */
fn bench_sorensen_dice(c: &mut Criterion) {
    let a = "Philosopher Friedrich Nietzsche";
    let b = "Philosopher Jean-Paul Sartre";

    run_benchmark(c, "sorensen_dice", || {
        strsim::sorensen_dice(a, b);
    });
}

/* -------------------------------------------------------------------------- */
/*  Long Sørensen‑Dice (multiple inputs, larger data)                        */
/* -------------------------------------------------------------------------- */
fn bench_sorensen_dice_long(c: &mut Criterion) {
    // A collection of string pairs with varying lengths and characteristics.
    let pairs = [
        // Short, similar strings
        ("night", "nacht"),
        // Medium, partially overlapping
        ("rust programming language", "rust language programming"),
        // Long, realistic sentences
        (
            "The quick brown fox jumps over the lazy dog while the sun sets behind the hills",
            "A swift auburn fox leaped over a sleepy canine as dusk fell beyond the mountains",
        ),
        // Persian thing
        (
            "در گذر زمان خواهی آموخت هر کسی ارزش جنگیدن ندارد",
            "در گذر زمان خواهی فهمید هر جایی ارزش ماندن ندارد",
        ),
        // Very long repetitive patterns
        (&"abcde".repeat(2000), &"abfde".repeat(2000)),
        // Unicode strings with diacritics
        ("café au lait", "cafe au lait"),
        // Strings with emojis
        ("😀😃😄😁😆", "😀😃😄😁😅"),
    ];

    // Benchmark each pair individually to capture variance.
    for (i, (a, b)) in pairs.iter().enumerate() {
        let name = format!("sorensen_dice_long_{}", i);
        // Clone the original `&str` values into owned `String`s once (setup phase).
        let a_owned = a.to_string();
        let b_owned = b.to_string();
        run_benchmark_setup(
            c,
            &name,
            || {
                // Setup phase: prepare owned strings.
                let _a = a_owned.clone();
                let _b = b_owned.clone();
                (a, b)
            },
            |(a, b)| {
                strsim::sorensen_dice(&a, &b);
            },
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(3));
    targets =
        bench_hamming,
        bench_jaro,
        bench_jaro_winkler,
        bench_jaro_longstring,
        bench_levenshtein,
        bench_levenshtein_on_u8,
        bench_normalized_levenshtein,
        bench_osa_distance,
        bench_damerau_levenshtein,
        bench_normalized_damerau_levenshtein,
        bench_sorensen_dice,
        bench_sorensen_dice_long
}
criterion_main!(benches);
