use crate::{file::FileOp, scan};
use core::fmt;
use std::io::Write;
use std::time::Instant;
use std::{fs::OpenOptions, thread};

pub struct ComparatorOp {
    dir1: String,
    dir2: String,

    source_ops: Vec<FileOp>,
    destination_ops: Vec<FileOp>,
    errors: Option<Vec<FileOp>>,
    started: Instant,
}

impl ComparatorOp {
    pub fn new(source: &str, destination: &str) -> ComparatorOp {
        let source_s = source.to_string();
        let destination_s = destination.to_string();
        let now = Instant::now();

        let source_files = thread::spawn(move || scan::scan_files(source_s));
        let destination_files = thread::spawn(move || scan::scan_files(destination_s));

        Self {
            dir1: source.to_string(),
            dir2: destination.to_string(),
            source_ops: source_files.join().unwrap(),
            destination_ops: destination_files.join().unwrap(),
            errors: None,
            started: now,
        }
    }

    fn filter_errors(&mut self) {
        self.errors = Some(
            self.source_ops
                .iter()
                .filter(|file| !self.destination_ops.iter().any(|d| d.hash == file.hash))
                .cloned()
                .collect(),
        )
    }

    pub fn check(&mut self) {
        let elapsed = self.started.elapsed();

        let mut data_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("result.txt")
            .expect("cannot open file");

        let elapse_line = format!(
            "Elapsed: {:.2?} - Rate: {:.2?}\n\n",
            elapsed,
            elapsed / self.total_len().try_into().unwrap()
        );

        println!("{elapse_line}");

        let total_line = format!(
            "Source: {} \n --count: {} \nTarget: {} \n --count: {} \n\nTotal: {}\n",
            self.dir1,
            self.source_len(),
            self.dir2,
            self.destination_len(),
            self.total_len()
        );

        data_file
            .write_all(elapse_line.as_bytes())
            .expect("write failed");
        data_file
            .write_all(total_line.as_bytes())
            .expect("write failed");

        self.filter_errors();

        match &self.errors {
            Some(e) => {
                let error_line = format!("Errors: {}\n\n", e.len());

                data_file
                    .write_all(error_line.as_bytes())
                    .expect("write failed");

                for err in e {
                    let errored_file = format!("{} {}\n", err.path, err.hash);

                    data_file
                        .write_all(errored_file.as_bytes())
                        .expect("write failed");
                }
            }
            None => {
                data_file
                    .write_all("No errors found".as_bytes())
                    .expect("write failed");
            }
        }
    }

    pub fn source_len(&self) -> usize {
        self.source_ops.len()
    }
    pub fn destination_len(&self) -> usize {
        self.destination_ops.len()
    }
    pub fn total_len(&self) -> usize {
        self.destination_len() + self.source_len()
    }
}

impl fmt::Debug for ComparatorOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CompareOp [source: {} - {:?} files] [destination: {} - {:?} files] [total: {}]",
            self.dir1,
            self.source_len(),
            self.dir2,
            self.destination_len(),
            self.total_len()
        )
    }
}
