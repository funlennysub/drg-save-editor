use std::{
    collections::HashMap,
    error,
    fs::{self, File},
    path::PathBuf,
    process::exit,
};

use regex::Regex;
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
    create_write_pretty, get_crafting_cost, get_res_amount, get_savegame_id, CosmeticType, Dwarf,
    ImportNoIdx, OverclockType, Schematic,
};

#[derive(Debug, Clone)]
pub(crate) struct CoresCommand {
    pub(crate) asset_dir: PathBuf,
    pub(crate) out_dir: PathBuf,
    matrix_cores: Vec<Schematic>,
    sid_map: HashMap<String, (String, Option<Dwarf>)>,
}

impl CoresCommand {
    pub(crate) fn new(asset_dir: PathBuf, out_dir: PathBuf) -> Self {
        Self {
            asset_dir,
            out_dir,
            matrix_cores: Vec::new(),
            sid_map: HashMap::new(),
        }
    }

    pub(crate) fn run(&mut self) {
        let res = &self.inner();
        if res.is_err() {
            eprintln!("Error getting information: {:?}", res.as_ref().unwrap_err());
            exit(1);
        }
        create_write_pretty(&self.out_dir, &self.matrix_cores);
    }

    fn inner(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.generate_sid_map()?;

        self.get_mineral_cores()?;
        self.get_cosmetic_cores()?;
        self.get_overclocks()?;

        Ok(())
    }

    fn generate_sid_map(&mut self) -> Result<(), Box<dyn error::Error>> {
        let analytics_dir = &self.asset_dir.join("FSD/Content/Game/Analytics");

        let data_file =
            File::open(&analytics_dir.join("Google_Analytics_Schematic_ID_mapping.uasset"))?;
        let bulk_file =
            File::open(&analytics_dir.join("Google_Analytics_Schematic_ID_mapping.uexp"))?;
        let asset = Asset::new(data_file, Some(bulk_file), EngineVersion::VER_UE4_27)?;
        let exports = cast!(Export, DataTableExport, &asset.asset_data.exports[0]).unwrap();

        let guid_re = Regex::new("([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})-([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})-([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})-([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})")?;
        let class_name_re = Regex::new("(?P<name>.*) - (?P<class>.*)")?;

        for export in &exports.table.data {
            let key = export.name.get_content();
            let guid = guid_re
                .replace_all(&key, "$4$3$2$1-$8$7$6$5-$12$11$10$9-$16$15$14$13")
                .to_string();
            let property = cast!(Property, StrProperty, &export.value[1]).unwrap();
            let name = property.value.clone().unwrap();

            let caps = class_name_re.captures(&name);
            if caps.is_none() {
                self.sid_map.insert(guid, (name, None));
                continue;
            }
            let caps = caps.unwrap();
            let name = caps.name("name").unwrap().as_str().to_owned();
            let class = caps.name("class").map(|m| m.as_str()).map(|c| match c {
                "Driller" => Dwarf::Driller,
                "Gunner" => Dwarf::Gunner,
                "Scout" => Dwarf::Scout,
                "Engineer" => Dwarf::Engineer,
                cl => todo!("{}", cl),
            });
            self.sid_map.insert(guid, (name, class));
        }
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
            .and_then(|e| get_savegame_id(&e.properties))
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

    fn get_cosmetics(
        &mut self,
        path: &str,
        file_name: &str,
        ty: CosmeticType,
    ) -> Result<(), Box<dyn error::Error>> {
        let dir = &self.asset_dir.join(path);
        let (cosmetics, uexp) = (
            File::open(&dir.join(format!("{file_name}.uasset")))?,
            File::open(&dir.join(format!("{file_name}.uexp")))?,
        );

        let asset = Asset::new(cosmetics, Some(uexp), EngineVersion::VER_UE4_27)?;
        let schematic_index = asset
            .find_import_no_index_by_content(
                &FName::from_slice("/Script/CoreUObject"),
                &FName::from_slice("Class"),
                &FName::from_slice("Schematic"),
            )
            .unwrap();

        for export in asset
            .asset_data
            .exports
            .iter()
            .filter(|e| e.get_base_export().class_index.index == schematic_index)
        {
            let props = &export.get_normal_export().unwrap().properties;

            let sid = get_savegame_id(props);
            if sid.is_none() {
                continue;
            }
            let guid = sid.unwrap();
            let cost = get_crafting_cost(props, &asset);
            let (name, dwarf) = self
                .sid_map
                .get(&guid)
                .map(|(n, d)| (n.to_owned(), d.to_owned()))
                .unwrap();
            let dwarf = dwarf.unwrap();
            self.matrix_cores.push(Schematic::Cosmetic {
                guid,
                name,
                cost,
                dwarf,
                ty: ty.clone(),
            })
        }

        Ok(())
    }

    fn get_cosmetic_cores(&mut self) -> Result<(), Box<dyn error::Error>> {
        for (path, name, ty) in vec![
            (
                r"FSD\Content\Character\Vanity2\Beards",
                "VSB_Beards",
                CosmeticType::Beard,
            ),
            (
                r"FSD\Content\Character\Vanity2\Headwear",
                "VSB_Headwear",
                CosmeticType::Headwear,
            ),
            (
                r"FSD\Content\Character\Vanity2\Moustaches",
                "VSB_Moustaches",
                CosmeticType::Moustache,
            ),
            (
                r"FSD\Content\Character\Vanity2\Sideburns",
                "VSB_Sideburns",
                CosmeticType::Sideburns,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_SwarmerStomp",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_FallingBarrel",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_TheatricalBow",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_CaveRaider",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_DualMugDrop",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_Kisses",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_HulkFlex",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_Pennywise",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\Character\Vanity2\VictoryPoses\Released",
                "VP_CrystalLover",
                CosmeticType::VictoryMove,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_Warmonger",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_TrustyRusty",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_ToolOfDestruction",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_PrimalBlood",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_MintAssault",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_MetallicVintage",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_JungleRaid",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_GhostlyPale",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_DigitalDanger",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_DesertRanger",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_DarkDescent",
                CosmeticType::PaintJob,
            ),
            (
                r"FSD\Content\WeaponsNTools\_GlobalSkins",
                "SKN_GlobalSkin_BeyondTheCurcuit",
                CosmeticType::PaintJob,
            ),
        ] {
            self.get_cosmetics(path, name, ty)?;
        }

        Ok(())
    }

    fn get_overclocks(&mut self) -> Result<(), Box<dyn error::Error>> {
        let content_dir = &self.asset_dir.join("FSD").join("Content");

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

                    let ty = properties
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

                    let guid = get_savegame_id(properties).unwrap();

                    let cost = get_crafting_cost(properties, &asset);
                    let (name, dwarf) = self
                        .sid_map
                        .get(&guid)
                        .map(|(n, d)| (n.to_owned(), d.to_owned()))
                        .unwrap();
                    let dwarf = dwarf.unwrap();

                    self.matrix_cores.push(Schematic::Overclock {
                        name,
                        cost,
                        guid,
                        ty,
                        dwarf,
                    });
                }
            }
        }
        Ok(())
    }
}
