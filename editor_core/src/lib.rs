use crate::error::{Error, ParsingError};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions}, path::Path,
};

use gvas::{
    properties::{
        self, int_property::FloatProperty, struct_property::StructPropertyValue, Property,
    },
    types::Guid,
    GvasFile,
};

pub mod error;
pub mod save_file;

// pub trait GetCustom {
//     fn get_custom(&self) -> Option<(&String, &Vec<(String, Property)>)>;
//     fn get_custom_mut(&mut self) -> Option<(&mut String, &mut Vec<(String, Property)>)>;
// }

// impl GetCustom for StructPropertyValue {
//     fn get_custom(&self) -> Option<(&String, &Vec<(String, Property)>)> {
//         match self {
//             StructPropertyValue::CustomStruct(name, properties) => Some((name, properties)),
//             _ => None,
//         }
//     }

//     fn get_custom_mut(&mut self) -> Option<(&mut String, &mut Vec<(String, Property)>)> {
//         match self {
//             StructPropertyValue::CustomStruct(name, properties) => Some((name, properties)),
//             _ => None,
//         }
//     }
// }

fn get_hints() -> HashMap<String, String> {
    let mut hints = HashMap::new();
    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );

    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty.CompletedSpecialChallenges.MapProperty.Key.StructProperty".to_string(), 
        "Guid".to_string()
    );

    hints.insert(
        "UnLockedMissionParameters.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "UnLockedMissionParameters.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );

    hints.insert(
        "ItemUpgradeSelections.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "ItemUpgradeSelections.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );

    hints.insert(
        "ItemUpgradeLoadouts.ArrayProperty.Loadout.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "ItemUpgradeLoadouts.ArrayProperty.Loadout.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );

    hints.insert(
        "EnemiesKilled.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "UnlockedItemSkins.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnlockedItemSkins.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );

    hints.insert(
        "Resources.StructProperty.OwnedResources.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "FSDEventRewardsSave.StructProperty.EventsSeen.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "GameDLCSave.StructProperty.AnnouncedIDs.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "Drinks.StructProperty.UnlockedDrinks.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "UnlockedItemSkins.MapProperty.Value.StructProperty.Skins.SetProperty.StructProperty"
            .to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "UnlockedPickaxeParts.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "MinersManualKnownObjects.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "FSDEventRewardsSave.StructProperty.EventsSeen.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "FSDEventRewardsSave.StructProperty.PopupsSeen.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints
}

pub fn read_gvas(path: &Path) -> Result<GvasFile, Error> {
    let mut file = File::open(path)?;

    let hints = get_hints();
    Ok(GvasFile::read_with_hints(&mut file, &hints)?)
}

pub fn gvas_json(gvas: &GvasFile) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(gvas)
}

pub fn write_gvas(path: &Path, gvas: &GvasFile) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)?;
    gvas.write(&mut file)?;

    Ok(())
}

pub fn get_owned_resources(gvas: &GvasFile) -> Result<HashMap<&Guid, &FloatProperty>, Error> {
    let props = &gvas.properties;
    let cs_resources = get!(props, "Resources", StructProperty)?
        .value
        .get_custom_struct()
        .ok_or_else(|| ParsingError::failed_cast("CustomStruct"))?;
    let owned_resources = &cs_resources.1[0].1;
    let owned_resources = &owned_resources
        .get_map()
        .ok_or_else(|| ParsingError::failed_cast("MapProperty"))?
        .value;

    let mut resources = HashMap::new();
    for or in owned_resources {
        let (k, v) = match or {
            (Property::StructProperty(k), Property::FloatProperty(v)) => Ok((
                k.value
                    .get_guid()
                    .ok_or_else(|| ParsingError::failed_cast("Guid"))?,
                v,
            )),
            (Property::StructProperty(_), _) => Err(ParsingError::failed_cast("FloatProperty")),
            _ => Err(ParsingError::failed_cast("StructProperty")),
        }?;
        resources.insert(k, v);
    }

    Ok(resources)
}

pub fn get_owned_resources_mut(
    gvas: &mut GvasFile,
) -> Result<HashMap<&Guid, &FloatProperty>, Error> {
    let props = &mut gvas.properties;
    let cs_resources = get_mut!(props, "Resources", StructProperty)?
        .value
        .get_custom_struct()
        .ok_or_else(|| ParsingError::failed_cast("CustomStruct"))?;
    let owned_resources = &cs_resources.1[0].1;
    let owned_resources = &owned_resources
        .get_map()
        .ok_or_else(|| ParsingError::failed_cast("MapProperty"))?
        .value;

    let mut resources = HashMap::new();
    for or in owned_resources {
        let (k, v) = match or {
            (Property::StructProperty(k), Property::FloatProperty(v)) => Ok((
                k.value
                    .get_guid()
                    .ok_or_else(|| ParsingError::failed_cast("Guid"))?,
                v,
            )),
            (Property::StructProperty(_), _) => Err(ParsingError::failed_cast("FloatProperty")),
            _ => Err(ParsingError::failed_cast("StructProperty")),
        }?;
        resources.insert(k, v);
    }

    Ok(resources)
}

#[macro_export]
macro_rules! get {
    ($props:expr, $prop_name:expr, $prop_class:ident) => {
        match $props
            .get($prop_name)
            .ok_or_else(|| ParsingError::missing_entry($prop_name))?
        {
            Property::$prop_class(p) => Ok(p),
            _ => Err(ParsingError::failed_cast(stringify!($prop_class))),
        }
    };
    ($res:expr, $guid:expr) => {
        $res.get($guid)
            .ok_or_else(|| ParsingError::missing_entry(&$guid.to_string()))
    };
}

#[macro_export]
macro_rules! get_mut {
    ($props:expr, $prop_name:expr, $prop_class:ident) => {
        match $props
            .get_mut($prop_name)
            .ok_or_else(|| ParsingError::missing_entry($prop_name))?
        {
            Property::$prop_class(p) => Ok(p),
            _ => Err(ParsingError::failed_cast(stringify!($prop_class))),
        }
    };
    ($res:expr, $guid:expr) => {
        $res.get_mut($guid)
            .ok_or_else(|| ParsingError::missing_entry(&$guid.to_string()))
    };
}

#[macro_export]
macro_rules! get_resource_mut {
    ($map:expr, $guid:expr) => {
        $map.get_mut(&Property::from(StructProperty::new(
            Guid::from(0),
            StructPropertyValue::Guid($guid),
        )))
        .ok_or_else(|| ParsingError::missing_entry(&$guid.to_string()))
    };
}
