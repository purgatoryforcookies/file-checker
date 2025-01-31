# File checker

Small tool written in rust to check how many files in the source directory can also be found in the target directory. 

Uses blake3 to hash files. 


## How it works

Each file in the source and the target searched recursively and hashed.
After both directories have been discovered it is checked that each hash from source is found from the target. 

If a file cannot be found from the target, the file path is written into results. 

After the operation completes, a `result.txt` is written into the current workind directory. 


## How to run 

