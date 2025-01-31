use walkdir::WalkDir;

use crate::file::FileOp;

pub fn scan_files(path: String) -> Vec<FileOp> {
    println!("Scanning: {}", path);

    let mut files = Vec::new();
    let iterator = WalkDir::new(path);

    for entry in iterator {
        let t = entry.expect("Could not read path");
        if !t.path().is_file() {
            continue;
        }
        let fileop = FileOp::new(t.path().to_str().unwrap());
        files.push(fileop);
    }
    return files;
}
