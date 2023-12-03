use aoc_lib::{day1::Day1, Day};
use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    let day = Day1 {
        input: include_str!("../../data/1.in"),
    };
    c.bench_function("Day 1, Part 1", |b| b.iter(|| day.part1()));
}

criterion_group!(benches, day1);
criterion_main!(benches);
