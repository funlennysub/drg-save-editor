use std::{error, fs::File, io::BufReader, path::PathBuf, process::exit};

use super::*;

pub fn run(file: PathBuf, out: PathBuf) {
    if !file.is_file() {
        eprintln!("Specified path is not a file!");
        exit(1);
    }
    let _ = inner(file, out);
}

struct Guid([u8; 16]);

fn inner(file: PathBuf, out: PathBuf) -> Result<(), Box<dyn error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let schematics: Vec<Schematic> = serde_json::from_reader(reader)?;
    print!("HashMap::from([");
    for sh in schematics {
        let guid = match &sh {
            Schematic::Overclock { guid, .. } => guid,
            Schematic::Cosmetic { guid, .. } => guid,
            Schematic::Mineral { guid, .. } => guid,
        };
        let guid = str_to_u8(guid);
        println!("({guid:?}, {sh}),");
    }
    print!("]);");

    Ok(())
}
