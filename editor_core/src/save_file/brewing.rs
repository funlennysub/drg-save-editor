use gvas::GvasFile;

use crate::{
    error::{Error, ParsingError},
    get, get_owned_resources,
};

use super::guids::resources::{BARLEY_BULB, MALT_STAR, STARCH_NUT, YEAST_CONE};

#[derive(Debug, Clone, Copy, Default)]
pub struct Brewing {
    pub starch_nut: f32,
    pub yeast_cone: f32,
    pub malt_star: f32,
    pub barley_bulb: f32,
}

impl Brewing {
    pub fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        let owned_resources = get_owned_resources(gvas)?;

        let starch_nut = get!(owned_resources, &STARCH_NUT)?.value.0;
        let yeast_cone = get!(owned_resources, &YEAST_CONE)?.value.0;
        let malt_star = get!(owned_resources, &MALT_STAR)?.value.0;
        let barley_bulb = get!(owned_resources, &BARLEY_BULB)?.value.0;

        Ok(Self {
            starch_nut,
            yeast_cone,
            malt_star,
            barley_bulb,
        })
    }
}
