use blake3::Hasher;
use std::{fmt, fs, io};

#[derive(Clone)]
pub struct FileOp {
    pub path: String,
    pub hash: u64,
}

impl FileOp {
    pub fn new(path: &str) -> Self {
        let mut file = fs::File::open(path).expect("Unable to read a file");

        let mut sha256 = Hasher::new();
        let n = io::copy(&mut file, &mut sha256).expect("Could not copy");
        sha256.finalize();
        println!("New hash counted for {} in {}", n, path);

        Self {
            hash: n,
            path: path.to_string(),
        }
    }
}

impl fmt::Debug for FileOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileOp [{} {}]", self.hash, self.path)
    }
}
