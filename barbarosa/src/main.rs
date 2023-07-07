use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[clap(long, action)]
    build_cache: bool,
}

fn main() {
    let args = CliArgs::parse();

    if args.build_cache {
        barbarosa::cube3::mus::cache::Cache::init();
    }
}
