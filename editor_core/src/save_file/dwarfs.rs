use gvas::{properties::Property, GvasFile};

use crate::{
    error::{Error, ParsingError},
    get,
    save_file::guids::dwarfs::{DRILLER, ENGINEER, GUNNER, SCOUT},
};

const PROMOTIONS: [&str; 20] = [
    "None",
    "Bronze 1",
    "Bronze 2",
    "Bronze 3",
    "Silver 1",
    "Silver 2",
    "Silver 3",
    "Gold 1",
    "Gold 2",
    "Gold 3",
    "Platinum 1",
    "Platinum 2",
    "Platinum 3",
    "Diamond 1",
    "Diamond 2",
    "Diamond 3",
    "Legendary 1",
    "Legendary 2",
    "Legendary 3",
    "Legendary 3+",
];

const XP_TABLE: [i32; 25] = [
    0, 3000, 7000, 12000, 18000, 25000, 33000, 42000, 52000, 63000, 75000, 88000, 102000, 117000,
    132500, 148500, 165000, 182000, 199500, 217500, 236000, 255000, 274500, 294500, 315000,
];

const MAX_LEVEL: i32 = 25;

#[derive(Debug, PartialEq, Eq)]
pub enum Dwarf {
    Engineer,
    Gunner,
    Driller,
    Scout,
}

#[derive(Debug, Default, Clone)]
pub struct Rank {
    pub xp: i32,
    pub times_retired: i32,
    pub promotion: String,
}

impl Rank {
    pub fn new(xp: i32, times_retired: i32) -> Self {
        let promotion = if times_retired > 18 {
            PROMOTIONS.last().unwrap()
        } else {
            PROMOTIONS[times_retired as usize]
        };

        Self {
            xp,
            times_retired,
            promotion: promotion.to_string(),
        }
    }

    pub fn xp_to_level(&self) -> (i32, i32) {
        for (idx, _) in XP_TABLE.iter().enumerate() {
            if self.xp < XP_TABLE[idx] {
                return (idx as i32, self.xp - XP_TABLE[idx - 1]);
            }
        }

        (MAX_LEVEL, 0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Characters {
    pub engineer: Rank,
    pub driller: Rank,
    pub gunner: Rank,
    pub scout: Rank,
}

impl Characters {
    pub fn from_gvas(gvas: &GvasFile) -> Result<Self, Error> {
        let Self {
            mut engineer,
            mut driller,
            mut gunner,
            mut scout,
        } = Self::default();

        let props = &gvas.properties;
        let character_save = get!(props, "CharacterSaves", ArrayProperty)?;
        character_save
            .properties
            .iter()
            .filter_map(|p| p.get_struct())
            .filter_map(|p| p.value.get_custom_struct())
            .map(|p| p.1)
            .map(|p| {
                p.iter()
                    .filter(|f| f.0 == "SavegameID" || f.0 == "XP" || f.0 == "TimesRetired")
                    .collect::<Vec<_>>()
            })
            .for_each(|d| {
                let guid = d[0].1.get_struct().unwrap().value.get_guid().unwrap();
                let xp = d[1].1.get_int().unwrap();
                let times_retired = d[2].1.get_int().unwrap();

                let rank = Rank::new(xp.value, times_retired.value);
                match *guid {
                    ENGINEER => engineer = rank,
                    DRILLER => driller = rank,
                    GUNNER => gunner = rank,
                    SCOUT => scout = rank,
                    _ => (),
                }
            });

        Ok(Self {
            engineer,
            driller,
            gunner,
            scout,
        })
    }
}
