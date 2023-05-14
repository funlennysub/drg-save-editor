use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek},
    path::Path,
    process::exit,
};

use unreal_asset::{
    cast, properties::Property, reader::archive_trait::ArchiveTrait, types::fname::FName, Asset,
};

pub(crate) mod assets;
pub(crate) mod cores;
pub(crate) mod json2rust;

pub(crate) trait ImportNoIdx {
    fn find_import_no_index_by_content(
        &self,
        class_package: &FName,
        class_name: &FName,
        object_name: &FName,
    ) -> Option<i32>;
}

impl<C: Read + Seek> ImportNoIdx for Asset<C> {
    fn find_import_no_index_by_content(
        &self,
        class_package: &FName,
        class_name: &FName,
        object_name: &FName,
    ) -> Option<i32> {
        for i in 0..self.imports.len() {
            let import = &self.imports[i];
            if import.class_package.get_content() == *class_package.get_content()
                && import.class_name.get_content() == *class_name.get_content()
                && import.object_name.get_content() == *object_name.get_content()
            {
                return Some(-(i as i32) - 1);
            }
        }
        None
    }
}

pub(crate) fn open_file(path: &Path) -> File {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not find/open file! Error: {err}");
            exit(1);
        }
    }
}

pub(crate) fn get_res_amount(name: &str, amount: i32) -> ResourceAmount {
    match name {
        "RES_Credits" => ResourceAmount::Credits(amount),
        "RES_EMBED_Enor" => ResourceAmount::EnorPearl(amount as f32),
        "RES_EMBED_Jadiz" => ResourceAmount::Jadiz(amount as f32),
        "RES_CARVED_Bismor" => ResourceAmount::Bismor(amount as f32),
        "RES_CARVED_Umanite" => ResourceAmount::Umanite(amount as f32),
        "RES_CARVED_Magnite" => ResourceAmount::Magnite(amount as f32),
        "RES_VEIN_Croppa" => ResourceAmount::Croppa(amount as f32),
        s => todo!("{}", s),
    }
}

pub(crate) fn u8_to_string(arr: [u8; 16]) -> String {
    arr.iter()
        .map(|d| format!("{d:02X}"))
        .collect::<Vec<_>>()
        .chunks_exact(4)
        .map(|c| c.join(""))
        .collect::<Vec<_>>()
        .join("-")
}

pub(crate) fn get_savegame_id(properties: &[Property]) -> Option<String> {
    properties
        .iter()
        .filter_map(|p| cast!(Property, StructProperty, p))
        .find(|p| p.name.get_content() == "SaveGameID")
        .map(|p| u8_to_string(cast!(Property, GuidProperty, &p.value[0]).unwrap().value))
}

pub(crate) fn get_crafting_cost(props: &[Property], asset: &Asset<File>) -> Vec<ResourceAmount> {
    let cost = props
        .iter()
        .filter_map(|p| cast!(Property, MapProperty, &p))
        .find(|p| p.name.get_content() == "CraftingCost")
        .map(|p| &p.value)
        .unwrap();
    cost.iter()
        .map(|(_, index, price)| {
            let price = cast!(Property, IntProperty, price).unwrap().value;
            let what = cast!(Property, ObjectProperty, index).unwrap().value;

            get_res_amount(
                &asset.get_import(what).unwrap().object_name.get_content(),
                price,
            )
        })
        .collect::<Vec<_>>()
}

pub(crate) fn create_write_pretty<T: serde::Serialize>(out_file: &Path, res: T) {
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
                .append(false)
                .open(out_file)
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) enum Dwarf {
    Engineer,
    Gunner,
    Driller,
    Scout,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum ResourceAmount {
    Credits(i32),
    Croppa(f32),
    Umanite(f32),
    Bismor(f32),
    Jadiz(f32),
    Magnite(f32),
    EnorPearl(f32),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum OverclockType {
    Clean,
    Balanced,
    Unstable,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum CosmeticType {
    Headwear,
    Moustache,
    Beard,
    Sideburns,
    VictoryMove,
    PaintJob,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Guid(pub [u8; 16]);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum Schematic {
    Overclock {
        guid: String,
        name: String,
        cost: Vec<ResourceAmount>,
        dwarf: Dwarf,
        ty: OverclockType,
    },
    Cosmetic {
        guid: String,
        name: String,
        cost: Vec<ResourceAmount>,
        dwarf: Dwarf,
        ty: CosmeticType,
    },
    Mineral {
        guid: String,
        resource: ResourceAmount,
    },
}
