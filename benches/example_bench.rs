use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

/// Example: benchmark a function with varying input sizes.
/// Replace with calls to your crate's API.
fn process(input: &[u8]) -> usize {
    input.iter().filter(|b| b.is_ascii_alphanumeric()).count()
}

fn bench_process(c: &mut Criterion) {
    let mut group = c.benchmark_group("process");
    for size in [64, 256, 1024, 4096] {
        let input: Vec<u8> = (0..size).map(|i| (i % 128) as u8).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| process(black_box(input)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_process);
criterion_main!(benches);