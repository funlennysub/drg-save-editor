pub mod brewing;
pub mod dwarfs;
pub mod guids;
pub mod minerals;
pub mod miscellaneous;
pub mod forge;

use std::path::Path;

use gvas::{
    properties::{
        int_property::{FloatProperty, IntProperty},
        map_property::MapProperty,
        struct_property::{StructProperty, StructPropertyValue},
        Property,
    },
    types::Guid,
    GvasFile,
};
use indexmap::IndexMap;

use crate::{
    error::{Error, ParsingError},
    get_mut, get_resource_mut,
    save_file::guids::resources::*,
    write_gvas,
};

use self::{
    brewing::Brewing, dwarfs::Characters, guids::resources, minerals::Minerals,
    miscellaneous::Miscellaneous, forge::Forge,
};

#[derive(Debug, Default)]
pub struct SaveFile {
    pub minerals: Minerals,
    pub brewing: Brewing,
    pub miscellaneous: Miscellaneous,
    pub dwarfs: Characters,
    pub forge: Forge
}

impl SaveFile {
    pub fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        Ok(Self {
            minerals: Minerals::from_gvas(gvas)?,
            brewing: Brewing::from_gvas(gvas)?,
            miscellaneous: Miscellaneous::from_gvas(gvas)?,
            dwarfs: Characters::from_gvas(gvas)?,
        })
    }

    pub fn save(&self, gvas: &mut GvasFile, path: &Path) -> Result<(), Error> {
        let Self {
            minerals,
            brewing,
            miscellaneous,
            dwarfs
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
        *get_resource_mut!(resources, JADIZ)? =
            Property::from(FloatProperty::new(minerals.jadiz));
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

        write_gvas(path, gvas)
    }
}
