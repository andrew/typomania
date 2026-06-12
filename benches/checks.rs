use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use typomania::{
    checks::{Bitflips, Check, Omitted, Repeated, SwappedCharacters, SwappedWords, Typos, Version},
    AuthorSet, Corpus, Harness, Package,
};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-";
const POPULAR: &str = include_str!("popular-crates.txt");

const INPUTS: &[&str] = &[
    "sym",
    "serde-yaml",
    "pin-project-lits",
    "windows-x86-64-gnullvm",
];

struct BenchPackage;

impl AuthorSet for BenchPackage {
    fn contains(&self, _author: &str) -> bool {
        false
    }
}

impl Package for BenchPackage {
    fn authors(&self) -> &dyn AuthorSet {
        self
    }

    fn description(&self) -> Option<&str> {
        None
    }

    fn shared_authors(&self, _other: &dyn AuthorSet) -> bool {
        false
    }
}

struct BenchCorpus(HashMap<String, BenchPackage>);

impl BenchCorpus {
    fn new() -> Self {
        Self(
            POPULAR
                .lines()
                .filter(|l| !l.is_empty())
                .map(|n| (String::from(n), BenchPackage))
                .collect(),
        )
    }
}

impl Corpus for BenchCorpus {
    fn contains_name(&self, name: &str) -> typomania::Result<bool> {
        Ok(self.0.contains_key(name))
    }

    fn get(&self, name: &str) -> typomania::Result<Option<&dyn Package>> {
        Ok(self.0.get(name).map(|p| p as &dyn Package))
    }
}

fn popular_names() -> Vec<&'static str> {
    POPULAR.lines().filter(|l| !l.is_empty()).collect()
}

fn typos_table() -> impl Iterator<Item = (char, Vec<String>)> {
    [
        ('a', vec!["q", "s", "z"]),
        ('e', vec!["w", "r", "3"]),
        ('i', vec!["u", "o", "1", "l"]),
        ('o', vec!["i", "p", "0"]),
        ('s', vec!["a", "d", "5"]),
        ('-', vec!["_", ""]),
        ('_', vec!["-", ""]),
    ]
    .into_iter()
    .map(|(c, v)| (c, v.into_iter().map(String::from).collect()))
}

fn bench_check<C: Check>(c: &mut Criterion, name: &str, check: C) {
    let corpus = BenchCorpus::new();
    let package = BenchPackage;

    let mut group = c.benchmark_group(name);
    for input in INPUTS {
        group.bench_with_input(BenchmarkId::from_parameter(input), input, |b, input| {
            b.iter(|| check.check(&corpus, input, &package).unwrap());
        });
    }
    group.finish();
}

fn checks(c: &mut Criterion) {
    let names = popular_names();

    bench_check(c, "repeated", Repeated);
    bench_check(c, "swapped_characters", SwappedCharacters);
    bench_check(c, "version", Version);
    bench_check(c, "omitted", Omitted::new(ALPHABET));
    bench_check(c, "swapped_words", SwappedWords::new("-_"));
    bench_check(c, "typos", Typos::new(typos_table()));
    bench_check(
        c,
        "bitflips",
        Bitflips::new(ALPHABET, names.iter().copied()),
    );
}

fn harness(c: &mut Criterion) {
    let names = popular_names();

    let harness = Harness::empty_builder()
        .with_check(Repeated)
        .with_check(SwappedCharacters)
        .with_check(Version)
        .with_check(Omitted::new(ALPHABET))
        .with_check(SwappedWords::new("-_"))
        .with_check(Typos::new(typos_table()))
        .with_check(Bitflips::new(ALPHABET, names.iter().copied()))
        .build(BenchCorpus::new());

    let mut group = c.benchmark_group("harness");
    for input in INPUTS {
        group.bench_with_input(BenchmarkId::from_parameter(input), input, |b, input| {
            b.iter(|| {
                harness
                    .check_package(input, Box::new(BenchPackage))
                    .unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, checks, harness);
criterion_main!(benches);
