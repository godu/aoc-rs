use common::read_input;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn part1(_input: &str) -> usize {
    todo!()
}

fn part2(_input: &str) -> usize {
    todo!()
}

fn bench_day01(c: &mut Criterion) {
    let input = read_input(1);
    c.bench_function("day01_part1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("day01_part2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, bench_day01);
criterion_main!(benches);
