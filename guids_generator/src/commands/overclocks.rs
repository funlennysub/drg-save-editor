use std::{
    fs::{self, File},
    io::{Read, Seek},
    path::{Path, PathBuf},
    process::exit,
};

use unreal_asset::{
    cast,
    engine_version::EngineVersion,
    exports::{Export, ExportBaseTrait, ExportNormalTrait},
    properties::Property,
    reader::archive_trait::ArchiveTrait,
    types::fname::FName,
    Asset,
};

use crate::fname;

use super::{create_write_pretty, Dwarf, ImportNoIdx, ResourceAmount};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OverclockType {
    Clean,
    Balanced,
    Unstable,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Overclock {
    pub name: String,
    pub cost: Vec<ResourceAmount>,
    pub guid: String,
    pub ty: OverclockType,
    pub dwarf: Dwarf,
}

pub fn run(asset_dir: &Path, out_file: PathBuf) {
    let res = overclocks_command_inner(asset_dir);
    if res.is_err() {
        eprintln!("Error reading assets: {:?}", res.unwrap_err());
        exit(1);
    }

    let res = res.unwrap();
    create_write_pretty(out_file, res);
}

fn overclocks_command_inner(
    asset_dir: &Path,
) -> Result<Vec<Overclock>, Box<dyn std::error::Error>> {
    let content_dir = asset_dir.join("FSD").join("Content");
    let st_path = &content_dir.join("Game").join("Text");
    let st_uasset = File::open(st_path.join("ST_GearUpgrades.uasset"))?;
    let st_uexp = File::open(st_path.join("ST_GearUpgrades.uexp"))?;
    let st_asset = Asset::new(st_uasset, Some(st_uexp), EngineVersion::VER_UE4_27)?;
    let st = &cast!(Export, StringTableExport, &st_asset.asset_data.exports[0])
        .unwrap()
        .table;

    let mut ocs_vec = Vec::new();

    for file in fs::read_dir(content_dir.join("WeaponsNTools"))? {
        let path = file?.path();

        if !(path.is_dir() && (path.join("Overclocks").exists() || path.join("OCs").exists())) {
            continue;
        }

        let ocs_path = if path.join("Overclocks").exists() {
            path.join("Overclocks")
        } else {
            path.join("OCs")
        };

        let mut osb_file_uasset = None;
        let mut osb_file_uexp = None;
        for oc_file in fs::read_dir(&ocs_path)? {
            let oc_file = oc_file?;
            if fname!(oc_file).starts_with("OSB_") {
                if fname!(oc_file).ends_with(".uasset") {
                    osb_file_uasset = Some(oc_file.path());
                }
                if fname!(oc_file).ends_with(".uexp") {
                    osb_file_uexp = Some(oc_file.path());
                }
            }
        }

        if let (Some(osb_file_uasset), Some(osb_file_uexp)) = (osb_file_uasset, osb_file_uexp) {
            let asset_data = File::open(&osb_file_uasset)?;
            let bulk_data = File::open(&osb_file_uexp)?;
            let asset = Asset::new(asset_data, Some(bulk_data), EngineVersion::VER_UE4_27)?;

            let schematic_index = asset
                .find_import_no_index_by_content(
                    &FName::from_slice("/Script/CoreUObject"),
                    &FName::from_slice("Class"),
                    &FName::from_slice("Schematic"),
                )
                .unwrap();
            let clean_oc = asset
                .find_import_no_index_by_content(
                    &FName::from_slice("/Script/FSD"),
                    &FName::from_slice("SchematicCategory"),
                    &FName::from_slice("SCAT_OC_Clean"),
                )
                .unwrap();
            let balanced_oc = asset
                .find_import_no_index_by_content(
                    &FName::from_slice("/Script/FSD"),
                    &FName::from_slice("SchematicCategory"),
                    &FName::from_slice("SCAT_OC_Balanced"),
                )
                .unwrap();
            let unstable_oc = asset
                .find_import_no_index_by_content(
                    &FName::from_slice("/Script/FSD"),
                    &FName::from_slice("SchematicCategory"),
                    &FName::from_slice("SCAT_OC_Unstable"),
                )
                .unwrap();

            for export in asset
                .asset_data
                .exports
                .iter()
                .filter(|e| e.get_base_export().class_index.index == schematic_index)
            {
                let normal_export = export.get_normal_export().ok_or("ne")?;
                let properties = &normal_export.properties;

                let oc_type = properties
                    .iter()
                    .filter_map(|p| cast!(Property, ObjectProperty, p))
                    .find(|p| p.name.get_content() == "Category")
                    .map(|p| match p.value.index {
                        i if i == clean_oc => OverclockType::Clean,
                        i if i == balanced_oc => OverclockType::Balanced,
                        i if i == unstable_oc => OverclockType::Unstable,
                        _ => unreachable!(),
                    })
                    .unwrap();
                let cost = &cast!(Property, MapProperty, &properties[5])
                    .ok_or("mp")?
                    .value;

                let guid = properties
                    .iter()
                    .filter_map(|p| cast!(Property, StructProperty, p))
                    .find(|p| p.name.get_content() == "SaveGameID")
                    .map(|p| {
                        cast!(Property, GuidProperty, &p.value[0])
                            .unwrap()
                            .value
                            .iter()
                            .map(|d| format!("{d:02X}"))
                            .collect::<Vec<_>>()
                            .chunks_exact(4)
                            .map(|c| c.join(""))
                            .collect::<Vec<_>>()
                            .join("-")
                    })
                    .unwrap();

                let dwarf = properties
                    .iter()
                    .filter_map(|p| cast!(Property, ObjectProperty, p))
                    .find(|p| p.name.get_content() == "UsedByCharacter")
                    .map(|p| {
                        match asset
                            .get_import(p.value)
                            .unwrap()
                            .object_name
                            .get_content()
                            .as_str()
                        {
                            "ScoutID" => Dwarf::Scout,
                            "DrillerID" => Dwarf::Driller,
                            "GunnerID" => Dwarf::Gunner,
                            "EngineerID" => Dwarf::Engineer,
                            _ => unreachable!(),
                        }
                    })
                    .unwrap();

                let cost = cost
                    .iter()
                    .map(|(_, index, price)| {
                        let price = cast!(Property, IntProperty, price).unwrap().value;
                        let what = cast!(Property, ObjectProperty, index).unwrap().value;

                        match asset
                            .get_import(what)
                            .unwrap()
                            .object_name
                            .get_content()
                            .as_str()
                        {
                            "RES_Credits" => ResourceAmount::Credits(price),
                            "RES_EMBED_Enor" => ResourceAmount::EnorPearl(price as f32),
                            "RES_EMBED_Jadiz" => ResourceAmount::Jadiz(price as f32),
                            "RES_CARVED_Bismor" => ResourceAmount::Bismor(price as f32),
                            "RES_CARVED_Umanite" => ResourceAmount::Umanite(price as f32),
                            "RES_CARVED_Magnite" => ResourceAmount::Magnite(price as f32),
                            "RES_VEIN_Croppa" => ResourceAmount::Croppa(price as f32),
                            s => todo!("{}", s),
                        }
                    })
                    .collect::<Vec<_>>();

                let item = cast!(Property, ObjectProperty, &properties[4])
                    .ok_or("op")?
                    .value;
                let item = &asset
                    .get_export(item)
                    .unwrap()
                    .get_normal_export()
                    .unwrap()
                    .properties;
                let oc = cast!(Property, ObjectProperty, &item[1]).unwrap().value;
                let oc_file = &asset
                    .get_import(asset.get_import(oc).unwrap().outer_index)
                    .unwrap()
                    .object_name
                    .get_content();
                let oc_file = Path::new(oc_file)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let oc_path = ocs_path.join(oc_file);

                let oc_asset_data = File::open(format!("{}.uasset", oc_path.display()))?;
                let oc_bulk_data = File::open(format!("{}.uexp", oc_path.display()))?;
                let oc_asset =
                    Asset::new(oc_asset_data, Some(oc_bulk_data), EngineVersion::VER_UE4_27)?;

                let ou_import = oc_asset
                    .find_import_no_index_by_content(
                        &FName::from_slice("/Script/CoreUObject"),
                        &FName::from_slice("Class"),
                        &FName::from_slice("OverclockUpgrade"),
                    )
                    .unwrap();

                let export = &oc_asset
                    .asset_data
                    .exports
                    .iter()
                    .find(|e| e.get_base_export().class_index.index == ou_import)
                    .unwrap();
                let name_export = &export.get_normal_export().unwrap().properties[2];
                let name = &cast!(Property, TextProperty, &name_export).unwrap();
                let name = if name.culture_invariant_string.is_none() {
                    st.get_by_key(&name.value.clone().unwrap())
                } else {
                    name.culture_invariant_string.as_ref()
                };

                ocs_vec.push(Overclock {
                    name: name.unwrap().to_owned(),
                    cost,
                    guid,
                    ty: oc_type,
                    dwarf,
                });
            }
        }
    }
    Ok(ocs_vec)
}
