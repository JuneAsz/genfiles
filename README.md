# genfiles

A simple Rust CLI tool that generates random files.

This is not a real project, just something made to practice Rust.

## Usage

Build in release mode:

cargo build --release


Run:

genfiles -a 100 -e "txt" -p "C:\output"


## Options

- `-a`, `--amount` — number of files to create  
- `-e`, `--extension` — file extension  
- `-p`, `--path` — output directory  


## Dependencies

- `clap` — for command-line argument parsing  
- `rand` — for random file name generation  
- `colored` — for colored terminal output  

## Note

This is a small learning project.  
Not meant for production use.
