use barbarosa::cube3::{heuristics, moves::alg::parse_alg, Cube};

use criterion::{criterion_group, criterion_main, Criterion};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_manhattan(c);
    bench_mus(c);
}

fn bench_manhattan(c: &mut Criterion) {
    let mut group = c.benchmark_group("A* manhattan move ladder");
    let algs = [
        "R",
        "R U",
        "L D F",
        "R2 F D L",
        "R2 F D L U",
        "D2 F D L U R'",
        "D2 F D L U R' D2",
        // "D2 F D L U R' D2 B'",
    ]
    .map(|alg| parse_alg(alg).unwrap());

    for alg in algs {
        let cube = Cube::from(&alg);

        group.bench_with_input(format!("{} moves", &alg.len()), &alg, |b, _alg| {
            b.iter(|| cube.solve_with_heuristic(heuristics::manhattan))
        });
    }
}

fn bench_mus(c: &mut Criterion) {
    let mut group = c.benchmark_group("A* manhattan move ladder");
    let algs = [
        "R",
        "R U",
        "L D F",
        "R2 F D L",
        "R2 F D L U",
        "D2 F D L U R'",
        "D2 F D L U R' D2",
        // "D2 F D L U R' D2 B'",
    ]
    .map(|alg| parse_alg(alg).unwrap());

    for alg in algs {
        let cube = Cube::from(&alg);

        group.bench_with_input(format!("{} moves", &alg.len()), &alg, |b, _alg| {
            b.iter(|| cube.solve_with_heuristic(heuristics::mus))
        });
    }
}
