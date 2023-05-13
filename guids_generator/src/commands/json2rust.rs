use std::{path::PathBuf, process::exit};

use walkdir::WalkDir;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Dwarf {
    Engineer,
    Gunner,
    Driller,
    Scout,
}

#[derive(Debug, Clone)]
pub enum CostInResource {
    Credits(i32),
    Croppa(f32),
    Umanite(f32),
    Bismor(f32),
    Jadiz(f32),
    Magnite(f32),
    EnorPearl(f32),
}

#[derive(Debug, Clone)]
pub enum OverclockType {
    Clean,
    Balanced,
    Unstable,
}

#[derive(Debug, Clone)]
pub enum CosmeticType {
    Headwear,
    Moustache,
    Beard,
    Sideburns,
    VictoryMove,
    PaintJob,
}

#[derive(Debug, Clone)]
pub enum Schematic {
    Overclock {
        name: String,
        cost: [CostInResource; 4],
        guid: String,
        dwarf: Dwarf,
        ty: OverclockType,
    },
    Cosmetic {
        name: String,
        cost: [CostInResource; 4],
        guid: String,
        dwarf: Dwarf,
        ty: CosmeticType,
    },
    Mineral {
        guid: String,
        resource: CostInResource,
    },
}

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
