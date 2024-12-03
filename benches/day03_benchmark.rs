use aoc2024::day3::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    c.bench_function("day03_part1", |b| b.iter(|| part1(black_box(input))));
}

pub fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    c.bench_function("day03_part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
