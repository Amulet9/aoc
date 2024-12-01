
use criterion::{criterion_group, criterion_main, Criterion};
use aoc::day1::{part1};

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/day1/part1");
    c.bench_function("part1", |b| b.iter(|| part1(input)));
}


criterion_group!(benches, bench_part1);
criterion_main!(benches);