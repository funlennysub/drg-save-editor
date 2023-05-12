use std::{
    fs::File,
    path::{Path, PathBuf},
    process::exit,
};

use regex::Regex;
use unreal_asset::{
    cast, engine_version::EngineVersion, exports::Export, properties::Property, Asset,
};

use super::create_write_pretty;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct MatrixCore {
    guid: String,
    name: String,
    class: Option<String>,
}

pub fn run(asset_dir: &Path, out: PathBuf) {
    let res = inner(asset_dir);
    if res.is_err() {
        eprintln!("Error getting information: {:?}", res.unwrap_err());
        exit(1);
    }

    let res = res.unwrap();
    create_write_pretty(out, res)
}

fn inner(asset_dir: &Path) -> Result<Vec<MatrixCore>, Box<dyn std::error::Error>> {
    let analytics_dir = asset_dir
        .join("FSD")
        .join("Content")
        .join("Game")
        .join("Analytics");

    let mut cores = Vec::new();

    let data_file =
        File::open(&analytics_dir.join("Google_Analytics_Schematic_ID_mapping.uasset"))?;
    let bulk_file = File::open(&analytics_dir.join("Google_Analytics_Schematic_ID_mapping.uexp"))?;
    let asset = Asset::new(data_file, Some(bulk_file), EngineVersion::VER_UE4_27)?;
    let exports = cast!(Export, DataTableExport, &asset.asset_data.exports[0]).unwrap();
    let guid_regex = Regex::new("([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})-([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})-([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})-([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})")?;
    for export in &exports.table.data {
        let key = export.name.get_content();
        let guid = guid_regex
            .replace_all(&key, "$4$3$2$1-$8$7$6$5-$12$11$10$9-$16$15$14$13")
            .to_string();
        let property = cast!(Property, StrProperty, &export.value[1]).unwrap();
        let name = property.value.clone().unwrap();
        let temp = &name.split_inclusive(" - ").collect::<Vec<_>>();
        let name = if temp.len() > 2 {
            temp[..=1].join("").to_string()
        } else {
            temp.first().unwrap().to_string()
        };
        let name = name[..name.len() - 3].to_string();
        let class = temp.last();

        cores.push(MatrixCore {
            name,
            guid,
            class: class.map(|c| c.to_owned().to_owned()),
        });
    }
    Ok(cores)
}
