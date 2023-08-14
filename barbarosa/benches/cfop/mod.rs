use barbarosa::{
    cube3::cfop::cross::{solve_cross, solve_cross_optimally},
    cube_n::space::Face,
    generic::Alg,
    prelude::*,
};
use criterion::Criterion;
use rand::{rngs::StdRng, SeedableRng};

pub fn bench(c: &mut Criterion) {
    bench_cross(c);
}

fn bench_cross(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(69420);

    let mut group = c.benchmark_group("cfop/cross");

    group.bench_function("fast", |b| {
        b.iter(|| {
            let scramble = Alg::<AxisMove>::random_with_rng(20, &mut rng);
            let cube = Cube3::SOLVED.moved(&scramble);

            solve_cross(&cube, &Face::U)
        });
    });

    group.bench_function("optimal", |b| {
        b.iter(|| {
            let scramble = Alg::<AxisMove>::random_with_rng(20, &mut rng);
            let cube = Cube3::SOLVED.moved(&scramble);

            solve_cross_optimally(&cube, &Face::U)
        });
    });
}
