use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    process::exit,
};

pub mod assets;
pub mod cores;
pub mod json2rust;
pub mod overclocks;

pub fn open_file(path: &Path) -> File {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not find/open file! Error: {err}");
            exit(1);
        }
    }
}

pub fn create_write_pretty<T: serde::Serialize>(out_file: PathBuf, res: T) {
    let dir_path = match out_file.parent() {
        Some(dir) => dir,
        None => {
            eprintln!("No parent directories found!");
            exit(1);
        }
    };

    match fs::create_dir_all(dir_path) {
        Ok(_) => {
            let mut file = match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&out_file)
            {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error creating file. Error: {err}");
                    exit(1);
                }
            };

            match serde_json::to_writer_pretty(&mut file, &res) {
                Ok(_) => println!("Done."),
                Err(err) => {
                    eprintln!("Error writing to file. Error: {err}");
                    exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error creating directories. Error: {err}");
            exit(1);
        }
    }
}

#[macro_export]
macro_rules! fname {
    ($f:expr) => {
        $f.file_name().to_string_lossy().to_string()
    };
}
