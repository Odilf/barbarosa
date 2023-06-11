use barbarosa::{cube3::{Cube, moves::alg::parse_alg}, search::heuristics};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("4 moves", |b| b.iter(|| {
		let cube = Cube::from(parse_alg("R U R' U'").unwrap());
		cube.solve(heuristics::manhattan)
	}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
