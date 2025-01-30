mod scan;
use blake3::Hasher;
// use sha2::{Digest, Sha256};
use std::{fmt, fs, io, thread};

pub(crate) struct FileOp {
    pub source_path: String,
    pub source_hash: u64,
}

impl FileOp {
    pub fn new(path: &str) -> Self {
        let mut file = fs::File::open(path).expect("Unable to read a file");

        let mut sha256 = Hasher::new();
        let n = io::copy(&mut file, &mut sha256).expect("Could not copy");
        sha256.finalize();
        println!("New hash counted for {} in {}", n, path);

        Self {
            source_hash: n,
            source_path: path.to_string(),
        }
    }
}

impl fmt::Debug for FileOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileOp [{} {}]", self.source_hash, self.source_path)
    }
}

pub(crate) struct ComparatorOp {
    dir1: String,
    dir2: String,

    source_ops: Vec<FileOp>,
    destination_ops: Vec<FileOp>,
}

impl ComparatorOp {
    pub fn new(source: &str, destination: &str) -> Self {
        let source_s = source.to_string();
        let destination_s = destination.to_string();

        let source_files = thread::spawn(move || scan::scan_files(source_s));
        let destination_files = thread::spawn(move || scan::scan_files(destination_s));

        Self {
            dir1: source.to_string(),
            dir2: destination.to_string(),
            source_ops: source_files.join().unwrap(),
            destination_ops: destination_files.join().unwrap(),
        }
    }

    pub fn check(&self) -> Vec<&FileOp> {
        let mut errors = Vec::new();

        for source_file in &self.source_ops {
            let destination_index = self
                .destination_ops
                .iter()
                .position(|f| f.source_hash == source_file.source_hash);

            match destination_index {
                Some(_) => continue,
                None => errors.push(source_file),
            }
        }

        return errors;
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
