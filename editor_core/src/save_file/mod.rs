pub mod brewing;
pub mod dwarfs;
pub mod forge;
pub mod minerals;
pub mod miscellaneous;

#[cfg(not(feature = "wasm"))]
use std::fs::File;
use std::{io::Cursor, path::Path};

use gvas::{
    properties::{
        int_property::{FloatProperty, IntProperty},
        struct_property::{StructProperty, StructPropertyValue},
        Property,
    },
    types::Guid,
    GvasFile,
};

use crate::{
    error::{Error, ParsingError},
    get_mut, get_resource_mut,
    registry::{
        get_hints, BARLEY_BULB, BISMOR, BLANK_CORES, CROPPA, DATA_CELLS, ENOR_PEARL, ERROR_CUBES,
        JADIZ, MAGNITE, MALT_STAR, PHAZYONITE, STARCH_NUT, UMANITE, YEAST_CONE,
    },
};

use self::{
    brewing::Brewing, dwarfs::Characters, forge::Forge, minerals::Minerals,
    miscellaneous::Miscellaneous,
};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SaveFile {
    pub minerals: Minerals,
    pub brewing: Brewing,
    pub miscellaneous: Miscellaneous,
    pub dwarfs: Characters,
    pub forge: Forge,
}

impl SaveFile {
    #[cfg(not(feature = "wasm"))]
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let gvas = GvasFile::read_with_hints(&mut file, &get_hints())?;

        Ok(Self::from_gvas(&gvas)?)
    }

    pub fn from_bytes_rs(bytes: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(bytes);
        let gvas = GvasFile::read_with_hints(&mut cursor, &get_hints())?;

        Ok(Self::from_gvas(&gvas)?)
    }

    fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        Ok(Self {
            minerals: Minerals::from_gvas(gvas)?,
            brewing: Brewing::from_gvas(gvas)?,
            miscellaneous: Miscellaneous::from_gvas(gvas)?,
            dwarfs: Characters::from_gvas(gvas)?,
            forge: Forge::default(),
        })
    }

    pub fn save(&self, gvas: &mut GvasFile, _path: &Path) -> Result<(), Error> {
        let Self {
            minerals,
            brewing,
            miscellaneous,
            dwarfs: _,
            forge: _,
        } = self;

        let props = &mut gvas.properties;
        let resources = get_mut!(props, "Resources", StructProperty)?;
        let resources = &mut resources
            .value
            .get_custom_struct_mut()
            .ok_or_else(|| ParsingError::failed_cast("CustomStruct"))?
            .1[0]
            .1
            .get_map_mut()
            .ok_or_else(|| ParsingError::failed_cast("MapProperty"))?
            .value;

        *get_resource_mut!(resources, MAGNITE)? =
            Property::from(FloatProperty::new(minerals.magnite));
        *get_resource_mut!(resources, BISMOR)? =
            Property::from(FloatProperty::new(minerals.bismor));
        *get_resource_mut!(resources, CROPPA)? =
            Property::from(FloatProperty::new(minerals.croppa));
        *get_resource_mut!(resources, UMANITE)? =
            Property::from(FloatProperty::new(minerals.umanite));
        *get_resource_mut!(resources, JADIZ)? = Property::from(FloatProperty::new(minerals.jadiz));
        *get_resource_mut!(resources, ENOR_PEARL)? =
            Property::from(FloatProperty::new(minerals.enor_pearl));

        *get_resource_mut!(resources, STARCH_NUT)? =
            Property::from(FloatProperty::new(brewing.starch_nut));
        *get_resource_mut!(resources, YEAST_CONE)? =
            Property::from(FloatProperty::new(brewing.yeast_cone));
        *get_resource_mut!(resources, MALT_STAR)? =
            Property::from(FloatProperty::new(brewing.malt_star));
        *get_resource_mut!(resources, BARLEY_BULB)? =
            Property::from(FloatProperty::new(brewing.barley_bulb));

        *get_resource_mut!(resources, ERROR_CUBES)? =
            Property::from(FloatProperty::new(miscellaneous.error_cubes));
        *get_resource_mut!(resources, DATA_CELLS)? =
            Property::from(FloatProperty::new(miscellaneous.data_cells));
        *get_resource_mut!(resources, BLANK_CORES)? =
            Property::from(FloatProperty::new(miscellaneous.blank_cores));
        *get_resource_mut!(resources, PHAZYONITE)? =
            Property::from(FloatProperty::new(miscellaneous.phazyonite));

        *get_mut!(props, "Credits", IntProperty)? = IntProperty::new(miscellaneous.credits);
        *get_mut!(props, "PerkPoints", IntProperty)? = IntProperty::new(miscellaneous.perk_points);

        // write_gvas(path, gvas)
        Ok(())
    }
}
