use crate::error::{Error, ParsingError};
use std::collections::HashMap;

use gvas::{
    properties::{int_property::FloatProperty, Property},
    types::Guid,
    GvasFile,
};

pub mod error;
pub mod registry;
pub mod save_file;

pub fn gvas_json(gvas: &GvasFile) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(gvas)
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
