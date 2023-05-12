use gvas::{properties::Property, GvasFile};

use crate::{
    error::{Error, ParsingError},
    get, get_owned_resources,
};

use super::guids::resources::{
    BLANK_CORES, DATA_CELLS, ERROR_CUBES, PHAZYONITE,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Miscellaneous {
    pub credits: i32,
    pub perk_points: i32,
    pub error_cubes: f32,
    pub data_cells: f32,
    pub blank_cores: f32,
    pub phazyonite: f32,
}

impl Miscellaneous {
    pub fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        let props = &gvas.properties;
        let owned_resources = get_owned_resources(gvas)?;

        let credits = get!(props, "Credits", IntProperty)?.value;
        let perk_points = get!(props, "PerkPoints", IntProperty)?.value;

        let error_cubes = get!(owned_resources, &ERROR_CUBES)?.value.0;
        let data_cells = get!(owned_resources, &DATA_CELLS)?.value.0;
        let blank_cores = get!(owned_resources, &BLANK_CORES)?.value.0;
        let phazyonite = get!(owned_resources, &PHAZYONITE)?.value.0;

        Ok(Self {
            credits,
            perk_points,
            error_cubes,
            data_cells,
            blank_cores,
            phazyonite,
        })
    }
}
