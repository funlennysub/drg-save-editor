use std::collections::HashMap;

use gvas::{
    properties::{array_property::ArrayProperty, Property},
    GvasFile,
};

use crate::{
    error::{Error, ParsingError},
    get,
    registry::{Schematic, Status, SCHEMATICS},
};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Forge {
    pub owned_schematics: HashMap<[u8; 16], Schematic>,
    pub forged_schematics: HashMap<[u8; 16], Schematic>,
}

impl Forge {
    pub(crate) fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        Ok(Self {
            owned_schematics: Self::get_owned(gvas)?,
            forged_schematics: Self::get_forged(gvas)?,
        })
    }

    fn get_schematic_save(gvas: &GvasFile) -> Result<&Vec<(String, Property)>, Error> {
        let properties = &gvas.properties;
        let property = get!(properties, "SchematicSave", StructProperty)?;
        let schematic_save = property
            .value
            .get_custom_struct()
            .ok_or_else(|| ParsingError::failed_cast("CustomStruct"))?;

        Ok(schematic_save.1)
    }

    fn get_owned(gvas: &GvasFile) -> Result<HashMap<[u8; 16], Schematic>, Error> {
        let schematic_save = Self::get_schematic_save(gvas)?;
        let properties = &get_array(schematic_save, "OwnedSchematics".to_owned())?.properties;

        let mut schematics = HashMap::new();
        for property in properties {
            let property = property
                .get_struct()
                .ok_or_else(|| ParsingError::failed_cast("StructProperty"))?;
            let schematic = property
                .value
                .get_guid()
                .ok_or_else(|| ParsingError::failed_cast("Guid"))?;

            if let Some(c) = SCHEMATICS.get(&schematic.0) {
                let mut schematic = c.to_owned();
                schematic.set_status(Some(Status::Unforged));

                schematics.insert(c.get_guid().0, schematic);
            }
        }

        Ok(schematics)
    }

    fn get_forged(gvas: &GvasFile) -> Result<HashMap<[u8; 16], Schematic>, Error> {
        let schematic_save = Self::get_schematic_save(gvas)?;
        let properties = &get_array(schematic_save, "ForgedSchematics".to_owned())?.properties;

        let mut schematics = HashMap::new();
        for property in properties {
            let property = property
                .get_struct()
                .ok_or_else(|| ParsingError::failed_cast("StructProperty"))?;
            let schematic = property
                .value
                .get_guid()
                .ok_or_else(|| ParsingError::failed_cast("Guid"))?;

            if let Some(c) = SCHEMATICS.get(&schematic.0) {
                let mut schematic = c.to_owned();
                schematic.set_status(Some(Status::Forged));

                schematics.insert(c.get_guid().0, schematic);
            }
        }

        Ok(schematics)
    }
}

fn get_array(
    schematic_save: &Vec<(String, Property)>,
    prop: String,
) -> Result<&ArrayProperty, Error> {
    Ok(schematic_save
        .iter()
        .find_map(|p| match p.0 == prop {
            true => Some(p.1.get_array()),
            false => None,
        })
        .ok_or_else(|| ParsingError::missing_entry(&prop))?
        .ok_or_else(|| ParsingError::failed_cast("ArrayProperty"))?)
}
