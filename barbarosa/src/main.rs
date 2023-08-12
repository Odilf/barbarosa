use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    /// Build the MUS cache. Takes like 2-10 mins on `--release` (and way to long on `--debug`)
    #[clap(long, action)]
    build_mus_cache: bool,

    /// Start an interactive 3x3 playgorund on the terminal
    #[clap(short, long, action)]
    playground_3x3: bool,
}

fn main() {
    let args = CliArgs::parse();

    if args.build_mus_cache {
        barbarosa::cube3::mus::cache::Cache::init();
    }

    if args.playground_3x3 {
        playground_3x3();
    }
}

fn playground_3x3() {
    use std::io::{BufRead, Write};

    use barbarosa::{
        cube_n::{moves::ExtendedAxisMove, Cube3, Orientable},
        generic::{Alg, Cube, Movable, Parsable},
    };

    let mut cube = Orientable::new(Cube3::SOLVED);

    fn playground_loop(cube: &mut Orientable<Cube3>) -> Result<(), Box<dyn std::error::Error>> {
        print!("> ");
        std::io::stdout().flush()?;
        let input = std::io::stdin().lock().lines().next().unwrap()?;

        let mov = Alg::<ExtendedAxisMove>::parse(&input)?;

        cube.apply(&mov);

        println!("{}", cube.base_cube);

        Ok(())
    }

    println!("{}", cube.base_cube);

    loop {
        if let Err(err) = playground_loop(&mut cube) {
            eprintln!("Error: {}", err);
        }
    }
}
