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
            path: path.to_string().replace("\\", "/"),
        }
    }
}

impl fmt::Debug for FileOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileOp [{} {}]", self.hash, self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes_file() {
        let file = FileOp::new("src/__fixtures__/folder1/file1");
        assert_eq!(file.hash, 12);
        assert_eq!(file.path, "src/__fixtures__/folder1/file1");
    }

    #[test]
    #[should_panic]
    fn it_panics_if_path_not_found() {
        // Theres no error handling in the code.
        // TODO: Handle errors
        FileOp::new("src/__fixtures__/folder1/file4");
    }
}
