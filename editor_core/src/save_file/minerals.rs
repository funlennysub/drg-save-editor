use gvas::GvasFile;

use crate::{
    error::{Error, ParsingError},
    get, get_owned_resources,
};

use super::guids::resources::{BISMOR, CROPPA, ENOR_PEARL, JADIZ, MAGNITE, UMANITE};

#[derive(Debug, Clone, Copy, Default)]
pub struct Minerals {
    pub magnite: f32,
    pub bismor: f32,
    pub croppa: f32,
    pub umanite: f32,
    pub jadiz: f32,
    pub enor_pearl: f32,
}

impl Minerals {
    pub fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        let owned_resources = get_owned_resources(gvas)?;

        let magnite = get!(owned_resources, &MAGNITE)?.value.0;
        let bismor = get!(owned_resources, &BISMOR)?.value.0;
        let croppa = get!(owned_resources, &CROPPA)?.value.0;
        let umanite = get!(owned_resources, &UMANITE)?.value.0;
        let jadiz = get!(owned_resources, &JADIZ)?.value.0;
        let enor_pearl = get!(owned_resources, &ENOR_PEARL)?.value.0;

        Ok(Self {
            magnite,
            bismor,
            croppa,
            umanite,
            jadiz,
            enor_pearl,
        })
    }
}
