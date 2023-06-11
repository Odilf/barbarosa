use barbarosa::{cube3::{Cube, moves::alg::parse_alg}, search::heuristics};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
	bench_ida(c);
}

fn bench_ida(c: &mut Criterion) {
	let mut group = c.benchmark_group("Ida*");
	let algs = [
		"R",
		"R U",
		"L D F",
		"R2 F D L",
		"R2 F D L U",
	].map(|alg| parse_alg(alg).unwrap());
	
	for alg in algs {
		let cube = Cube::from(&alg);
		group.bench_with_input(format!("{} moves", &alg.len()), &alg, |b, _alg| b.iter(|| {
			cube.solve(heuristics::manhattan)
		}));
	}
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
