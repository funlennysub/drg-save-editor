use std::{
    error,
    fs::{self, File},
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

use super::{
    create_write_pretty, get_res_amount, u8_to_string, Dwarf, ImportNoIdx, OverclockType, Schematic,
};

/*
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_Warmonger.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_TrustyRusty.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_ToolOfDestruction.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_PrimalBlood.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_MintAssault.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_MetallicVintage.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_JungleRaid.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_GhostlyPale.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_DigitalDanger.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_DesertRanger.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_DarkDescent.json
FSD\Content\WeaponsNTools\_GlobalSkins\SKN_GlobalSkin_BeyondTheCurcuit.json

FSD\Content\GameElements\Schematics\ResourceSchematics\SCE_Umanite.json
FSD\Content\GameElements\Schematics\ResourceSchematics\SCE_Magnite.json
FSD\Content\GameElements\Schematics\ResourceSchematics\SCE_Enor.json
FSD\Content\GameElements\Schematics\ResourceSchematics\SCE_Jadiz.json
FSD\Content\GameElements\Schematics\ResourceSchematics\SCE_Croppa.json
FSD\Content\GameElements\Schematics\ResourceSchematics\SCE_Bismor.json

FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_SwarmerStomp.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_FallingBarrel.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_TheatricalBow.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_CaveRaider.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_DualMugDrop.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_Kisses.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_HulkFlex.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_Pennywise.json
FSD\Content\Character\Vanity2\VictoryPoses\Released\VP_CrystalLover.json

FSD\Content\Character\Vanity2\Sideburns\VSB_Sideburns.json

FSD\Content\Character\Vanity2\Moustaches\VSB_Moustaches.json

FSD\Content\Character\Vanity2\Headwear\VSB_Headwear.json

FSD\Content\Character\Vanity2\Beards\VSB_Beards.json
*/

#[derive(Debug, Clone)]
pub(crate) struct CoresCommand {
    pub(crate) asset_dir: PathBuf,
    pub(crate) out_dir: PathBuf,
    matrix_cores: Vec<Schematic>,
}

impl CoresCommand {
    pub(crate) fn new(asset_dir: PathBuf, out_dir: PathBuf) -> Self {
        Self {
            asset_dir,
            out_dir,
            matrix_cores: Vec::new(),
        }
    }

    pub(crate) fn run(&mut self) {
        let res = &self.inner();
        if res.is_err() {
            eprintln!("Error getting information: {:?}", res.as_ref().unwrap_err());
            exit(1);
        }
        let res = res.as_ref().unwrap();
        create_write_pretty(&self.out_dir, res);
    }

    fn inner(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.get_mineral_cores()?;
        self.get_cosmetic_cores()?;
        self.get_overclocks()?;

        dbg!(&self.matrix_cores);

        Ok(())
    }

    fn get_mineral_info(asset: &Asset<File>) -> Schematic {
        let exports = &asset
            .asset_data
            .exports
            .iter()
            .filter_map(|e| cast!(Export, NormalExport, &e))
            .collect::<Vec<_>>();

        let guid = exports
            .get(1)
            .map(|e| &e.properties)
            .and_then(|e| {
                e.iter()
                    .filter_map(|p| cast!(Property, StructProperty, p))
                    .find(|p| p.name.get_content() == "SaveGameID")
                    .map(|p| {
                        u8_to_string(cast!(Property, GuidProperty, &p.value[0]).unwrap().value)
                    })
            })
            .unwrap();
        let resource = exports
            .get(0)
            .map(|e| &e.properties)
            .map(|e| {
                let amount = e
                    .iter()
                    .find_map(|p| cast!(Property, IntProperty, &p))
                    .map(|p| p.value)
                    .unwrap();
                e.iter()
                    .find_map(|p| cast!(Property, ObjectProperty, &p))
                    .map(|p| {
                        let name = asset.get_import(p.value).unwrap().object_name.get_content();
                        get_res_amount(&name, amount)
                    })
                    .unwrap()
            })
            .unwrap();

        Schematic::Mineral { guid, resource }
    }

    fn get_mineral_cores(&mut self) -> Result<(), Box<dyn error::Error>> {
        let resource_schematics_dir = &self
            .asset_dir
            .join("FSD/Content/GameElements/Schematics/ResourceSchematics");

        let (bismor, bismor_uexp) = (
            File::open(resource_schematics_dir.join("SCE_Bismor.uasset"))?,
            File::open(resource_schematics_dir.join("SCE_Bismor.uexp"))?,
        );
        let (croppa, croppa_uexp) = (
            File::open(resource_schematics_dir.join("SCE_Croppa.uasset"))?,
            File::open(resource_schematics_dir.join("SCE_Croppa.uexp"))?,
        );
        let (enor, enor_uexp) = (
            File::open(resource_schematics_dir.join("SCE_Enor.uasset"))?,
            File::open(resource_schematics_dir.join("SCE_Enor.uexp"))?,
        );
        let (jadiz, jadiz_uexp) = (
            File::open(resource_schematics_dir.join("SCE_Jadiz.uasset"))?,
            File::open(resource_schematics_dir.join("SCE_Jadiz.uexp"))?,
        );
        let (magnite, magnite_uexp) = (
            File::open(resource_schematics_dir.join("SCE_Magnite.uasset"))?,
            File::open(resource_schematics_dir.join("SCE_Magnite.uexp"))?,
        );
        let (umanite, umanite_uexp) = (
            File::open(resource_schematics_dir.join("SCE_Umanite.uasset"))?,
            File::open(resource_schematics_dir.join("SCE_Umanite.uexp"))?,
        );

        let bismor = Asset::new(bismor, Some(bismor_uexp), EngineVersion::VER_UE4_27)?;
        let croppa = Asset::new(croppa, Some(croppa_uexp), EngineVersion::VER_UE4_27)?;
        let enor = Asset::new(enor, Some(enor_uexp), EngineVersion::VER_UE4_27)?;
        let jadiz = Asset::new(jadiz, Some(jadiz_uexp), EngineVersion::VER_UE4_27)?;
        let magnite = Asset::new(magnite, Some(magnite_uexp), EngineVersion::VER_UE4_27)?;
        let umanite = Asset::new(umanite, Some(umanite_uexp), EngineVersion::VER_UE4_27)?;

        self.matrix_cores.push(Self::get_mineral_info(&bismor));
        self.matrix_cores.push(Self::get_mineral_info(&croppa));
        self.matrix_cores.push(Self::get_mineral_info(&enor));
        self.matrix_cores.push(Self::get_mineral_info(&jadiz));
        self.matrix_cores.push(Self::get_mineral_info(&magnite));
        self.matrix_cores.push(Self::get_mineral_info(&umanite));

        Ok(())
    }

    fn get_cosmetic_cores(&mut self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    fn get_overclocks(&mut self) -> Result<(), Box<dyn error::Error>> {
        let content_dir = &self.asset_dir.join("FSD").join("Content");
        let st_path = &content_dir.join("Game").join("Text");
        let st_uasset = File::open(st_path.join("ST_GearUpgrades.uasset"))?;
        let st_uexp = File::open(st_path.join("ST_GearUpgrades.uexp"))?;
        let st_asset = Asset::new(st_uasset, Some(st_uexp), EngineVersion::VER_UE4_27)?;
        let st = &cast!(Export, StringTableExport, &st_asset.asset_data.exports[0])
            .unwrap()
            .table;

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

                            get_res_amount(
                                &asset.get_import(what).unwrap().object_name.get_content(),
                                price,
                            )
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

                    self.matrix_cores.push(Schematic::Overclock {
                        name: name.unwrap().to_owned(),
                        cost: cost.try_into().unwrap(),
                        guid,
                        ty: oc_type,
                        dwarf,
                    });
                }
            }
        }
        Ok(())
    }
}
