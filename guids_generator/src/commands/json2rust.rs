use std::{path::PathBuf, process::exit};

use walkdir::WalkDir;

pub fn run(input: &PathBuf) {
    if !input.is_dir() {
        eprintln!("Specified path is not a directory!");
        exit(1);
    }

    let walker = WalkDir::new(input).into_iter();
    for entry in walker.filter_map(|e| e.ok()).filter(|e| e.path().is_file()) {
        println!("{:?}", entry.path());
    }
}
