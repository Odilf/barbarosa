use barbarosa::{
    cube3::heuristics::{manhattan, mus},
    cube_n::{AxisMove, Cube3},
    generic::{alg::Alg, Cube, Movable},
};
use criterion::Criterion;
use rand::{rngs::StdRng, SeedableRng};

macro_rules! bench_heuristic {
    ($heuristic:tt, $c:tt, [$($amount:literal),*]) => {
        let mut group = $c.benchmark_group("ida*");

        $(        
            let mut rng = StdRng::seed_from_u64(69420);
            let alg = Alg::<AxisMove>::random_with_rng($amount, &mut rng);
            let cube = Cube3::new_solved().moved(&alg);

            println!("Benching solving alg {alg}");

            let solution = cube.solve_with_heuristic(&$heuristic).unwrap();
            assert!(cube.clone().moved(&solution).is_solved());

            group.bench_function(format!("{}/{}", stringify!($heuristic), $amount), |b| {
                b.iter(|| -> Alg<AxisMove> { cube.solve_with_heuristic(&$heuristic).unwrap() })
            });
        )*

        group.finish();
    };
}

pub fn bench(c: &mut Criterion) {
    bench_heuristic!(manhattan, c, [4, 5]);
    bench_heuristic!(mus, c, [4, 6, 10]);    
}
