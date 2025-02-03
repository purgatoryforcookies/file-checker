mod core;
mod file;
mod scan;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    source: String,
    #[clap(short, long)]
    destination: String,
}

fn main() {
    let cli = Args::parse();

    let mut operation = core::ComparatorOp::new(&cli.source, &cli.destination);
    operation.check();
}
