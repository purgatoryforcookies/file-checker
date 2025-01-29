use std::io::Write;
use std::{fs::File, io::BufWriter};

mod file;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let dir1 = "target";
    let dir2 = "target2";

    let operation = file::ComparatorOp::new(&dir1, &dir2);

    println!("{:?}", operation);

    let elapsed = now.elapsed();
    println!(
        "Elapsed: {:.2?} - Rate: {:.2?}",
        elapsed,
        elapsed / (operation.total_len()).try_into().unwrap()
    );

    println!("Checking for integrity...");

    let errors = operation.check();

    if errors.len() > 0 {
        let path = "result.txt";
        let f = File::create(path).expect("unable to create file");
        let mut f = BufWriter::new(f);

        for e in &errors {
            write!(f, "{} <> {}\n", e.source_path, e.source_hash).expect("unable to write");
        }

        println!(
            "Found {} errors. Please see result.txt for more information",
            errors.len()
        )
    } else {
        println!("No errors")
    }
}
