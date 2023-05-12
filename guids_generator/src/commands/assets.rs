use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::exit,
};

use unreal_pak::PakReader;

use crate::commands::open_file;

// https://github.com/AstroTechies/unrealmodding/tree/main/unreal_pak_cli
pub fn run(pak_file: &PathBuf, asset_dir: &PathBuf) {
    let path = Path::new(&pak_file);
    let file = open_file(path);
    let mut pak = PakReader::new(&file);
    check_header(&mut pak);

    // temp values required to extend lifetimes outside of match scope
    let output_folder = PathBuf::from(&asset_dir);

    println!("Extracting to {output_folder:?}");

    for (i, (file_name, data)) in pak.iter().enumerate() {
        match data {
            Ok(data) => {
                let path = output_folder.join(file_name);
                let dir_path = match path.parent() {
                    Some(dir) => dir,
                    None => {
                        eprintln!("No parent directories found! {i}: {file_name:?}");
                        exit(1);
                    }
                };

                // Create the parent directories, then files.
                match std::fs::create_dir_all(dir_path) {
                    Ok(_) => {
                        // Create the file
                        let mut file = match File::create(&path) {
                            Ok(file) => file,
                            Err(err) => {
                                eprintln!("Error creating file {i}: {path:?}! Error: {err}");
                                exit(1);
                            }
                        };
                        // Write the file
                        match file.write_all(data.as_slice()) {
                            Ok(_) => {
                                println!("Record {i}: {file_name}");
                            }
                            Err(err) => {
                                eprintln!("Error writing to file {i}: {path:?}! Error: {err}");
                                exit(1);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error creating directories {dir_path:?}! Error: {err}");
                        exit(1);
                    }
                };
            }
            Err(err) => {
                eprintln!("Error reading record {i}: {file_name:?}! Error: {err}");
                exit(1);
            }
        }
    }
}

fn check_header(pak: &mut PakReader<File>) {
    match pak.load_index() {
        Ok(_) => println!("Header is ok"),
        Err(err) => {
            eprintln!("Error reading header! Error: {err}");
            exit(1);
        }
    }
    println!("Found {:?} records", pak.get_entry_names().len());
}
