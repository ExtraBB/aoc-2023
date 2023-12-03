use aoc_lib::{day1::Day1, day2::Day2, day3::Day3, Day};
use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    let day = Day1 {
        input: include_str!("../../data/1.in"),
    };
    c.bench_function("Day 1, Part 1", |b| b.iter(|| day.part1()));
    c.bench_function("Day 1, Part 2", |b| b.iter(|| day.part2()));
}

fn day2(c: &mut Criterion) {
    let day = Day2 {
        input: include_str!("../../data/2.in"),
    };
    c.bench_function("Day 2, Part 1", |b| b.iter(|| day.part1()));
    c.bench_function("Day 2, Part 2", |b| b.iter(|| day.part2()));
}

fn day3(c: &mut Criterion) {
    let day = Day3 {
        input: include_str!("../../data/3.in"),
    };
    c.bench_function("Day 3, Part 1", |b| b.iter(|| day.part1()));
}

criterion_group!(benches, day1, day2, day3);
criterion_main!(benches);
