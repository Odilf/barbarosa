use barbarosa::{
    cube3::heuristics::mus,
    cube_n::{AxisMove, Cube3},
    generic::{alg::Alg, Cube, Movable},
};
use criterion::Criterion;
use rand::{rngs::StdRng, SeedableRng};

macro_rules! bench_heuristic {
    ($heuristic:tt, $c:ident, [$ ($amount:tt),* ]) => {
        let mut group = $c.benchmark_group("ida*");

        $(
            bench_heuristic!($heuristic, $c, group, $amount);
        )*

        group.finish();
    };



    ($heuristic:tt, $c:tt, $group:expr, $amount:literal) => {
        let mut rng = StdRng::seed_from_u64(69420);
        let alg = Alg::<AxisMove>::random_with_rng($amount, &mut rng);
        let cube = Cube3::new_solved().moved(&alg);

        println!("Benching solving alg {alg}");

        let solution = cube.solve_with_heuristic(&$heuristic).expect("Cube should be solvable");
        assert!(cube.clone().moved(&solution).is_solved());
        assert!(solution.moves.len() <= alg.moves.len(), "Solution: {solution}, alg: {alg}");

        if $amount >= 13 {
            $group.sample_size(10);
        }

        $group.bench_function(format!("{}/{}", stringify!($heuristic), $amount), |b| {
            b.iter(|| -> Alg<AxisMove> { cube.solve_with_heuristic(&$heuristic).unwrap() })
        });
    };
}

pub fn bench(c: &mut Criterion) {
    bench_heuristic!(mus, c, [4, 6, 10, 12, 13]);
}
