use aoc_lib::day1;
use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    let data = include_str!("../../data/1.in");
    c.bench_function("Day 1, Part 1", |b| b.iter(|| day1::part1(&data)));
}

criterion_group!(benches, day1);
criterion_main!(benches);
