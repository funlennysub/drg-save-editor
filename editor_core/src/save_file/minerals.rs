use gvas::GvasFile;

use crate::{
    error::{Error, ParsingError},
    get, get_owned_resources,
};

use crate::registry::{BISMOR, CROPPA, ENOR_PEARL, JADIZ, MAGNITE, UMANITE};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Minerals {
    pub magnite: f32,
    pub bismor: f32,
    pub croppa: f32,
    pub umanite: f32,
    pub jadiz: f32,
    pub enor_pearl: f32,
}

// #[cfg(feature = "field_access")]
pub enum Mineral {
    Magnite,
    Bismor,
    Croppa,
    Umanite,
    Jadiz,
    EnorPearl,
}

impl Minerals {
    pub(crate) fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
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

    // #[cfg(feature = "field_access")]
    pub fn get(&self, field: Mineral) -> f32 {
        match field {
            Mineral::Magnite => self.magnite,
            Mineral::Bismor => self.bismor,
            Mineral::Croppa => self.croppa,
            Mineral::Umanite => self.umanite,
            Mineral::Jadiz => self.jadiz,
            Mineral::EnorPearl => self.enor_pearl,
        }
    }

    pub fn set(&mut self, field: Mineral, value: f32) {
        match field {
            Mineral::Magnite => self.magnite = value,
            Mineral::Bismor => self.bismor = value,
            Mineral::Croppa => self.croppa = value,
            Mineral::Umanite => self.umanite = value,
            Mineral::Jadiz => self.jadiz = value,
            Mineral::EnorPearl => self.enor_pearl = value,
        }
    }
}
