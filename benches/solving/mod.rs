use barbarosa::{
    cube3::heuristics,
    cube_n::Cube3,
    generic::{alg::Alg, Cube, Movable},
    search::Searchable,
};
use criterion::Criterion;
use rand::{rngs::StdRng, SeedableRng};

pub fn bench(c: &mut Criterion) {
    let heuristics: Vec<(&str, Box<dyn Fn(&Cube3) -> i8>)> = vec![
        ("manhattan", Box::new(heuristics::manhattan)),
        ("mus", Box::new(heuristics::mus)),
    ];

    // Setting the rng to an arbitrary seed for reproducibility
    let mut rng = StdRng::seed_from_u64(69420);

    let amounts = [4, 6].into_iter();

    amounts.for_each(|move_amount: usize| {
        let alg = Alg::random_with_rng(move_amount, &mut rng);
        let cube = Cube3::new_solved().moved(&alg);

        let mut group = c.benchmark_group(format!("solving/{} moves", move_amount));

        for (name, heuristic) in &heuristics {
            group.bench_function(*name, |b| b.iter(|| cube.solve_with_heuristic(heuristic)));
        }

        group.finish();
    });
}