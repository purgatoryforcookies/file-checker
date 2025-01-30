use std::env;
use std::io::Write;
use std::{fs::File, io::BufWriter};

mod file;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();

    let dir1 = &args[1];
    let dir2 = &args[2];

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
    let path = "result.txt";
    let f = File::create(path).expect("unable to create file");
    let mut f = BufWriter::new(f);

    write!(
        f,
        "Errors: {} Time: {:.2?} Rate: {:.2?}\n",
        errors.len(),
        elapsed,
        elapsed / (operation.total_len()).try_into().unwrap()
    )
    .expect("unable to write");
    write!(
        f,
        "Source: {} Destination: {} Total: {}\n\n",
        operation.source_len(),
        operation.destination_len(),
        operation.total_len()
    )
    .expect("unable to write");

    if errors.len() > 0 {
        for e in &errors {
            write!(f, "{} {}\n", e.source_path, e.source_hash).expect("unable to write");
        }

        println!(
            "Found {} errors. Please see result.txt for more information",
            errors.len()
        )
    } else {
        println!("No errors")
    }
}
