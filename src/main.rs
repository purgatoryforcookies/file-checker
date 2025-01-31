mod core;
mod file;
mod scan;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dir1 = &args[1];
    let dir2 = &args[2];

    let mut operation = core::ComparatorOp::new(&dir1, &dir2);
    operation.check();
}
