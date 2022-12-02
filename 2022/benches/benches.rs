use advent_of_code_2022::{day1, safe_lines};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn criterion_benchmark(c: &mut Criterion) {
    let mut day1_lines: Vec<String> = safe_lines("res/day_1_input.txt").unwrap().collect();
    day1_lines.extend(day1_lines.clone());
    day1_lines.extend(day1_lines.clone());

    let mut day1 = c.benchmark_group("day 1");
    day1.throughput(Throughput::Elements(day1_lines.len() as u64));

    day1.bench_with_input("solution 1", &day1_lines.clone(), |b, i| {
        b.iter(|| day1::solution::<100, _, _>(i.iter()))
    });
    day1.bench_with_input("solution 2", &day1_lines.clone(), |b, i| {
        b.iter(|| day1::solution_improved(100, i.iter()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
