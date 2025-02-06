# File checker

Small tool written in rust to check how many files in the source directory can also be found in the target directory. 
Uses blake3 to hash files. 

Use this tool to verify your file copy made it through intact. Does not care about file structure, only checks files regardless where they are.

## How it works

1. Scans source and calculates hash for each file
2. Scans destination and calculates hash for each file
3. Compares destination to source

After the operation completes, a `result.txt` is written into the current working directory containing files that were not found in destination.

## How to run

Easiest way to use the tool is to `git clone` and use `cargo run -- --source *path* --destination *path*`
