use std::collections::HashMap;

use crate::registry::{CosmeticType::*, Dwarf::*, OverclockType::*, Resource::*, Schematic::*};

use gvas::types::Guid;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Resource {
    Credits,
    Croppa,
    Umanite,
    Bismor,
    Jadiz,
    Magnite,
    EnorPearl,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OverclockType {
    Clean,
    Balanced,
    Unstable,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CosmeticType {
    Headwear,
    Moustache,
    Beard,
    Sideburns,
    VictoryMove,
    PaintJob,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Status {
    Forged,
    Unforged,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Schematic {
    Overclock {
        name: String,
        guid: Guid,
        dwarf: Dwarf,
        ty: OverclockType,
        status: Option<Status>,
    },
    Cosmetic {
        name: String,
        guid: Guid,
        dwarf: Dwarf,
        ty: CosmeticType,
        status: Option<Status>,
    },
    Mineral {
        guid: Guid,
        resource: Resource,
        status: Option<Status>,
    },
}

impl Schematic {
    pub fn set_status(&mut self, new: Option<Status>) {
        *match self {
            Overclock { status, .. } => status,
            Cosmetic { status, .. } => status,
            Mineral { status, .. } => status,
        } = new;
    }

    pub fn get_guid(&self) -> Guid {
        match *self {
            Overclock { guid, .. } => guid,
            Cosmetic { guid, .. } => guid,
            Mineral { guid, .. } => guid,
        }
    }
}

pub const DRILLER: Guid = Guid([
    0x9E, 0xDD, 0x56, 0xF1, 0xEE, 0xBC, 0xC5, 0x48, 0x8D, 0x5B, 0x5E, 0x5B, 0x80, 0xB6, 0x2D, 0xB4,
]);
pub const ENGINEER: Guid = Guid([
    0x85, 0xEF, 0x62, 0x6C, 0x65, 0xF1, 0x02, 0x4A, 0x8D, 0xFE, 0xB5, 0xD0, 0xF3, 0x90, 0x9D, 0x2E,
]);
pub const GUNNER: Guid = Guid([
    0xAE, 0x56, 0xE1, 0x80, 0xFE, 0xC0, 0xC4, 0x4D, 0x96, 0xFA, 0x29, 0xC2, 0x83, 0x66, 0xB9, 0x7B,
]);
pub const SCOUT: Guid = Guid([
    0x30, 0xD8, 0xEA, 0x17, 0xD8, 0xFB, 0xBA, 0x4C, 0x95, 0x30, 0x6D, 0xE9, 0x65, 0x5C, 0x2F, 0x8C,
]);

pub const MAGNITE: Guid = Guid([
    0xAA, 0xDE, 0xD8, 0x76, 0x6C, 0x22, 0x7D, 0x40, 0x80, 0x32, 0xAF, 0xD1, 0x8D, 0x63, 0x56, 0x1E,
]);
pub const BISMOR: Guid = Guid([
    0xAF, 0x0D, 0xC4, 0xFE, 0x83, 0x61, 0xBB, 0x48, 0xB3, 0x2C, 0x92, 0xCC, 0x97, 0xE2, 0x1D, 0xE7,
]);
pub const CROPPA: Guid = Guid([
    0x8A, 0xA7, 0xFB, 0x43, 0x29, 0x3A, 0x0B, 0x49, 0xB8, 0xBE, 0x42, 0xFF, 0xE0, 0x68, 0xA4, 0x4C,
]);
pub const UMANITE: Guid = Guid([
    0x5F, 0x2B, 0xCF, 0x83, 0x47, 0x76, 0x0A, 0x42, 0xA2, 0x3B, 0x6E, 0xDC, 0x07, 0xC0, 0x94, 0x1D,
]);
pub const JADIZ: Guid = Guid([
    0x22, 0xBC, 0x4F, 0x7D, 0x07, 0xD1, 0x3E, 0x43, 0xBF, 0xCA, 0x81, 0xBD, 0x9C, 0x14, 0xB1, 0xAF,
]);
pub const ENOR_PEARL: Guid = Guid([
    0x48, 0x8D, 0x05, 0x14, 0x6F, 0x5F, 0x75, 0x4B, 0xA3, 0xD4, 0x61, 0x0D, 0x08, 0xC0, 0x60, 0x3E,
]);

pub const STARCH_NUT: Guid = Guid([
    0x72, 0x31, 0x22, 0x04, 0xE2, 0x87, 0xBC, 0x41, 0x81, 0x55, 0x40, 0xA0, 0xCF, 0x88, 0x12, 0x80,
]);
pub const YEAST_CONE: Guid = Guid([
    0x07, 0x85, 0x48, 0xB9, 0x32, 0x32, 0xC0, 0x40, 0x85, 0xF8, 0x92, 0xE0, 0x84, 0xA7, 0x41, 0x00,
]);
pub const MALT_STAR: Guid = Guid([
    0x41, 0xEA, 0x55, 0x0C, 0x1D, 0x46, 0xC5, 0x4B, 0xBE, 0x2E, 0x9C, 0xA5, 0xA7, 0xAC, 0xCB, 0x06,
]);
pub const BARLEY_BULB: Guid = Guid([
    0x22, 0xDA, 0xA7, 0x57, 0xAD, 0x7A, 0x80, 0x49, 0x89, 0x1B, 0x17, 0xED, 0xCC, 0x2F, 0xE0, 0x98,
]);

pub const ERROR_CUBES: Guid = Guid([
    0x58, 0x28, 0x65, 0x2C, 0x9A, 0x5D, 0xE8, 0x45, 0xA9, 0xE2, 0xE1, 0xB8, 0xB4, 0x63, 0xC5, 0x16,
]);
pub const DATA_CELLS: Guid = Guid([
    0x99, 0xFA, 0x52, 0x6A, 0xD8, 0x77, 0x48, 0x45, 0x94, 0x98, 0x90, 0x5A, 0x27, 0x86, 0x93, 0xF6,
]);
pub const BLANK_CORES: Guid = Guid([
    0xA1, 0x0C, 0xB2, 0x85, 0x38, 0x71, 0xFB, 0x49, 0x9A, 0xC8, 0x54, 0xA1, 0xCD, 0xE2, 0x20, 0x2C,
]);
pub const PHAZYONITE: Guid = Guid([
    0x67, 0x66, 0x8A, 0xAE, 0x82, 0x8F, 0xDB, 0x48, 0xA9, 0x11, 0x1E, 0x1B, 0x91, 0x2D, 0xBF, 0xA4,
]);

pub const XP_TABLE: [i32; 25] = [
    0, 3000, 7000, 12000, 18000, 25000, 33000, 42000, 52000, 63000, 75000, 88000, 102000, 117000,
    132500, 148500, 165000, 182000, 199500, 217500, 236000, 255000, 274500, 294500, 315000,
];

pub const MAX_LEVEL: i32 = 25;
pub const MAX_F32: f32 = 268_435_456.0;
pub const MAX_I32: i32 = 268_435_456;

#[derive(Debug, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Dwarf {
    Engineer,
    Gunner,
    Driller,
    Scout,
}

pub(crate) fn get_hints() -> HashMap<String, String> {
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

lazy_static! {
    pub static ref PROMOTIONS: Vec<String> = vec![
        String::from("None"),
        String::from("Bronze 1"),
        String::from("Bronze 2"),
        String::from("Bronze 3"),
        String::from("Silver 1"),
        String::from("Silver 2"),
        String::from("Silver 3"),
        String::from("Gold 1"),
        String::from("Gold 2"),
        String::from("Gold 3"),
        String::from("Platinum 1"),
        String::from("Platinum 2"),
        String::from("Platinum 3"),
        String::from("Diamond 1"),
        String::from("Diamond 2"),
        String::from("Diamond 3"),
        String::from("Legendary 1"),
        String::from("Legendary 2"),
        String::from("Legendary 3"),
        String::from("Legendary 3+"),
    ];
    pub static ref SCHEMATICS: HashMap<[u8; 16], Schematic> = HashMap::from([
        (
            [199, 9, 209, 25, 27, 38, 0, 65, 170, 64, 8, 111, 229, 120, 193, 4],
            Mineral {
                guid: Guid([199, 9, 209, 25, 27, 38, 0, 65, 170, 64, 8, 111, 229, 120, 193, 4]),
                resource: Bismor,
                status: None
            }
        ),
        (
            [93, 215, 225, 98, 63, 9, 28, 70, 131, 67, 180, 38, 82, 135, 153, 176],
            Mineral {
                guid: Guid([93, 215, 225, 98, 63, 9, 28, 70, 131, 67, 180, 38, 82, 135, 153, 176]),
                resource: Croppa,
                status: None
            }
        ),
        (
            [201, 151, 165, 213, 148, 173, 54, 67, 169, 153, 144, 120, 166, 210, 26, 77],
            Mineral {
                guid: Guid([
                    201, 151, 165, 213, 148, 173, 54, 67, 169, 153, 144, 120, 166, 210, 26, 77
                ]),
                resource: EnorPearl,
                status: None
            }
        ),
        (
            [49, 77, 39, 219, 48, 42, 241, 65, 142, 217, 153, 89, 120, 150, 65, 69],
            Mineral {
                guid: Guid([49, 77, 39, 219, 48, 42, 241, 65, 142, 217, 153, 89, 120, 150, 65, 69]),
                resource: Jadiz,
                status: None
            }
        ),
        (
            [55, 243, 253, 50, 74, 92, 117, 75, 144, 213, 31, 144, 101, 80, 199, 86],
            Mineral {
                guid: Guid([
                    55, 243, 253, 50, 74, 92, 117, 75, 144, 213, 31, 144, 101, 80, 199, 86
                ]),
                resource: Magnite,
                status: None
            }
        ),
        (
            [167, 210, 155, 172, 77, 253, 65, 74, 138, 0, 125, 55, 34, 157, 26, 211],
            Mineral {
                guid: Guid([
                    167, 210, 155, 172, 77, 253, 65, 74, 138, 0, 125, 55, 34, 157, 26, 211
                ]),
                resource: Umanite,
                status: None
            }
        ),
        (
            [70, 169, 53, 46, 171, 90, 57, 76, 144, 113, 58, 160, 119, 72, 107, 90],
            Cosmetic {
                guid: Guid([70, 169, 53, 46, 171, 90, 57, 76, 144, 113, 58, 160, 119, 72, 107, 90]),
                name: String::from("Bound Goatee - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [119, 174, 60, 246, 255, 183, 75, 77, 149, 11, 83, 72, 251, 69, 128, 230],
            Cosmetic {
                guid: Guid([
                    119, 174, 60, 246, 255, 183, 75, 77, 149, 11, 83, 72, 251, 69, 128, 230
                ]),
                name: String::from("Bound Goatee - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [4, 28, 86, 84, 237, 233, 88, 77, 165, 217, 249, 15, 215, 142, 90, 0],
            Cosmetic {
                guid: Guid([4, 28, 86, 84, 237, 233, 88, 77, 165, 217, 249, 15, 215, 142, 90, 0]),
                name: String::from("Mighty Pelt"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [38, 184, 183, 222, 147, 33, 41, 68, 150, 75, 103, 31, 191, 22, 31, 37],
            Cosmetic {
                guid: Guid([38, 184, 183, 222, 147, 33, 41, 68, 150, 75, 103, 31, 191, 22, 31, 37]),
                name: String::from("Mighty Pelt - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [76, 204, 106, 135, 112, 173, 24, 74, 139, 179, 186, 226, 91, 165, 41, 50],
            Cosmetic {
                guid: Guid([
                    76, 204, 106, 135, 112, 173, 24, 74, 139, 179, 186, 226, 91, 165, 41, 50
                ]),
                name: String::from("Monumental Goatee"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [62, 160, 133, 25, 209, 146, 97, 65, 190, 139, 68, 80, 165, 7, 63, 150],
            Cosmetic {
                guid: Guid([62, 160, 133, 25, 209, 146, 97, 65, 190, 139, 68, 80, 165, 7, 63, 150]),
                name: String::from("Monumental Goatee - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [78, 134, 188, 248, 247, 144, 151, 76, 187, 104, 248, 236, 125, 158, 253, 225],
            Cosmetic {
                guid: Guid([
                    78, 134, 188, 248, 247, 144, 151, 76, 187, 104, 248, 236, 125, 158, 253, 225
                ]),
                name: String::from("Monumental Goatee - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [18, 131, 81, 83, 63, 4, 82, 74, 187, 126, 36, 229, 79, 69, 38, 123],
            Cosmetic {
                guid: Guid([18, 131, 81, 83, 63, 4, 82, 74, 187, 126, 36, 229, 79, 69, 38, 123]),
                name: String::from("Rambunctious Elder"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [153, 5, 240, 33, 100, 69, 32, 74, 149, 214, 165, 32, 39, 72, 247, 14],
            Cosmetic {
                guid: Guid([153, 5, 240, 33, 100, 69, 32, 74, 149, 214, 165, 32, 39, 72, 247, 14]),
                name: String::from("Twinned Wintry"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [11, 185, 1, 71, 78, 134, 176, 68, 189, 8, 122, 206, 249, 145, 103, 123],
            Cosmetic {
                guid: Guid([
                    11, 185, 1, 71, 78, 134, 176, 68, 189, 8, 122, 206, 249, 145, 103, 123
                ]),
                name: String::from("Twinned Wintry - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [65, 0, 1, 6, 52, 25, 173, 71, 183, 233, 47, 38, 111, 94, 221, 137],
            Cosmetic {
                guid: Guid([65, 0, 1, 6, 52, 25, 173, 71, 183, 233, 47, 38, 111, 94, 221, 137]),
                name: String::from("Twinned Wintry - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [168, 97, 24, 229, 166, 240, 133, 78, 136, 167, 117, 147, 219, 115, 130, 64],
            Cosmetic {
                guid: Guid([
                    168, 97, 24, 229, 166, 240, 133, 78, 136, 167, 117, 147, 219, 115, 130, 64
                ]),
                name: String::from("Mighty Pelt - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [97, 102, 58, 247, 110, 154, 242, 73, 152, 245, 175, 143, 179, 220, 237, 86],
            Cosmetic {
                guid: Guid([
                    97, 102, 58, 247, 110, 154, 242, 73, 152, 245, 175, 143, 179, 220, 237, 86
                ]),
                name: String::from("Bound Goatee"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [59, 122, 148, 10, 85, 90, 181, 65, 129, 140, 232, 210, 136, 40, 133, 67],
            Cosmetic {
                guid: Guid([
                    59, 122, 148, 10, 85, 90, 181, 65, 129, 140, 232, 210, 136, 40, 133, 67
                ]),
                name: String::from("Great Fork - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [117, 127, 8, 226, 215, 50, 55, 65, 168, 146, 47, 62, 12, 203, 118, 78],
            Cosmetic {
                guid: Guid([117, 127, 8, 226, 215, 50, 55, 65, 168, 146, 47, 62, 12, 203, 118, 78]),
                name: String::from("Great Fork"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [219, 50, 254, 10, 19, 208, 23, 66, 143, 138, 41, 124, 122, 58, 192, 210],
            Cosmetic {
                guid: Guid([
                    219, 50, 254, 10, 19, 208, 23, 66, 143, 138, 41, 124, 122, 58, 192, 210
                ]),
                name: String::from("Great Fork - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [192, 147, 124, 174, 164, 4, 67, 79, 144, 99, 214, 142, 110, 237, 146, 166],
            Cosmetic {
                guid: Guid([
                    192, 147, 124, 174, 164, 4, 67, 79, 144, 99, 214, 142, 110, 237, 146, 166
                ]),
                name: String::from("Double Horsetails"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [114, 55, 212, 228, 65, 3, 55, 65, 160, 29, 202, 209, 163, 50, 35, 86],
            Cosmetic {
                guid: Guid([114, 55, 212, 228, 65, 3, 55, 65, 160, 29, 202, 209, 163, 50, 35, 86]),
                name: String::from("Double Horsetails - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [242, 76, 230, 146, 42, 163, 14, 69, 186, 38, 86, 254, 221, 105, 219, 198],
            Cosmetic {
                guid: Guid([
                    242, 76, 230, 146, 42, 163, 14, 69, 186, 38, 86, 254, 221, 105, 219, 198
                ]),
                name: String::from("Double Horsetails - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [110, 222, 45, 156, 120, 24, 162, 70, 174, 244, 92, 63, 104, 180, 249, 184],
            Cosmetic {
                guid: Guid([
                    110, 222, 45, 156, 120, 24, 162, 70, 174, 244, 92, 63, 104, 180, 249, 184
                ]),
                name: String::from("Braided Abundance"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [162, 175, 150, 146, 54, 166, 220, 75, 183, 138, 126, 141, 79, 27, 6, 69],
            Cosmetic {
                guid: Guid([
                    162, 175, 150, 146, 54, 166, 220, 75, 183, 138, 126, 141, 79, 27, 6, 69
                ]),
                name: String::from("Braided Abundance - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [208, 36, 41, 236, 40, 147, 103, 75, 143, 219, 183, 83, 188, 45, 250, 188],
            Cosmetic {
                guid: Guid([
                    208, 36, 41, 236, 40, 147, 103, 75, 143, 219, 183, 83, 188, 45, 250, 188
                ]),
                name: String::from("Braided Abundance - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [42, 236, 148, 174, 129, 195, 98, 66, 170, 35, 206, 22, 229, 72, 245, 147],
            Cosmetic {
                guid: Guid([
                    42, 236, 148, 174, 129, 195, 98, 66, 170, 35, 206, 22, 229, 72, 245, 147
                ]),
                name: String::from("Savage Full"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [251, 142, 187, 12, 237, 197, 114, 77, 131, 218, 74, 13, 116, 51, 189, 31],
            Cosmetic {
                guid: Guid([
                    251, 142, 187, 12, 237, 197, 114, 77, 131, 218, 74, 13, 116, 51, 189, 31
                ]),
                name: String::from("Savage Full - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [131, 228, 7, 80, 68, 252, 240, 64, 145, 20, 231, 44, 121, 39, 80, 143],
            Cosmetic {
                guid: Guid([131, 228, 7, 80, 68, 252, 240, 64, 145, 20, 231, 44, 121, 39, 80, 143]),
                name: String::from("Savage Full - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [190, 163, 77, 203, 120, 94, 240, 69, 164, 234, 2, 212, 255, 219, 42, 235],
            Cosmetic {
                guid: Guid([
                    190, 163, 77, 203, 120, 94, 240, 69, 164, 234, 2, 212, 255, 219, 42, 235
                ]),
                name: String::from("Massive Braid"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [82, 130, 114, 240, 108, 175, 18, 74, 176, 185, 68, 145, 135, 70, 56, 219],
            Cosmetic {
                guid: Guid([
                    82, 130, 114, 240, 108, 175, 18, 74, 176, 185, 68, 145, 135, 70, 56, 219
                ]),
                name: String::from("Massive Braid - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [52, 61, 68, 207, 139, 138, 120, 78, 129, 115, 235, 174, 110, 206, 226, 132],
            Cosmetic {
                guid: Guid([
                    52, 61, 68, 207, 139, 138, 120, 78, 129, 115, 235, 174, 110, 206, 226, 132
                ]),
                name: String::from("Massive Braid - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [114, 246, 140, 128, 201, 20, 237, 64, 147, 5, 77, 77, 53, 130, 74, 103],
            Cosmetic {
                guid: Guid([
                    114, 246, 140, 128, 201, 20, 237, 64, 147, 5, 77, 77, 53, 130, 74, 103
                ]),
                name: String::from("Triple Trouble"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [33, 178, 204, 2, 154, 107, 120, 65, 166, 99, 80, 128, 64, 0, 177, 8],
            Cosmetic {
                guid: Guid([33, 178, 204, 2, 154, 107, 120, 65, 166, 99, 80, 128, 64, 0, 177, 8]),
                name: String::from("Triple Trouble - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [35, 105, 172, 119, 237, 84, 243, 75, 139, 143, 118, 89, 48, 136, 83, 3],
            Cosmetic {
                guid: Guid([
                    35, 105, 172, 119, 237, 84, 243, 75, 139, 143, 118, 89, 48, 136, 83, 3
                ]),
                name: String::from("Triple Trouble - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [40, 68, 220, 178, 1, 39, 68, 78, 148, 192, 104, 183, 8, 76, 176, 49],
            Cosmetic {
                guid: Guid([40, 68, 220, 178, 1, 39, 68, 78, 148, 192, 104, 183, 8, 76, 176, 49]),
                name: String::from("Crested Waterfall"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [59, 50, 148, 9, 166, 118, 123, 70, 143, 10, 11, 141, 91, 72, 65, 17],
            Cosmetic {
                guid: Guid([59, 50, 148, 9, 166, 118, 123, 70, 143, 10, 11, 141, 91, 72, 65, 17]),
                name: String::from("Tank Division"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [87, 191, 134, 239, 176, 148, 65, 79, 185, 77, 248, 116, 106, 238, 92, 144],
            Cosmetic {
                guid: Guid([
                    87, 191, 134, 239, 176, 148, 65, 79, 185, 77, 248, 116, 106, 238, 92, 144
                ]),
                name: String::from("Bound Unicord"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [217, 192, 25, 37, 165, 31, 144, 79, 157, 49, 63, 142, 183, 141, 127, 72],
            Cosmetic {
                guid: Guid([
                    217, 192, 25, 37, 165, 31, 144, 79, 157, 49, 63, 142, 183, 141, 127, 72
                ]),
                name: String::from("Bound Unicord - Armored"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [116, 125, 54, 31, 100, 172, 5, 71, 174, 223, 54, 163, 40, 118, 108, 21],
            Cosmetic {
                guid: Guid([
                    116, 125, 54, 31, 100, 172, 5, 71, 174, 223, 54, 163, 40, 118, 108, 21
                ]),
                name: String::from("Bound Unicord - Gilded"),
                dwarf: Driller,
                ty: Beard,
                status: None
            }
        ),
        (
            [167, 119, 93, 166, 144, 208, 200, 68, 171, 214, 83, 95, 102, 231, 135, 19],
            Cosmetic {
                guid: Guid([
                    167, 119, 93, 166, 144, 208, 200, 68, 171, 214, 83, 95, 102, 231, 135, 19
                ]),
                name: String::from("Bound Goatee - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [161, 177, 20, 189, 47, 231, 188, 64, 177, 232, 168, 235, 248, 49, 82, 140],
            Cosmetic {
                guid: Guid([
                    161, 177, 20, 189, 47, 231, 188, 64, 177, 232, 168, 235, 248, 49, 82, 140
                ]),
                name: String::from("Bound Goatee - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [7, 58, 173, 63, 210, 77, 254, 79, 139, 51, 233, 249, 153, 111, 29, 170],
            Cosmetic {
                guid: Guid([
                    7, 58, 173, 63, 210, 77, 254, 79, 139, 51, 233, 249, 153, 111, 29, 170
                ]),
                name: String::from("Mighty Pelt"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [49, 9, 175, 209, 43, 213, 64, 65, 178, 193, 95, 83, 116, 230, 195, 125],
            Cosmetic {
                guid: Guid([
                    49, 9, 175, 209, 43, 213, 64, 65, 178, 193, 95, 83, 116, 230, 195, 125
                ]),
                name: String::from("Mighty Pelt - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [235, 48, 204, 50, 32, 69, 124, 70, 149, 29, 61, 146, 208, 126, 134, 0],
            Cosmetic {
                guid: Guid([235, 48, 204, 50, 32, 69, 124, 70, 149, 29, 61, 146, 208, 126, 134, 0]),
                name: String::from("Monumental Goatee"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [208, 194, 55, 134, 173, 107, 75, 70, 154, 150, 33, 24, 224, 47, 27, 218],
            Cosmetic {
                guid: Guid([
                    208, 194, 55, 134, 173, 107, 75, 70, 154, 150, 33, 24, 224, 47, 27, 218
                ]),
                name: String::from("Monumental Goatee - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [60, 119, 95, 168, 116, 118, 203, 66, 172, 237, 54, 142, 142, 3, 51, 252],
            Cosmetic {
                guid: Guid([
                    60, 119, 95, 168, 116, 118, 203, 66, 172, 237, 54, 142, 142, 3, 51, 252
                ]),
                name: String::from("Monumental Goatee - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [54, 139, 23, 121, 103, 81, 52, 66, 147, 229, 215, 30, 38, 58, 86, 20],
            Cosmetic {
                guid: Guid([54, 139, 23, 121, 103, 81, 52, 66, 147, 229, 215, 30, 38, 58, 86, 20]),
                name: String::from("Rambunctious Elder"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [195, 154, 112, 203, 21, 38, 61, 76, 188, 168, 2, 221, 231, 168, 139, 129],
            Cosmetic {
                guid: Guid([
                    195, 154, 112, 203, 21, 38, 61, 76, 188, 168, 2, 221, 231, 168, 139, 129
                ]),
                name: String::from("Twinned Wintry"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [226, 103, 134, 147, 137, 59, 254, 67, 169, 44, 9, 105, 67, 231, 51, 247],
            Cosmetic {
                guid: Guid([
                    226, 103, 134, 147, 137, 59, 254, 67, 169, 44, 9, 105, 67, 231, 51, 247
                ]),
                name: String::from("Twinned Wintry - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [75, 56, 12, 9, 154, 53, 204, 78, 146, 171, 211, 190, 79, 158, 171, 83],
            Cosmetic {
                guid: Guid([75, 56, 12, 9, 154, 53, 204, 78, 146, 171, 211, 190, 79, 158, 171, 83]),
                name: String::from("Twinned Wintry - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [223, 202, 43, 242, 166, 140, 16, 64, 141, 74, 98, 55, 36, 78, 70, 212],
            Cosmetic {
                guid: Guid([223, 202, 43, 242, 166, 140, 16, 64, 141, 74, 98, 55, 36, 78, 70, 212]),
                name: String::from("Mighty Pelt - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [46, 220, 106, 8, 133, 55, 202, 68, 170, 115, 15, 169, 68, 125, 150, 190],
            Cosmetic {
                guid: Guid([
                    46, 220, 106, 8, 133, 55, 202, 68, 170, 115, 15, 169, 68, 125, 150, 190
                ]),
                name: String::from("Bound Goatee"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [79, 120, 52, 121, 90, 233, 237, 75, 150, 180, 8, 178, 29, 201, 213, 158],
            Cosmetic {
                guid: Guid([
                    79, 120, 52, 121, 90, 233, 237, 75, 150, 180, 8, 178, 29, 201, 213, 158
                ]),
                name: String::from("Great Fork - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [146, 87, 46, 177, 142, 249, 69, 78, 156, 127, 225, 6, 212, 62, 139, 214],
            Cosmetic {
                guid: Guid([
                    146, 87, 46, 177, 142, 249, 69, 78, 156, 127, 225, 6, 212, 62, 139, 214
                ]),
                name: String::from("Great Fork"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [85, 84, 143, 235, 12, 208, 5, 76, 168, 175, 19, 178, 13, 176, 50, 179],
            Cosmetic {
                guid: Guid([85, 84, 143, 235, 12, 208, 5, 76, 168, 175, 19, 178, 13, 176, 50, 179]),
                name: String::from("Great Fork - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [118, 206, 113, 184, 1, 208, 137, 76, 138, 24, 16, 211, 118, 188, 91, 61],
            Cosmetic {
                guid: Guid([
                    118, 206, 113, 184, 1, 208, 137, 76, 138, 24, 16, 211, 118, 188, 91, 61
                ]),
                name: String::from("Double Horsetails"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [15, 100, 1, 238, 135, 236, 93, 74, 155, 200, 239, 252, 158, 78, 110, 251],
            Cosmetic {
                guid: Guid([
                    15, 100, 1, 238, 135, 236, 93, 74, 155, 200, 239, 252, 158, 78, 110, 251
                ]),
                name: String::from("Double Horsetails - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [141, 223, 131, 190, 56, 129, 9, 71, 191, 33, 84, 124, 246, 16, 218, 9],
            Cosmetic {
                guid: Guid([141, 223, 131, 190, 56, 129, 9, 71, 191, 33, 84, 124, 246, 16, 218, 9]),
                name: String::from("Double Horsetails - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [9, 27, 254, 148, 101, 27, 51, 71, 137, 180, 228, 32, 103, 209, 228, 179],
            Cosmetic {
                guid: Guid([
                    9, 27, 254, 148, 101, 27, 51, 71, 137, 180, 228, 32, 103, 209, 228, 179
                ]),
                name: String::from("Braided Abundance"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [199, 112, 21, 206, 191, 126, 42, 78, 168, 193, 139, 8, 99, 95, 119, 249],
            Cosmetic {
                guid: Guid([
                    199, 112, 21, 206, 191, 126, 42, 78, 168, 193, 139, 8, 99, 95, 119, 249
                ]),
                name: String::from("Braided Abundance - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [126, 15, 166, 186, 199, 122, 79, 74, 191, 234, 199, 213, 18, 17, 175, 13],
            Cosmetic {
                guid: Guid([
                    126, 15, 166, 186, 199, 122, 79, 74, 191, 234, 199, 213, 18, 17, 175, 13
                ]),
                name: String::from("Braided Abundance - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [38, 200, 166, 14, 115, 229, 226, 77, 146, 92, 199, 193, 13, 248, 94, 35],
            Cosmetic {
                guid: Guid([
                    38, 200, 166, 14, 115, 229, 226, 77, 146, 92, 199, 193, 13, 248, 94, 35
                ]),
                name: String::from("Savage Full"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [109, 72, 12, 2, 78, 56, 165, 72, 181, 255, 46, 207, 169, 148, 28, 13],
            Cosmetic {
                guid: Guid([109, 72, 12, 2, 78, 56, 165, 72, 181, 255, 46, 207, 169, 148, 28, 13]),
                name: String::from("Savage Full - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [113, 213, 215, 186, 215, 169, 74, 78, 178, 223, 107, 24, 78, 193, 143, 180],
            Cosmetic {
                guid: Guid([
                    113, 213, 215, 186, 215, 169, 74, 78, 178, 223, 107, 24, 78, 193, 143, 180
                ]),
                name: String::from("Savage Full - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [216, 244, 37, 113, 218, 204, 147, 66, 131, 166, 0, 163, 201, 181, 76, 44],
            Cosmetic {
                guid: Guid([
                    216, 244, 37, 113, 218, 204, 147, 66, 131, 166, 0, 163, 201, 181, 76, 44
                ]),
                name: String::from("Massive Braid"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [122, 121, 41, 6, 183, 4, 205, 70, 175, 132, 188, 26, 171, 185, 43, 112],
            Cosmetic {
                guid: Guid([
                    122, 121, 41, 6, 183, 4, 205, 70, 175, 132, 188, 26, 171, 185, 43, 112
                ]),
                name: String::from("Massive Braid - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [130, 115, 250, 127, 46, 239, 91, 73, 167, 74, 203, 28, 211, 29, 121, 171],
            Cosmetic {
                guid: Guid([
                    130, 115, 250, 127, 46, 239, 91, 73, 167, 74, 203, 28, 211, 29, 121, 171
                ]),
                name: String::from("Massive Braid - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [167, 240, 11, 197, 229, 126, 113, 79, 191, 248, 2, 35, 213, 174, 133, 251],
            Cosmetic {
                guid: Guid([
                    167, 240, 11, 197, 229, 126, 113, 79, 191, 248, 2, 35, 213, 174, 133, 251
                ]),
                name: String::from("Triple Trouble"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [178, 201, 101, 251, 75, 43, 0, 69, 154, 209, 14, 254, 142, 189, 85, 226],
            Cosmetic {
                guid: Guid([
                    178, 201, 101, 251, 75, 43, 0, 69, 154, 209, 14, 254, 142, 189, 85, 226
                ]),
                name: String::from("Triple Trouble - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [81, 166, 214, 50, 177, 172, 234, 75, 183, 132, 37, 35, 63, 200, 42, 202],
            Cosmetic {
                guid: Guid([
                    81, 166, 214, 50, 177, 172, 234, 75, 183, 132, 37, 35, 63, 200, 42, 202
                ]),
                name: String::from("Triple Trouble - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [81, 74, 48, 172, 108, 84, 0, 65, 188, 88, 34, 56, 34, 69, 141, 12],
            Cosmetic {
                guid: Guid([81, 74, 48, 172, 108, 84, 0, 65, 188, 88, 34, 56, 34, 69, 141, 12]),
                name: String::from("Crested Waterfall"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [67, 156, 181, 198, 11, 97, 197, 73, 189, 136, 28, 141, 117, 255, 146, 48],
            Cosmetic {
                guid: Guid([
                    67, 156, 181, 198, 11, 97, 197, 73, 189, 136, 28, 141, 117, 255, 146, 48
                ]),
                name: String::from("Tank Division"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [148, 129, 126, 94, 81, 199, 41, 78, 182, 155, 23, 163, 12, 160, 104, 220],
            Cosmetic {
                guid: Guid([
                    148, 129, 126, 94, 81, 199, 41, 78, 182, 155, 23, 163, 12, 160, 104, 220
                ]),
                name: String::from("Bound Unicord"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [52, 26, 138, 177, 169, 4, 133, 70, 128, 21, 250, 161, 137, 67, 99, 61],
            Cosmetic {
                guid: Guid([52, 26, 138, 177, 169, 4, 133, 70, 128, 21, 250, 161, 137, 67, 99, 61]),
                name: String::from("Bound Unicord - Armored"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [12, 173, 76, 149, 165, 131, 89, 68, 172, 217, 239, 227, 231, 141, 30, 231],
            Cosmetic {
                guid: Guid([
                    12, 173, 76, 149, 165, 131, 89, 68, 172, 217, 239, 227, 231, 141, 30, 231
                ]),
                name: String::from("Bound Unicord - Gilded"),
                dwarf: Engineer,
                ty: Beard,
                status: None
            }
        ),
        (
            [11, 199, 107, 173, 160, 107, 194, 69, 160, 7, 17, 8, 68, 144, 146, 57],
            Cosmetic {
                guid: Guid([11, 199, 107, 173, 160, 107, 194, 69, 160, 7, 17, 8, 68, 144, 146, 57]),
                name: String::from("Bound Goatee - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [106, 187, 45, 151, 159, 137, 159, 70, 164, 0, 58, 120, 226, 141, 167, 114],
            Cosmetic {
                guid: Guid([
                    106, 187, 45, 151, 159, 137, 159, 70, 164, 0, 58, 120, 226, 141, 167, 114
                ]),
                name: String::from("Bound Goatee - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [167, 44, 10, 176, 120, 153, 2, 66, 148, 50, 104, 84, 168, 209, 255, 34],
            Cosmetic {
                guid: Guid([
                    167, 44, 10, 176, 120, 153, 2, 66, 148, 50, 104, 84, 168, 209, 255, 34
                ]),
                name: String::from("Mighty Pelt"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [131, 152, 180, 251, 78, 229, 16, 65, 157, 129, 215, 71, 223, 12, 173, 242],
            Cosmetic {
                guid: Guid([
                    131, 152, 180, 251, 78, 229, 16, 65, 157, 129, 215, 71, 223, 12, 173, 242
                ]),
                name: String::from("Mighty Pelt - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [199, 132, 252, 9, 139, 44, 35, 73, 183, 144, 55, 70, 114, 139, 178, 51],
            Cosmetic {
                guid: Guid([
                    199, 132, 252, 9, 139, 44, 35, 73, 183, 144, 55, 70, 114, 139, 178, 51
                ]),
                name: String::from("Monumental Goatee"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [226, 191, 185, 175, 203, 243, 152, 70, 187, 252, 124, 151, 27, 97, 70, 32],
            Cosmetic {
                guid: Guid([
                    226, 191, 185, 175, 203, 243, 152, 70, 187, 252, 124, 151, 27, 97, 70, 32
                ]),
                name: String::from("Monumental Goatee - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [181, 85, 189, 236, 170, 41, 255, 74, 171, 220, 101, 206, 56, 21, 207, 89],
            Cosmetic {
                guid: Guid([
                    181, 85, 189, 236, 170, 41, 255, 74, 171, 220, 101, 206, 56, 21, 207, 89
                ]),
                name: String::from("Monumental Goatee - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [176, 213, 105, 131, 30, 64, 168, 79, 137, 25, 203, 238, 177, 4, 228, 202],
            Cosmetic {
                guid: Guid([
                    176, 213, 105, 131, 30, 64, 168, 79, 137, 25, 203, 238, 177, 4, 228, 202
                ]),
                name: String::from("Rambunctious Elder"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [255, 177, 252, 155, 100, 203, 121, 72, 178, 132, 127, 153, 221, 230, 176, 237],
            Cosmetic {
                guid: Guid([
                    255, 177, 252, 155, 100, 203, 121, 72, 178, 132, 127, 153, 221, 230, 176, 237
                ]),
                name: String::from("Twinned Wintry"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [237, 42, 87, 134, 96, 184, 232, 73, 136, 5, 201, 219, 247, 190, 127, 164],
            Cosmetic {
                guid: Guid([
                    237, 42, 87, 134, 96, 184, 232, 73, 136, 5, 201, 219, 247, 190, 127, 164
                ]),
                name: String::from("Twinned Wintry - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [7, 144, 173, 241, 27, 229, 102, 73, 179, 41, 133, 74, 21, 6, 57, 253],
            Cosmetic {
                guid: Guid([7, 144, 173, 241, 27, 229, 102, 73, 179, 41, 133, 74, 21, 6, 57, 253]),
                name: String::from("Twinned Wintry - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [5, 252, 217, 112, 32, 179, 84, 67, 175, 117, 67, 201, 74, 190, 64, 188],
            Cosmetic {
                guid: Guid([
                    5, 252, 217, 112, 32, 179, 84, 67, 175, 117, 67, 201, 74, 190, 64, 188
                ]),
                name: String::from("Mighty Pelt - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [161, 149, 37, 14, 157, 99, 16, 69, 168, 231, 19, 116, 222, 13, 83, 188],
            Cosmetic {
                guid: Guid([
                    161, 149, 37, 14, 157, 99, 16, 69, 168, 231, 19, 116, 222, 13, 83, 188
                ]),
                name: String::from("Bound Goatee"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [246, 67, 57, 189, 137, 29, 122, 65, 138, 10, 63, 143, 103, 187, 99, 57],
            Cosmetic {
                guid: Guid([
                    246, 67, 57, 189, 137, 29, 122, 65, 138, 10, 63, 143, 103, 187, 99, 57
                ]),
                name: String::from("Great Fork - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [3, 27, 17, 145, 176, 239, 76, 64, 168, 135, 34, 239, 245, 2, 165, 188],
            Cosmetic {
                guid: Guid([3, 27, 17, 145, 176, 239, 76, 64, 168, 135, 34, 239, 245, 2, 165, 188]),
                name: String::from("Great Fork"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [186, 12, 177, 228, 85, 178, 18, 78, 176, 206, 207, 148, 223, 226, 97, 144],
            Cosmetic {
                guid: Guid([
                    186, 12, 177, 228, 85, 178, 18, 78, 176, 206, 207, 148, 223, 226, 97, 144
                ]),
                name: String::from("Great Fork - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [112, 91, 188, 121, 238, 249, 210, 76, 175, 224, 232, 48, 93, 141, 234, 18],
            Cosmetic {
                guid: Guid([
                    112, 91, 188, 121, 238, 249, 210, 76, 175, 224, 232, 48, 93, 141, 234, 18
                ]),
                name: String::from("Double Horsetails"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [54, 223, 26, 86, 232, 89, 189, 74, 159, 130, 90, 83, 125, 109, 54, 87],
            Cosmetic {
                guid: Guid([54, 223, 26, 86, 232, 89, 189, 74, 159, 130, 90, 83, 125, 109, 54, 87]),
                name: String::from("Double Horsetails - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [75, 13, 216, 221, 162, 216, 54, 70, 173, 61, 96, 248, 29, 9, 220, 0],
            Cosmetic {
                guid: Guid([75, 13, 216, 221, 162, 216, 54, 70, 173, 61, 96, 248, 29, 9, 220, 0]),
                name: String::from("Double Horsetails - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [24, 226, 34, 202, 142, 246, 127, 66, 150, 199, 233, 9, 237, 58, 110, 9],
            Cosmetic {
                guid: Guid([
                    24, 226, 34, 202, 142, 246, 127, 66, 150, 199, 233, 9, 237, 58, 110, 9
                ]),
                name: String::from("Braided Abundance"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [172, 2, 37, 154, 36, 41, 138, 66, 139, 71, 10, 239, 157, 100, 50, 126],
            Cosmetic {
                guid: Guid([172, 2, 37, 154, 36, 41, 138, 66, 139, 71, 10, 239, 157, 100, 50, 126]),
                name: String::from("Braided Abundance - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [180, 61, 89, 53, 169, 32, 242, 75, 133, 186, 238, 204, 113, 233, 197, 47],
            Cosmetic {
                guid: Guid([
                    180, 61, 89, 53, 169, 32, 242, 75, 133, 186, 238, 204, 113, 233, 197, 47
                ]),
                name: String::from("Braided Abundance - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [61, 70, 56, 88, 233, 120, 231, 65, 182, 29, 218, 73, 38, 67, 35, 51],
            Cosmetic {
                guid: Guid([61, 70, 56, 88, 233, 120, 231, 65, 182, 29, 218, 73, 38, 67, 35, 51]),
                name: String::from("Savage Full"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [161, 204, 108, 231, 71, 177, 177, 66, 137, 54, 186, 122, 0, 207, 39, 148],
            Cosmetic {
                guid: Guid([
                    161, 204, 108, 231, 71, 177, 177, 66, 137, 54, 186, 122, 0, 207, 39, 148
                ]),
                name: String::from("Savage Full - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [158, 42, 16, 40, 36, 54, 207, 76, 185, 240, 145, 204, 4, 170, 206, 214],
            Cosmetic {
                guid: Guid([
                    158, 42, 16, 40, 36, 54, 207, 76, 185, 240, 145, 204, 4, 170, 206, 214
                ]),
                name: String::from("Savage Full - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [55, 142, 82, 121, 207, 108, 28, 67, 151, 69, 249, 174, 220, 11, 91, 109],
            Cosmetic {
                guid: Guid([
                    55, 142, 82, 121, 207, 108, 28, 67, 151, 69, 249, 174, 220, 11, 91, 109
                ]),
                name: String::from("Massive Braid"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [197, 199, 185, 138, 97, 140, 193, 72, 163, 229, 45, 92, 121, 102, 47, 154],
            Cosmetic {
                guid: Guid([
                    197, 199, 185, 138, 97, 140, 193, 72, 163, 229, 45, 92, 121, 102, 47, 154
                ]),
                name: String::from("Massive Braid - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [142, 190, 213, 46, 128, 42, 7, 76, 151, 105, 68, 32, 172, 215, 178, 254],
            Cosmetic {
                guid: Guid([
                    142, 190, 213, 46, 128, 42, 7, 76, 151, 105, 68, 32, 172, 215, 178, 254
                ]),
                name: String::from("Massive Braid - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [191, 223, 254, 231, 153, 97, 201, 65, 184, 160, 98, 93, 128, 82, 96, 162],
            Cosmetic {
                guid: Guid([
                    191, 223, 254, 231, 153, 97, 201, 65, 184, 160, 98, 93, 128, 82, 96, 162
                ]),
                name: String::from("Triple Trouble"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [9, 58, 216, 155, 185, 59, 219, 73, 155, 225, 81, 21, 34, 199, 39, 197],
            Cosmetic {
                guid: Guid([9, 58, 216, 155, 185, 59, 219, 73, 155, 225, 81, 21, 34, 199, 39, 197]),
                name: String::from("Triple Trouble - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [202, 249, 72, 51, 67, 20, 101, 71, 158, 39, 61, 125, 63, 235, 120, 131],
            Cosmetic {
                guid: Guid([
                    202, 249, 72, 51, 67, 20, 101, 71, 158, 39, 61, 125, 63, 235, 120, 131
                ]),
                name: String::from("Triple Trouble - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [127, 15, 129, 8, 128, 39, 57, 64, 171, 237, 230, 174, 48, 255, 62, 69],
            Cosmetic {
                guid: Guid([127, 15, 129, 8, 128, 39, 57, 64, 171, 237, 230, 174, 48, 255, 62, 69]),
                name: String::from("Crested Waterfall"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [0, 28, 255, 21, 218, 105, 179, 69, 168, 48, 129, 164, 95, 152, 244, 30],
            Cosmetic {
                guid: Guid([
                    0, 28, 255, 21, 218, 105, 179, 69, 168, 48, 129, 164, 95, 152, 244, 30
                ]),
                name: String::from("Tank Division"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [58, 6, 221, 111, 47, 118, 85, 69, 147, 201, 65, 145, 161, 166, 19, 250],
            Cosmetic {
                guid: Guid([
                    58, 6, 221, 111, 47, 118, 85, 69, 147, 201, 65, 145, 161, 166, 19, 250
                ]),
                name: String::from("Bound Unicord"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [72, 205, 134, 197, 224, 55, 145, 71, 186, 184, 9, 49, 4, 214, 248, 8],
            Cosmetic {
                guid: Guid([72, 205, 134, 197, 224, 55, 145, 71, 186, 184, 9, 49, 4, 214, 248, 8]),
                name: String::from("Bound Unicord - Armored"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [180, 59, 204, 121, 9, 127, 102, 73, 179, 101, 85, 234, 106, 179, 47, 81],
            Cosmetic {
                guid: Guid([
                    180, 59, 204, 121, 9, 127, 102, 73, 179, 101, 85, 234, 106, 179, 47, 81
                ]),
                name: String::from("Bound Unicord - Gilded"),
                dwarf: Gunner,
                ty: Beard,
                status: None
            }
        ),
        (
            [95, 226, 9, 238, 231, 5, 129, 72, 176, 63, 239, 229, 118, 62, 254, 85],
            Cosmetic {
                guid: Guid([95, 226, 9, 238, 231, 5, 129, 72, 176, 63, 239, 229, 118, 62, 254, 85]),
                name: String::from("Bound Goatee - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [243, 164, 253, 13, 134, 226, 107, 71, 133, 195, 146, 205, 147, 209, 72, 164],
            Cosmetic {
                guid: Guid([
                    243, 164, 253, 13, 134, 226, 107, 71, 133, 195, 146, 205, 147, 209, 72, 164
                ]),
                name: String::from("Bound Goatee - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [4, 98, 198, 192, 153, 169, 252, 72, 187, 190, 20, 151, 36, 61, 195, 143],
            Cosmetic {
                guid: Guid([
                    4, 98, 198, 192, 153, 169, 252, 72, 187, 190, 20, 151, 36, 61, 195, 143
                ]),
                name: String::from("Mighty Pelt"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [133, 130, 226, 253, 245, 157, 9, 77, 134, 61, 26, 28, 70, 62, 132, 101],
            Cosmetic {
                guid: Guid([
                    133, 130, 226, 253, 245, 157, 9, 77, 134, 61, 26, 28, 70, 62, 132, 101
                ]),
                name: String::from("Mighty Pelt - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [180, 98, 234, 19, 146, 128, 28, 67, 162, 142, 104, 204, 138, 104, 103, 221],
            Cosmetic {
                guid: Guid([
                    180, 98, 234, 19, 146, 128, 28, 67, 162, 142, 104, 204, 138, 104, 103, 221
                ]),
                name: String::from("Monumental Goatee"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [240, 153, 174, 156, 209, 4, 215, 77, 161, 121, 1, 107, 107, 255, 129, 23],
            Cosmetic {
                guid: Guid([
                    240, 153, 174, 156, 209, 4, 215, 77, 161, 121, 1, 107, 107, 255, 129, 23
                ]),
                name: String::from("Monumental Goatee - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [223, 232, 79, 217, 240, 156, 133, 76, 133, 18, 55, 0, 196, 25, 11, 10],
            Cosmetic {
                guid: Guid([223, 232, 79, 217, 240, 156, 133, 76, 133, 18, 55, 0, 196, 25, 11, 10]),
                name: String::from("Monumental Goatee - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [69, 143, 81, 216, 1, 111, 104, 78, 129, 235, 45, 187, 117, 244, 77, 182],
            Cosmetic {
                guid: Guid([
                    69, 143, 81, 216, 1, 111, 104, 78, 129, 235, 45, 187, 117, 244, 77, 182
                ]),
                name: String::from("Rambunctious Elder"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [104, 163, 58, 186, 95, 6, 105, 71, 164, 88, 59, 250, 102, 233, 170, 247],
            Cosmetic {
                guid: Guid([
                    104, 163, 58, 186, 95, 6, 105, 71, 164, 88, 59, 250, 102, 233, 170, 247
                ]),
                name: String::from("Twinned Wintry"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [183, 181, 212, 94, 134, 142, 200, 69, 177, 199, 215, 0, 6, 88, 157, 198],
            Cosmetic {
                guid: Guid([
                    183, 181, 212, 94, 134, 142, 200, 69, 177, 199, 215, 0, 6, 88, 157, 198
                ]),
                name: String::from("Twinned Wintry - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [109, 58, 53, 238, 206, 135, 81, 78, 150, 175, 173, 163, 174, 150, 50, 79],
            Cosmetic {
                guid: Guid([
                    109, 58, 53, 238, 206, 135, 81, 78, 150, 175, 173, 163, 174, 150, 50, 79
                ]),
                name: String::from("Twinned Wintry - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [26, 22, 198, 196, 239, 27, 47, 76, 167, 254, 21, 83, 10, 150, 194, 147],
            Cosmetic {
                guid: Guid([
                    26, 22, 198, 196, 239, 27, 47, 76, 167, 254, 21, 83, 10, 150, 194, 147
                ]),
                name: String::from("Mighty Pelt - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [154, 243, 95, 14, 11, 62, 95, 79, 190, 221, 72, 37, 58, 148, 103, 125],
            Cosmetic {
                guid: Guid([154, 243, 95, 14, 11, 62, 95, 79, 190, 221, 72, 37, 58, 148, 103, 125]),
                name: String::from("Bound Goatee"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [250, 42, 129, 31, 123, 160, 33, 67, 168, 220, 114, 11, 193, 9, 101, 197],
            Cosmetic {
                guid: Guid([
                    250, 42, 129, 31, 123, 160, 33, 67, 168, 220, 114, 11, 193, 9, 101, 197
                ]),
                name: String::from("Great Fork - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [114, 156, 130, 243, 72, 119, 151, 67, 163, 177, 14, 200, 24, 145, 141, 142],
            Cosmetic {
                guid: Guid([
                    114, 156, 130, 243, 72, 119, 151, 67, 163, 177, 14, 200, 24, 145, 141, 142
                ]),
                name: String::from("Great Fork"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [12, 167, 186, 159, 104, 156, 37, 66, 136, 199, 71, 184, 5, 7, 204, 149],
            Cosmetic {
                guid: Guid([
                    12, 167, 186, 159, 104, 156, 37, 66, 136, 199, 71, 184, 5, 7, 204, 149
                ]),
                name: String::from("Great Fork - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [15, 184, 201, 176, 39, 212, 127, 66, 148, 75, 79, 250, 32, 2, 108, 168],
            Cosmetic {
                guid: Guid([
                    15, 184, 201, 176, 39, 212, 127, 66, 148, 75, 79, 250, 32, 2, 108, 168
                ]),
                name: String::from("Double Horsetails"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [176, 31, 25, 64, 216, 84, 221, 69, 173, 216, 120, 80, 126, 156, 11, 235],
            Cosmetic {
                guid: Guid([
                    176, 31, 25, 64, 216, 84, 221, 69, 173, 216, 120, 80, 126, 156, 11, 235
                ]),
                name: String::from("Double Horsetails - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [85, 82, 31, 184, 44, 97, 112, 78, 177, 64, 35, 113, 55, 232, 165, 163],
            Cosmetic {
                guid: Guid([85, 82, 31, 184, 44, 97, 112, 78, 177, 64, 35, 113, 55, 232, 165, 163]),
                name: String::from("Double Horsetails - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [189, 145, 74, 214, 244, 98, 9, 77, 166, 35, 199, 25, 9, 54, 107, 149],
            Cosmetic {
                guid: Guid([189, 145, 74, 214, 244, 98, 9, 77, 166, 35, 199, 25, 9, 54, 107, 149]),
                name: String::from("Braided Abundance"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [135, 148, 154, 90, 250, 164, 251, 67, 188, 216, 235, 231, 80, 78, 219, 136],
            Cosmetic {
                guid: Guid([
                    135, 148, 154, 90, 250, 164, 251, 67, 188, 216, 235, 231, 80, 78, 219, 136
                ]),
                name: String::from("Braided Abundance - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [237, 135, 212, 14, 189, 128, 192, 78, 159, 12, 131, 81, 41, 111, 73, 162],
            Cosmetic {
                guid: Guid([
                    237, 135, 212, 14, 189, 128, 192, 78, 159, 12, 131, 81, 41, 111, 73, 162
                ]),
                name: String::from("Braided Abundance - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [111, 225, 41, 44, 44, 20, 27, 79, 166, 200, 126, 211, 2, 122, 234, 233],
            Cosmetic {
                guid: Guid([
                    111, 225, 41, 44, 44, 20, 27, 79, 166, 200, 126, 211, 2, 122, 234, 233
                ]),
                name: String::from("Savage Full"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [115, 188, 79, 224, 45, 101, 227, 66, 166, 249, 233, 196, 50, 17, 28, 13],
            Cosmetic {
                guid: Guid([
                    115, 188, 79, 224, 45, 101, 227, 66, 166, 249, 233, 196, 50, 17, 28, 13
                ]),
                name: String::from("Savage Full - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [13, 6, 204, 155, 73, 104, 50, 70, 141, 57, 84, 6, 234, 82, 172, 152],
            Cosmetic {
                guid: Guid([13, 6, 204, 155, 73, 104, 50, 70, 141, 57, 84, 6, 234, 82, 172, 152]),
                name: String::from("Savage Full - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [157, 105, 186, 252, 128, 60, 121, 70, 140, 27, 212, 241, 157, 198, 143, 143],
            Cosmetic {
                guid: Guid([
                    157, 105, 186, 252, 128, 60, 121, 70, 140, 27, 212, 241, 157, 198, 143, 143
                ]),
                name: String::from("Massive Braid"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [7, 41, 55, 8, 218, 163, 137, 73, 129, 86, 244, 70, 67, 167, 236, 180],
            Cosmetic {
                guid: Guid([7, 41, 55, 8, 218, 163, 137, 73, 129, 86, 244, 70, 67, 167, 236, 180]),
                name: String::from("Massive Braid - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [48, 87, 26, 164, 89, 164, 214, 79, 178, 226, 80, 219, 230, 26, 188, 167],
            Cosmetic {
                guid: Guid([
                    48, 87, 26, 164, 89, 164, 214, 79, 178, 226, 80, 219, 230, 26, 188, 167
                ]),
                name: String::from("Massive Braid - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [140, 8, 60, 50, 88, 37, 93, 73, 177, 201, 37, 205, 75, 243, 240, 231],
            Cosmetic {
                guid: Guid([140, 8, 60, 50, 88, 37, 93, 73, 177, 201, 37, 205, 75, 243, 240, 231]),
                name: String::from("Triple Trouble"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [55, 82, 250, 175, 88, 94, 252, 78, 167, 73, 74, 231, 115, 215, 156, 171],
            Cosmetic {
                guid: Guid([
                    55, 82, 250, 175, 88, 94, 252, 78, 167, 73, 74, 231, 115, 215, 156, 171
                ]),
                name: String::from("Triple Trouble - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [148, 1, 246, 124, 99, 107, 219, 74, 140, 102, 151, 227, 57, 14, 172, 99],
            Cosmetic {
                guid: Guid([
                    148, 1, 246, 124, 99, 107, 219, 74, 140, 102, 151, 227, 57, 14, 172, 99
                ]),
                name: String::from("Triple Trouble - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [151, 204, 87, 67, 157, 145, 62, 68, 150, 73, 198, 191, 174, 136, 217, 40],
            Cosmetic {
                guid: Guid([
                    151, 204, 87, 67, 157, 145, 62, 68, 150, 73, 198, 191, 174, 136, 217, 40
                ]),
                name: String::from("Crested Waterfall"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [115, 191, 35, 208, 67, 55, 62, 69, 128, 165, 83, 31, 60, 53, 155, 162],
            Cosmetic {
                guid: Guid([115, 191, 35, 208, 67, 55, 62, 69, 128, 165, 83, 31, 60, 53, 155, 162]),
                name: String::from("Tank Division"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [73, 204, 87, 32, 87, 1, 43, 76, 128, 248, 62, 190, 86, 96, 127, 128],
            Cosmetic {
                guid: Guid([73, 204, 87, 32, 87, 1, 43, 76, 128, 248, 62, 190, 86, 96, 127, 128]),
                name: String::from("Bound Unicord"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [162, 211, 34, 141, 221, 89, 103, 64, 131, 22, 72, 217, 117, 126, 56, 195],
            Cosmetic {
                guid: Guid([
                    162, 211, 34, 141, 221, 89, 103, 64, 131, 22, 72, 217, 117, 126, 56, 195
                ]),
                name: String::from("Bound Unicord - Armored"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [113, 33, 82, 239, 223, 57, 201, 74, 164, 149, 174, 187, 251, 122, 93, 38],
            Cosmetic {
                guid: Guid([
                    113, 33, 82, 239, 223, 57, 201, 74, 164, 149, 174, 187, 251, 122, 93, 38
                ]),
                name: String::from("Bound Unicord - Gilded"),
                dwarf: Scout,
                ty: Beard,
                status: None
            }
        ),
        (
            [61, 95, 92, 15, 145, 189, 23, 74, 176, 106, 222, 111, 33, 248, 243, 202],
            Cosmetic {
                guid: Guid([
                    61, 95, 92, 15, 145, 189, 23, 74, 176, 106, 222, 111, 33, 248, 243, 202
                ]),
                name: String::from("Iron Cyclops"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [242, 1, 30, 195, 198, 15, 150, 71, 183, 3, 222, 207, 228, 169, 157, 178],
            Cosmetic {
                guid: Guid([
                    242, 1, 30, 195, 198, 15, 150, 71, 183, 3, 222, 207, 228, 169, 157, 178
                ]),
                name: String::from("Techy Hard Hat"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [78, 184, 48, 46, 247, 169, 17, 65, 151, 154, 50, 154, 169, 142, 124, 130],
            Cosmetic {
                guid: Guid([
                    78, 184, 48, 46, 247, 169, 17, 65, 151, 154, 50, 154, 169, 142, 124, 130
                ]),
                name: String::from("Grim Specter - Full Face"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [97, 115, 217, 86, 20, 44, 137, 77, 171, 0, 11, 128, 194, 154, 128, 202],
            Cosmetic {
                guid: Guid([
                    97, 115, 217, 86, 20, 44, 137, 77, 171, 0, 11, 128, 194, 154, 128, 202
                ]),
                name: String::from("Grim Specter - Half Face"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [183, 159, 98, 170, 219, 144, 73, 79, 157, 224, 139, 185, 128, 72, 114, 214],
            Cosmetic {
                guid: Guid([
                    183, 159, 98, 170, 219, 144, 73, 79, 157, 224, 139, 185, 128, 72, 114, 214
                ]),
                name: String::from("Heatwave"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [186, 101, 85, 252, 210, 109, 192, 77, 152, 62, 83, 216, 170, 75, 75, 39],
            Cosmetic {
                guid: Guid([
                    186, 101, 85, 252, 210, 109, 192, 77, 152, 62, 83, 216, 170, 75, 75, 39
                ]),
                name: String::from("Spiked Mohawk"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [218, 109, 86, 186, 124, 255, 128, 76, 181, 108, 77, 144, 85, 112, 217, 211],
            Cosmetic {
                guid: Guid([
                    218, 109, 86, 186, 124, 255, 128, 76, 181, 108, 77, 144, 85, 112, 217, 211
                ]),
                name: String::from("First Blood"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [206, 82, 165, 163, 96, 246, 251, 71, 137, 183, 29, 65, 46, 1, 223, 37],
            Cosmetic {
                guid: Guid([206, 82, 165, 163, 96, 246, 251, 71, 137, 183, 29, 65, 46, 1, 223, 37]),
                name: String::from("Full Headwrap"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [156, 226, 22, 173, 150, 163, 199, 78, 145, 60, 178, 105, 251, 18, 185, 43],
            Cosmetic {
                guid: Guid([
                    156, 226, 22, 173, 150, 163, 199, 78, 145, 60, 178, 105, 251, 18, 185, 43
                ]),
                name: String::from("Goggled Headwrap"),
                dwarf: Driller,
                ty: Headwear,
                status: None
            }
        ),
        (
            [215, 23, 170, 87, 85, 176, 95, 72, 177, 75, 191, 242, 219, 154, 74, 64],
            Cosmetic {
                guid: Guid([
                    215, 23, 170, 87, 85, 176, 95, 72, 177, 75, 191, 242, 219, 154, 74, 64
                ]),
                name: String::from("Iron Cyclops"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [77, 63, 31, 219, 180, 166, 94, 66, 171, 145, 185, 162, 119, 203, 185, 86],
            Cosmetic {
                guid: Guid([
                    77, 63, 31, 219, 180, 166, 94, 66, 171, 145, 185, 162, 119, 203, 185, 86
                ]),
                name: String::from("Techy Hard Hat"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [202, 215, 69, 63, 240, 222, 38, 79, 140, 111, 233, 0, 32, 94, 113, 101],
            Cosmetic {
                guid: Guid([
                    202, 215, 69, 63, 240, 222, 38, 79, 140, 111, 233, 0, 32, 94, 113, 101
                ]),
                name: String::from("Grim Specter - Full Face"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [42, 63, 75, 169, 93, 65, 202, 77, 161, 70, 8, 139, 167, 152, 183, 179],
            Cosmetic {
                guid: Guid([42, 63, 75, 169, 93, 65, 202, 77, 161, 70, 8, 139, 167, 152, 183, 179]),
                name: String::from("Grim Specter - Half Face"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [86, 204, 99, 114, 18, 212, 66, 73, 187, 170, 152, 4, 8, 106, 83, 117],
            Cosmetic {
                guid: Guid([86, 204, 99, 114, 18, 212, 66, 73, 187, 170, 152, 4, 8, 106, 83, 117]),
                name: String::from("Heatwave"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [194, 228, 59, 58, 242, 245, 234, 67, 143, 59, 113, 17, 254, 223, 250, 72],
            Cosmetic {
                guid: Guid([
                    194, 228, 59, 58, 242, 245, 234, 67, 143, 59, 113, 17, 254, 223, 250, 72
                ]),
                name: String::from("Spiked Mohawk"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [204, 93, 7, 2, 183, 222, 57, 68, 150, 193, 82, 125, 109, 181, 211, 171],
            Cosmetic {
                guid: Guid([
                    204, 93, 7, 2, 183, 222, 57, 68, 150, 193, 82, 125, 109, 181, 211, 171
                ]),
                name: String::from("First Blood"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [72, 116, 115, 99, 255, 242, 163, 71, 153, 29, 205, 114, 32, 66, 128, 44],
            Cosmetic {
                guid: Guid([
                    72, 116, 115, 99, 255, 242, 163, 71, 153, 29, 205, 114, 32, 66, 128, 44
                ]),
                name: String::from("Full Headwrap"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [63, 76, 150, 51, 158, 128, 102, 73, 172, 12, 240, 146, 209, 182, 0, 199],
            Cosmetic {
                guid: Guid([
                    63, 76, 150, 51, 158, 128, 102, 73, 172, 12, 240, 146, 209, 182, 0, 199
                ]),
                name: String::from("Goggled Headwrap"),
                dwarf: Engineer,
                ty: Headwear,
                status: None
            }
        ),
        (
            [11, 142, 109, 231, 72, 19, 165, 65, 170, 89, 128, 212, 102, 135, 173, 24],
            Cosmetic {
                guid: Guid([
                    11, 142, 109, 231, 72, 19, 165, 65, 170, 89, 128, 212, 102, 135, 173, 24
                ]),
                name: String::from("Iron Cyclops"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [69, 159, 167, 203, 181, 7, 209, 67, 183, 4, 98, 242, 252, 59, 6, 237],
            Cosmetic {
                guid: Guid([69, 159, 167, 203, 181, 7, 209, 67, 183, 4, 98, 242, 252, 59, 6, 237]),
                name: String::from("Techy Hard Hat"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [113, 21, 254, 108, 31, 253, 123, 70, 156, 206, 227, 243, 154, 104, 0, 77],
            Cosmetic {
                guid: Guid([
                    113, 21, 254, 108, 31, 253, 123, 70, 156, 206, 227, 243, 154, 104, 0, 77
                ]),
                name: String::from("Grim Specter - Full Face"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [124, 62, 88, 20, 74, 185, 74, 73, 168, 129, 19, 47, 152, 107, 202, 15],
            Cosmetic {
                guid: Guid([124, 62, 88, 20, 74, 185, 74, 73, 168, 129, 19, 47, 152, 107, 202, 15]),
                name: String::from("Grim Specter - Half Face"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [86, 172, 22, 4, 177, 150, 20, 79, 166, 80, 151, 10, 143, 118, 111, 223],
            Cosmetic {
                guid: Guid([
                    86, 172, 22, 4, 177, 150, 20, 79, 166, 80, 151, 10, 143, 118, 111, 223
                ]),
                name: String::from("Heatwave"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [245, 248, 239, 218, 120, 208, 174, 79, 172, 25, 200, 109, 66, 104, 82, 203],
            Cosmetic {
                guid: Guid([
                    245, 248, 239, 218, 120, 208, 174, 79, 172, 25, 200, 109, 66, 104, 82, 203
                ]),
                name: String::from("Spiked Mohawk"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [35, 46, 248, 74, 12, 215, 208, 79, 143, 207, 87, 131, 251, 238, 221, 188],
            Cosmetic {
                guid: Guid([
                    35, 46, 248, 74, 12, 215, 208, 79, 143, 207, 87, 131, 251, 238, 221, 188
                ]),
                name: String::from("First Blood"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [6, 219, 86, 127, 188, 91, 120, 77, 175, 95, 235, 172, 180, 202, 71, 47],
            Cosmetic {
                guid: Guid([
                    6, 219, 86, 127, 188, 91, 120, 77, 175, 95, 235, 172, 180, 202, 71, 47
                ]),
                name: String::from("Full Headwrap"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [43, 44, 79, 54, 53, 50, 227, 68, 147, 184, 238, 59, 148, 221, 226, 151],
            Cosmetic {
                guid: Guid([
                    43, 44, 79, 54, 53, 50, 227, 68, 147, 184, 238, 59, 148, 221, 226, 151
                ]),
                name: String::from("Goggled Headwrap"),
                dwarf: Gunner,
                ty: Headwear,
                status: None
            }
        ),
        (
            [226, 89, 184, 127, 110, 69, 131, 69, 191, 226, 80, 145, 115, 143, 43, 228],
            Cosmetic {
                guid: Guid([
                    226, 89, 184, 127, 110, 69, 131, 69, 191, 226, 80, 145, 115, 143, 43, 228
                ]),
                name: String::from("Iron Cyclops"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [166, 197, 5, 62, 221, 86, 75, 78, 153, 144, 246, 168, 225, 164, 40, 22],
            Cosmetic {
                guid: Guid([
                    166, 197, 5, 62, 221, 86, 75, 78, 153, 144, 246, 168, 225, 164, 40, 22
                ]),
                name: String::from("Techy Hard Hat"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [187, 63, 220, 29, 74, 147, 141, 78, 176, 38, 97, 68, 74, 97, 110, 225],
            Cosmetic {
                guid: Guid([187, 63, 220, 29, 74, 147, 141, 78, 176, 38, 97, 68, 74, 97, 110, 225]),
                name: String::from("Grim Specter - Full Face"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [30, 237, 237, 206, 206, 0, 85, 77, 143, 28, 229, 134, 61, 114, 128, 33],
            Cosmetic {
                guid: Guid([
                    30, 237, 237, 206, 206, 0, 85, 77, 143, 28, 229, 134, 61, 114, 128, 33
                ]),
                name: String::from("Grim Specter - Half Face"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [29, 81, 92, 22, 61, 138, 121, 71, 185, 54, 222, 252, 42, 216, 151, 209],
            Cosmetic {
                guid: Guid([
                    29, 81, 92, 22, 61, 138, 121, 71, 185, 54, 222, 252, 42, 216, 151, 209
                ]),
                name: String::from("Heatwave"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [70, 54, 53, 255, 119, 26, 193, 67, 183, 178, 3, 117, 191, 228, 111, 139],
            Cosmetic {
                guid: Guid([
                    70, 54, 53, 255, 119, 26, 193, 67, 183, 178, 3, 117, 191, 228, 111, 139
                ]),
                name: String::from("Spiked Mohawk"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [153, 183, 42, 25, 143, 91, 29, 73, 156, 11, 164, 247, 166, 86, 130, 200],
            Cosmetic {
                guid: Guid([
                    153, 183, 42, 25, 143, 91, 29, 73, 156, 11, 164, 247, 166, 86, 130, 200
                ]),
                name: String::from("First Blood"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [46, 191, 74, 120, 19, 187, 78, 65, 149, 3, 89, 156, 142, 114, 48, 25],
            Cosmetic {
                guid: Guid([46, 191, 74, 120, 19, 187, 78, 65, 149, 3, 89, 156, 142, 114, 48, 25]),
                name: String::from("Full Headwrap"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [147, 250, 173, 203, 231, 121, 53, 65, 152, 84, 124, 76, 95, 193, 24, 34],
            Cosmetic {
                guid: Guid([
                    147, 250, 173, 203, 231, 121, 53, 65, 152, 84, 124, 76, 95, 193, 24, 34
                ]),
                name: String::from("Goggled Headwrap"),
                dwarf: Scout,
                ty: Headwear,
                status: None
            }
        ),
        (
            [40, 121, 248, 139, 218, 94, 110, 65, 161, 159, 83, 216, 157, 106, 152, 237],
            Cosmetic {
                guid: Guid([
                    40, 121, 248, 139, 218, 94, 110, 65, 161, 159, 83, 216, 157, 106, 152, 237
                ]),
                name: String::from("Curly Optimist"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [173, 164, 88, 133, 120, 48, 3, 67, 167, 170, 238, 246, 154, 91, 136, 124],
            Cosmetic {
                guid: Guid([
                    173, 164, 88, 133, 120, 48, 3, 67, 167, 170, 238, 246, 154, 91, 136, 124
                ]),
                name: String::from("Exquisite Handlebar"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [169, 224, 165, 247, 161, 119, 111, 75, 173, 59, 209, 176, 15, 194, 116, 230],
            Cosmetic {
                guid: Guid([
                    169, 224, 165, 247, 161, 119, 111, 75, 173, 59, 209, 176, 15, 194, 116, 230
                ]),
                name: String::from("Lowrider"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [84, 236, 254, 115, 20, 114, 31, 79, 185, 130, 148, 139, 88, 38, 52, 113],
            Cosmetic {
                guid: Guid([
                    84, 236, 254, 115, 20, 114, 31, 79, 185, 130, 148, 139, 88, 38, 52, 113
                ]),
                name: String::from("Bushy Goodguy"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [35, 193, 207, 175, 7, 53, 185, 72, 178, 190, 214, 75, 159, 179, 190, 209],
            Cosmetic {
                guid: Guid([
                    35, 193, 207, 175, 7, 53, 185, 72, 178, 190, 214, 75, 159, 179, 190, 209
                ]),
                name: String::from("Marshall"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [204, 101, 78, 235, 180, 9, 203, 65, 156, 101, 149, 130, 15, 41, 223, 205],
            Cosmetic {
                guid: Guid([
                    204, 101, 78, 235, 180, 9, 203, 65, 156, 101, 149, 130, 15, 41, 223, 205
                ]),
                name: String::from("Dandy Handlebar"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [39, 115, 96, 137, 229, 111, 158, 75, 166, 158, 159, 92, 91, 34, 180, 130],
            Cosmetic {
                guid: Guid([
                    39, 115, 96, 137, 229, 111, 158, 75, 166, 158, 159, 92, 91, 34, 180, 130
                ]),
                name: String::from("Magnificent Raider"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [244, 10, 105, 36, 72, 246, 118, 66, 144, 75, 116, 83, 212, 136, 32, 172],
            Cosmetic {
                guid: Guid([
                    244, 10, 105, 36, 72, 246, 118, 66, 144, 75, 116, 83, 212, 136, 32, 172
                ]),
                name: String::from("Spiky Neurotic"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [210, 108, 184, 171, 137, 21, 145, 74, 160, 105, 167, 36, 174, 116, 164, 27],
            Cosmetic {
                guid: Guid([
                    210, 108, 184, 171, 137, 21, 145, 74, 160, 105, 167, 36, 174, 116, 164, 27
                ]),
                name: String::from("Bound Braids"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [32, 110, 130, 203, 12, 57, 22, 72, 137, 112, 250, 199, 26, 3, 252, 100],
            Cosmetic {
                guid: Guid([
                    32, 110, 130, 203, 12, 57, 22, 72, 137, 112, 250, 199, 26, 3, 252, 100
                ]),
                name: String::from("Bangled Braids"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [89, 37, 236, 1, 151, 26, 222, 71, 163, 183, 249, 114, 102, 245, 46, 45],
            Cosmetic {
                guid: Guid([
                    89, 37, 236, 1, 151, 26, 222, 71, 163, 183, 249, 114, 102, 245, 46, 45
                ]),
                name: String::from("Crescent Moon"),
                dwarf: Driller,
                ty: Moustache,
                status: None
            }
        ),
        (
            [10, 197, 3, 215, 19, 236, 80, 78, 136, 92, 137, 12, 121, 193, 223, 33],
            Cosmetic {
                guid: Guid([10, 197, 3, 215, 19, 236, 80, 78, 136, 92, 137, 12, 121, 193, 223, 33]),
                name: String::from("Curly Optimist"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [26, 142, 94, 48, 216, 230, 221, 64, 136, 134, 184, 4, 12, 118, 169, 103],
            Cosmetic {
                guid: Guid([
                    26, 142, 94, 48, 216, 230, 221, 64, 136, 134, 184, 4, 12, 118, 169, 103
                ]),
                name: String::from("Exquisite Handlebar"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [239, 47, 112, 12, 69, 35, 14, 79, 182, 179, 173, 174, 226, 3, 125, 45],
            Cosmetic {
                guid: Guid([239, 47, 112, 12, 69, 35, 14, 79, 182, 179, 173, 174, 226, 3, 125, 45]),
                name: String::from("Lowrider"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [187, 160, 124, 109, 123, 237, 221, 68, 135, 59, 105, 42, 110, 38, 25, 202],
            Cosmetic {
                guid: Guid([
                    187, 160, 124, 109, 123, 237, 221, 68, 135, 59, 105, 42, 110, 38, 25, 202
                ]),
                name: String::from("Bushy Goodguy"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [224, 9, 94, 239, 252, 177, 190, 72, 167, 127, 83, 51, 120, 96, 117, 230],
            Cosmetic {
                guid: Guid([
                    224, 9, 94, 239, 252, 177, 190, 72, 167, 127, 83, 51, 120, 96, 117, 230
                ]),
                name: String::from("Marshall"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [117, 110, 46, 152, 80, 12, 42, 78, 148, 20, 90, 209, 199, 74, 131, 16],
            Cosmetic {
                guid: Guid([117, 110, 46, 152, 80, 12, 42, 78, 148, 20, 90, 209, 199, 74, 131, 16]),
                name: String::from("Dandy Handlebar"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [51, 228, 12, 198, 191, 231, 230, 64, 169, 154, 7, 140, 35, 199, 214, 59],
            Cosmetic {
                guid: Guid([
                    51, 228, 12, 198, 191, 231, 230, 64, 169, 154, 7, 140, 35, 199, 214, 59
                ]),
                name: String::from("Magnificent Raider"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [239, 187, 75, 175, 159, 56, 84, 68, 151, 108, 167, 103, 147, 180, 101, 229],
            Cosmetic {
                guid: Guid([
                    239, 187, 75, 175, 159, 56, 84, 68, 151, 108, 167, 103, 147, 180, 101, 229
                ]),
                name: String::from("Spiky Neurotic"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [230, 200, 254, 33, 38, 12, 114, 70, 148, 160, 1, 127, 22, 93, 240, 251],
            Cosmetic {
                guid: Guid([
                    230, 200, 254, 33, 38, 12, 114, 70, 148, 160, 1, 127, 22, 93, 240, 251
                ]),
                name: String::from("Bound Braids"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [47, 159, 83, 88, 164, 42, 157, 74, 164, 62, 36, 173, 139, 80, 148, 102],
            Cosmetic {
                guid: Guid([
                    47, 159, 83, 88, 164, 42, 157, 74, 164, 62, 36, 173, 139, 80, 148, 102
                ]),
                name: String::from("Bangled Braids"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [69, 38, 57, 198, 99, 180, 25, 66, 180, 101, 147, 177, 227, 101, 234, 161],
            Cosmetic {
                guid: Guid([
                    69, 38, 57, 198, 99, 180, 25, 66, 180, 101, 147, 177, 227, 101, 234, 161
                ]),
                name: String::from("Crescent Moon"),
                dwarf: Engineer,
                ty: Moustache,
                status: None
            }
        ),
        (
            [146, 26, 250, 131, 133, 138, 205, 76, 133, 28, 244, 228, 134, 121, 3, 155],
            Cosmetic {
                guid: Guid([
                    146, 26, 250, 131, 133, 138, 205, 76, 133, 28, 244, 228, 134, 121, 3, 155
                ]),
                name: String::from("Curly Optimist"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [201, 181, 88, 42, 6, 201, 226, 70, 189, 74, 34, 124, 66, 62, 211, 203],
            Cosmetic {
                guid: Guid([201, 181, 88, 42, 6, 201, 226, 70, 189, 74, 34, 124, 66, 62, 211, 203]),
                name: String::from("Exquisite Handlebar"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [122, 132, 155, 167, 58, 232, 107, 64, 175, 76, 235, 184, 146, 234, 40, 170],
            Cosmetic {
                guid: Guid([
                    122, 132, 155, 167, 58, 232, 107, 64, 175, 76, 235, 184, 146, 234, 40, 170
                ]),
                name: String::from("Lowrider"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [28, 133, 223, 231, 242, 0, 132, 76, 174, 86, 3, 22, 202, 232, 52, 98],
            Cosmetic {
                guid: Guid([28, 133, 223, 231, 242, 0, 132, 76, 174, 86, 3, 22, 202, 232, 52, 98]),
                name: String::from("Bushy Goodguy"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [40, 60, 248, 73, 236, 12, 91, 70, 160, 194, 131, 172, 106, 188, 252, 236],
            Cosmetic {
                guid: Guid([
                    40, 60, 248, 73, 236, 12, 91, 70, 160, 194, 131, 172, 106, 188, 252, 236
                ]),
                name: String::from("Marshall"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [211, 145, 87, 247, 231, 182, 62, 66, 166, 170, 135, 141, 114, 66, 137, 88],
            Cosmetic {
                guid: Guid([
                    211, 145, 87, 247, 231, 182, 62, 66, 166, 170, 135, 141, 114, 66, 137, 88
                ]),
                name: String::from("Dandy Handlebar"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [38, 228, 39, 99, 109, 98, 36, 73, 179, 215, 61, 151, 77, 81, 88, 43],
            Cosmetic {
                guid: Guid([38, 228, 39, 99, 109, 98, 36, 73, 179, 215, 61, 151, 77, 81, 88, 43]),
                name: String::from("Magnificent Raider"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [40, 34, 5, 116, 140, 97, 8, 75, 134, 226, 78, 83, 224, 171, 5, 20],
            Cosmetic {
                guid: Guid([40, 34, 5, 116, 140, 97, 8, 75, 134, 226, 78, 83, 224, 171, 5, 20]),
                name: String::from("Spiky Neurotic"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [121, 44, 213, 88, 218, 229, 116, 78, 164, 248, 246, 255, 202, 251, 95, 161],
            Cosmetic {
                guid: Guid([
                    121, 44, 213, 88, 218, 229, 116, 78, 164, 248, 246, 255, 202, 251, 95, 161
                ]),
                name: String::from("Bound Braids"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [91, 72, 15, 61, 170, 180, 206, 71, 168, 142, 43, 163, 213, 37, 62, 106],
            Cosmetic {
                guid: Guid([
                    91, 72, 15, 61, 170, 180, 206, 71, 168, 142, 43, 163, 213, 37, 62, 106
                ]),
                name: String::from("Bangled Braids"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [0, 232, 82, 218, 4, 236, 31, 65, 167, 69, 60, 68, 228, 55, 52, 172],
            Cosmetic {
                guid: Guid([0, 232, 82, 218, 4, 236, 31, 65, 167, 69, 60, 68, 228, 55, 52, 172]),
                name: String::from("Crescent Moon"),
                dwarf: Gunner,
                ty: Moustache,
                status: None
            }
        ),
        (
            [73, 213, 3, 254, 183, 125, 72, 64, 132, 35, 120, 213, 237, 159, 158, 2],
            Cosmetic {
                guid: Guid([
                    73, 213, 3, 254, 183, 125, 72, 64, 132, 35, 120, 213, 237, 159, 158, 2
                ]),
                name: String::from("Curly Optimist"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [213, 190, 158, 83, 19, 34, 83, 76, 128, 189, 150, 91, 156, 110, 197, 103],
            Cosmetic {
                guid: Guid([
                    213, 190, 158, 83, 19, 34, 83, 76, 128, 189, 150, 91, 156, 110, 197, 103
                ]),
                name: String::from("Exquisite Handlebar"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [200, 74, 164, 12, 131, 211, 39, 65, 146, 142, 36, 229, 104, 172, 232, 151],
            Cosmetic {
                guid: Guid([
                    200, 74, 164, 12, 131, 211, 39, 65, 146, 142, 36, 229, 104, 172, 232, 151
                ]),
                name: String::from("Lowrider"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [212, 120, 169, 167, 16, 104, 155, 74, 139, 18, 29, 198, 201, 53, 61, 230],
            Cosmetic {
                guid: Guid([
                    212, 120, 169, 167, 16, 104, 155, 74, 139, 18, 29, 198, 201, 53, 61, 230
                ]),
                name: String::from("Bushy Goodguy"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [40, 48, 53, 205, 139, 29, 77, 68, 145, 149, 230, 248, 222, 42, 170, 166],
            Cosmetic {
                guid: Guid([
                    40, 48, 53, 205, 139, 29, 77, 68, 145, 149, 230, 248, 222, 42, 170, 166
                ]),
                name: String::from("Marshall"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [121, 214, 160, 59, 156, 139, 92, 70, 182, 43, 113, 184, 169, 206, 142, 101],
            Cosmetic {
                guid: Guid([
                    121, 214, 160, 59, 156, 139, 92, 70, 182, 43, 113, 184, 169, 206, 142, 101
                ]),
                name: String::from("Dandy Handlebar"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [15, 151, 122, 221, 226, 7, 25, 77, 172, 13, 243, 225, 136, 44, 93, 152],
            Cosmetic {
                guid: Guid([
                    15, 151, 122, 221, 226, 7, 25, 77, 172, 13, 243, 225, 136, 44, 93, 152
                ]),
                name: String::from("Magnificent Raider"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [165, 148, 178, 220, 173, 158, 3, 70, 167, 235, 6, 253, 249, 221, 24, 18],
            Cosmetic {
                guid: Guid([
                    165, 148, 178, 220, 173, 158, 3, 70, 167, 235, 6, 253, 249, 221, 24, 18
                ]),
                name: String::from("Spiky Neurotic"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [178, 215, 54, 60, 158, 114, 136, 74, 155, 200, 125, 212, 18, 183, 249, 140],
            Cosmetic {
                guid: Guid([
                    178, 215, 54, 60, 158, 114, 136, 74, 155, 200, 125, 212, 18, 183, 249, 140
                ]),
                name: String::from("Bound Braids"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [232, 56, 95, 212, 200, 217, 79, 69, 134, 191, 199, 219, 160, 208, 49, 39],
            Cosmetic {
                guid: Guid([
                    232, 56, 95, 212, 200, 217, 79, 69, 134, 191, 199, 219, 160, 208, 49, 39
                ]),
                name: String::from("Bangled Braids"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [39, 214, 61, 121, 157, 13, 113, 78, 139, 167, 173, 240, 226, 171, 244, 210],
            Cosmetic {
                guid: Guid([
                    39, 214, 61, 121, 157, 13, 113, 78, 139, 167, 173, 240, 226, 171, 244, 210
                ]),
                name: String::from("Crescent Moon"),
                dwarf: Scout,
                ty: Moustache,
                status: None
            }
        ),
        (
            [101, 44, 255, 126, 3, 109, 165, 76, 174, 67, 206, 121, 148, 135, 70, 108],
            Cosmetic {
                guid: Guid([
                    101, 44, 255, 126, 3, 109, 165, 76, 174, 67, 206, 121, 148, 135, 70, 108
                ]),
                name: String::from("Immense Danglies"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [177, 16, 242, 93, 240, 17, 1, 69, 134, 106, 236, 20, 241, 74, 130, 140],
            Cosmetic {
                guid: Guid([
                    177, 16, 242, 93, 240, 17, 1, 69, 134, 106, 236, 20, 241, 74, 130, 140
                ]),
                name: String::from("Braided Bangles"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [162, 7, 121, 31, 177, 66, 86, 75, 135, 149, 9, 24, 83, 125, 233, 59],
            Cosmetic {
                guid: Guid([162, 7, 121, 31, 177, 66, 86, 75, 135, 149, 9, 24, 83, 125, 233, 59]),
                name: String::from("Pointed Noble"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [31, 199, 106, 27, 19, 6, 166, 76, 177, 44, 193, 157, 101, 182, 123, 74],
            Cosmetic {
                guid: Guid([
                    31, 199, 106, 27, 19, 6, 166, 76, 177, 44, 193, 157, 101, 182, 123, 74
                ]),
                name: String::from("The Thickest"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [85, 133, 124, 219, 35, 16, 149, 78, 132, 212, 183, 67, 184, 132, 190, 120],
            Cosmetic {
                guid: Guid([
                    85, 133, 124, 219, 35, 16, 149, 78, 132, 212, 183, 67, 184, 132, 190, 120
                ]),
                name: String::from("Upbeat Youngling"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [213, 173, 127, 198, 43, 215, 37, 70, 160, 191, 85, 217, 131, 108, 41, 250],
            Cosmetic {
                guid: Guid([
                    213, 173, 127, 198, 43, 215, 37, 70, 160, 191, 85, 217, 131, 108, 41, 250
                ]),
                name: String::from("Berserker"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [159, 196, 19, 82, 30, 17, 156, 79, 163, 239, 48, 93, 65, 44, 148, 67],
            Cosmetic {
                guid: Guid([159, 196, 19, 82, 30, 17, 156, 79, 163, 239, 48, 93, 65, 44, 148, 67]),
                name: String::from("Valiant Berserker"),
                dwarf: Driller,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [114, 127, 253, 193, 91, 112, 181, 65, 149, 38, 46, 64, 218, 61, 203, 95],
            Cosmetic {
                guid: Guid([
                    114, 127, 253, 193, 91, 112, 181, 65, 149, 38, 46, 64, 218, 61, 203, 95
                ]),
                name: String::from("Immense Danglies"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [238, 233, 3, 229, 250, 201, 245, 69, 187, 84, 34, 231, 178, 39, 75, 108],
            Cosmetic {
                guid: Guid([
                    238, 233, 3, 229, 250, 201, 245, 69, 187, 84, 34, 231, 178, 39, 75, 108
                ]),
                name: String::from("Braided Bangles"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [114, 87, 131, 12, 86, 94, 240, 66, 160, 252, 80, 19, 216, 242, 86, 91],
            Cosmetic {
                guid: Guid([114, 87, 131, 12, 86, 94, 240, 66, 160, 252, 80, 19, 216, 242, 86, 91]),
                name: String::from("Pointed Noble"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [76, 89, 82, 49, 70, 93, 172, 70, 141, 3, 160, 134, 58, 47, 191, 7],
            Cosmetic {
                guid: Guid([76, 89, 82, 49, 70, 93, 172, 70, 141, 3, 160, 134, 58, 47, 191, 7]),
                name: String::from("The Thickest"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [129, 119, 35, 53, 253, 37, 201, 68, 188, 33, 43, 40, 208, 186, 94, 94],
            Cosmetic {
                guid: Guid([129, 119, 35, 53, 253, 37, 201, 68, 188, 33, 43, 40, 208, 186, 94, 94]),
                name: String::from("Upbeat Youngling"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [125, 59, 249, 20, 253, 177, 2, 73, 144, 84, 9, 167, 74, 121, 235, 155],
            Cosmetic {
                guid: Guid([125, 59, 249, 20, 253, 177, 2, 73, 144, 84, 9, 167, 74, 121, 235, 155]),
                name: String::from("Berserker"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [162, 122, 36, 21, 114, 7, 86, 67, 133, 252, 156, 192, 46, 56, 167, 30],
            Cosmetic {
                guid: Guid([162, 122, 36, 21, 114, 7, 86, 67, 133, 252, 156, 192, 46, 56, 167, 30]),
                name: String::from("Valiant Berserker"),
                dwarf: Engineer,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [48, 241, 67, 232, 222, 134, 209, 67, 130, 236, 0, 33, 197, 67, 208, 179],
            Cosmetic {
                guid: Guid([
                    48, 241, 67, 232, 222, 134, 209, 67, 130, 236, 0, 33, 197, 67, 208, 179
                ]),
                name: String::from("Immense Danglies"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [55, 235, 124, 206, 137, 200, 108, 76, 152, 89, 210, 17, 64, 219, 108, 91],
            Cosmetic {
                guid: Guid([
                    55, 235, 124, 206, 137, 200, 108, 76, 152, 89, 210, 17, 64, 219, 108, 91
                ]),
                name: String::from("Braided Bangles"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [37, 40, 55, 178, 142, 149, 172, 76, 185, 154, 177, 99, 163, 168, 251, 181],
            Cosmetic {
                guid: Guid([
                    37, 40, 55, 178, 142, 149, 172, 76, 185, 154, 177, 99, 163, 168, 251, 181
                ]),
                name: String::from("Pointed Noble"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [32, 207, 122, 147, 25, 250, 140, 70, 135, 11, 186, 124, 189, 76, 38, 212],
            Cosmetic {
                guid: Guid([
                    32, 207, 122, 147, 25, 250, 140, 70, 135, 11, 186, 124, 189, 76, 38, 212
                ]),
                name: String::from("The Thickest"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [103, 123, 146, 162, 114, 144, 152, 70, 166, 155, 252, 160, 126, 141, 213, 127],
            Cosmetic {
                guid: Guid([
                    103, 123, 146, 162, 114, 144, 152, 70, 166, 155, 252, 160, 126, 141, 213, 127
                ]),
                name: String::from("Upbeat Youngling"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [67, 106, 244, 78, 46, 121, 202, 68, 146, 151, 45, 194, 206, 183, 177, 217],
            Cosmetic {
                guid: Guid([
                    67, 106, 244, 78, 46, 121, 202, 68, 146, 151, 45, 194, 206, 183, 177, 217
                ]),
                name: String::from("Berserker"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [249, 93, 1, 203, 152, 208, 111, 70, 138, 194, 246, 26, 18, 188, 206, 32],
            Cosmetic {
                guid: Guid([
                    249, 93, 1, 203, 152, 208, 111, 70, 138, 194, 246, 26, 18, 188, 206, 32
                ]),
                name: String::from("Valiant Berserker"),
                dwarf: Gunner,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [93, 110, 44, 66, 95, 22, 176, 66, 144, 235, 135, 156, 190, 96, 206, 37],
            Cosmetic {
                guid: Guid([
                    93, 110, 44, 66, 95, 22, 176, 66, 144, 235, 135, 156, 190, 96, 206, 37
                ]),
                name: String::from("Immense Danglies"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [243, 202, 200, 2, 104, 59, 244, 79, 129, 210, 102, 34, 75, 226, 27, 47],
            Cosmetic {
                guid: Guid([
                    243, 202, 200, 2, 104, 59, 244, 79, 129, 210, 102, 34, 75, 226, 27, 47
                ]),
                name: String::from("Braided Bangles"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [67, 38, 234, 14, 110, 42, 65, 71, 176, 65, 185, 149, 178, 44, 233, 1],
            Cosmetic {
                guid: Guid([67, 38, 234, 14, 110, 42, 65, 71, 176, 65, 185, 149, 178, 44, 233, 1]),
                name: String::from("Pointed Noble"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [43, 39, 191, 4, 126, 201, 1, 66, 180, 251, 45, 67, 207, 151, 214, 254],
            Cosmetic {
                guid: Guid([43, 39, 191, 4, 126, 201, 1, 66, 180, 251, 45, 67, 207, 151, 214, 254]),
                name: String::from("The Thickest"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [93, 209, 105, 248, 252, 49, 178, 69, 180, 210, 159, 132, 158, 28, 73, 233],
            Cosmetic {
                guid: Guid([
                    93, 209, 105, 248, 252, 49, 178, 69, 180, 210, 159, 132, 158, 28, 73, 233
                ]),
                name: String::from("Upbeat Youngling"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [170, 11, 29, 80, 9, 177, 245, 69, 186, 209, 242, 202, 145, 179, 133, 19],
            Cosmetic {
                guid: Guid([
                    170, 11, 29, 80, 9, 177, 245, 69, 186, 209, 242, 202, 145, 179, 133, 19
                ]),
                name: String::from("Berserker"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [16, 232, 164, 126, 252, 174, 153, 76, 163, 192, 254, 218, 110, 42, 215, 112],
            Cosmetic {
                guid: Guid([
                    16, 232, 164, 126, 252, 174, 153, 76, 163, 192, 254, 218, 110, 42, 215, 112
                ]),
                name: String::from("Valiant Berserker"),
                dwarf: Scout,
                ty: Sideburns,
                status: None
            }
        ),
        (
            [238, 203, 2, 71, 52, 105, 51, 76, 167, 199, 64, 176, 36, 178, 9, 81],
            Cosmetic {
                guid: Guid([238, 203, 2, 71, 52, 105, 51, 76, 167, 199, 64, 176, 36, 178, 9, 81]),
                name: String::from("Swarmer Stomp"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [45, 101, 251, 134, 239, 182, 92, 75, 149, 184, 12, 88, 173, 125, 184, 191],
            Cosmetic {
                guid: Guid([
                    45, 101, 251, 134, 239, 182, 92, 75, 149, 184, 12, 88, 173, 125, 184, 191
                ]),
                name: String::from("Swarmer Stomp"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [77, 125, 208, 196, 115, 112, 139, 67, 172, 11, 2, 182, 238, 184, 55, 50],
            Cosmetic {
                guid: Guid([
                    77, 125, 208, 196, 115, 112, 139, 67, 172, 11, 2, 182, 238, 184, 55, 50
                ]),
                name: String::from("Swarmer Stomp"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [35, 131, 110, 82, 79, 251, 72, 71, 172, 13, 116, 28, 117, 95, 82, 246],
            Cosmetic {
                guid: Guid([35, 131, 110, 82, 79, 251, 72, 71, 172, 13, 116, 28, 117, 95, 82, 246]),
                name: String::from("Swarmer Stomp"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [165, 72, 45, 104, 162, 248, 214, 74, 183, 55, 37, 130, 232, 218, 160, 50],
            Cosmetic {
                guid: Guid([
                    165, 72, 45, 104, 162, 248, 214, 74, 183, 55, 37, 130, 232, 218, 160, 50
                ]),
                name: String::from("Barrel Drop"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [151, 94, 167, 131, 193, 14, 67, 66, 191, 246, 127, 19, 112, 223, 41, 164],
            Cosmetic {
                guid: Guid([
                    151, 94, 167, 131, 193, 14, 67, 66, 191, 246, 127, 19, 112, 223, 41, 164
                ]),
                name: String::from("Barrel Drop"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [159, 247, 68, 222, 157, 226, 215, 67, 182, 30, 4, 23, 86, 154, 110, 95],
            Cosmetic {
                guid: Guid([
                    159, 247, 68, 222, 157, 226, 215, 67, 182, 30, 4, 23, 86, 154, 110, 95
                ]),
                name: String::from("Barrel Drop"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [175, 134, 36, 134, 33, 99, 71, 69, 159, 5, 130, 249, 223, 110, 95, 141],
            Cosmetic {
                guid: Guid([
                    175, 134, 36, 134, 33, 99, 71, 69, 159, 5, 130, 249, 223, 110, 95, 141
                ]),
                name: String::from("Barrel Drop"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [206, 174, 82, 191, 199, 57, 109, 79, 191, 242, 60, 245, 163, 117, 185, 250],
            Cosmetic {
                guid: Guid([
                    206, 174, 82, 191, 199, 57, 109, 79, 191, 242, 60, 245, 163, 117, 185, 250
                ]),
                name: String::from("Theatrical Bow"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [121, 21, 243, 145, 217, 18, 73, 77, 148, 188, 222, 142, 95, 235, 49, 227],
            Cosmetic {
                guid: Guid([
                    121, 21, 243, 145, 217, 18, 73, 77, 148, 188, 222, 142, 95, 235, 49, 227
                ]),
                name: String::from("Theatrical Bow"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [13, 147, 15, 169, 4, 105, 187, 79, 144, 97, 96, 1, 61, 248, 149, 0],
            Cosmetic {
                guid: Guid([13, 147, 15, 169, 4, 105, 187, 79, 144, 97, 96, 1, 61, 248, 149, 0]),
                name: String::from("Theatrical Bow"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [178, 240, 84, 248, 196, 64, 155, 77, 153, 81, 112, 142, 37, 225, 216, 72],
            Cosmetic {
                guid: Guid([
                    178, 240, 84, 248, 196, 64, 155, 77, 153, 81, 112, 142, 37, 225, 216, 72
                ]),
                name: String::from("Theatrical Bow"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [87, 157, 148, 223, 19, 67, 69, 68, 137, 58, 173, 126, 228, 35, 108, 148],
            Cosmetic {
                guid: Guid([
                    87, 157, 148, 223, 19, 67, 69, 68, 137, 58, 173, 126, 228, 35, 108, 148
                ]),
                name: String::from("Fingerguns"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [37, 1, 219, 111, 182, 61, 145, 70, 162, 58, 187, 75, 118, 189, 240, 89],
            Cosmetic {
                guid: Guid([
                    37, 1, 219, 111, 182, 61, 145, 70, 162, 58, 187, 75, 118, 189, 240, 89
                ]),
                name: String::from("Fingerguns"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [101, 43, 44, 202, 249, 138, 101, 68, 130, 49, 42, 91, 50, 245, 216, 240],
            Cosmetic {
                guid: Guid([
                    101, 43, 44, 202, 249, 138, 101, 68, 130, 49, 42, 91, 50, 245, 216, 240
                ]),
                name: String::from("Fingerguns"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [197, 175, 94, 155, 52, 236, 166, 65, 129, 71, 147, 155, 33, 112, 179, 244],
            Cosmetic {
                guid: Guid([
                    197, 175, 94, 155, 52, 236, 166, 65, 129, 71, 147, 155, 33, 112, 179, 244
                ]),
                name: String::from("Fingerguns"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [220, 104, 13, 167, 16, 31, 168, 70, 161, 78, 74, 86, 80, 95, 251, 124],
            Cosmetic {
                guid: Guid([220, 104, 13, 167, 16, 31, 168, 70, 161, 78, 74, 86, 80, 95, 251, 124]),
                name: String::from("Dual Drink Drop"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [22, 189, 111, 6, 206, 219, 10, 73, 174, 82, 107, 112, 139, 158, 51, 4],
            Cosmetic {
                guid: Guid([22, 189, 111, 6, 206, 219, 10, 73, 174, 82, 107, 112, 139, 158, 51, 4]),
                name: String::from("Dual Drink Drop"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [76, 76, 7, 36, 96, 3, 57, 71, 135, 65, 247, 147, 125, 94, 164, 219],
            Cosmetic {
                guid: Guid([76, 76, 7, 36, 96, 3, 57, 71, 135, 65, 247, 147, 125, 94, 164, 219]),
                name: String::from("Dual Drink Drop"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [138, 239, 130, 40, 103, 224, 78, 66, 131, 60, 68, 149, 162, 27, 157, 66],
            Cosmetic {
                guid: Guid([
                    138, 239, 130, 40, 103, 224, 78, 66, 131, 60, 68, 149, 162, 27, 157, 66
                ]),
                name: String::from("Dual Drink Drop"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [110, 60, 205, 16, 114, 21, 31, 69, 172, 63, 252, 88, 91, 140, 8, 47],
            Cosmetic {
                guid: Guid([110, 60, 205, 16, 114, 21, 31, 69, 172, 63, 252, 88, 91, 140, 8, 47]),
                name: String::from("Smooches"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [136, 12, 93, 136, 111, 37, 234, 78, 154, 183, 163, 156, 37, 44, 152, 246],
            Cosmetic {
                guid: Guid([
                    136, 12, 93, 136, 111, 37, 234, 78, 154, 183, 163, 156, 37, 44, 152, 246
                ]),
                name: String::from("Smooches"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [150, 255, 57, 45, 129, 219, 102, 69, 171, 121, 126, 171, 32, 123, 4, 20],
            Cosmetic {
                guid: Guid([
                    150, 255, 57, 45, 129, 219, 102, 69, 171, 121, 126, 171, 32, 123, 4, 20
                ]),
                name: String::from("Smooches"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [44, 110, 150, 52, 246, 236, 73, 77, 174, 228, 231, 170, 7, 218, 49, 215],
            Cosmetic {
                guid: Guid([
                    44, 110, 150, 52, 246, 236, 73, 77, 174, 228, 231, 170, 7, 218, 49, 215
                ]),
                name: String::from("Smooches"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [106, 214, 238, 115, 115, 89, 133, 67, 144, 243, 131, 165, 6, 62, 129, 143],
            Cosmetic {
                guid: Guid([
                    106, 214, 238, 115, 115, 89, 133, 67, 144, 243, 131, 165, 6, 62, 129, 143
                ]),
                name: String::from("Flex Them Guns"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [69, 194, 19, 68, 74, 244, 197, 75, 165, 173, 186, 120, 186, 63, 136, 99],
            Cosmetic {
                guid: Guid([
                    69, 194, 19, 68, 74, 244, 197, 75, 165, 173, 186, 120, 186, 63, 136, 99
                ]),
                name: String::from("Flex Them Guns"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [96, 61, 137, 117, 165, 147, 8, 67, 147, 63, 85, 2, 68, 182, 79, 3],
            Cosmetic {
                guid: Guid([96, 61, 137, 117, 165, 147, 8, 67, 147, 63, 85, 2, 68, 182, 79, 3]),
                name: String::from("Flex Them Guns"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [37, 42, 59, 17, 58, 185, 95, 65, 171, 195, 216, 6, 204, 191, 115, 83],
            Cosmetic {
                guid: Guid([37, 42, 59, 17, 58, 185, 95, 65, 171, 195, 216, 6, 204, 191, 115, 83]),
                name: String::from("Flex Them Guns"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [208, 111, 121, 61, 132, 3, 4, 78, 191, 143, 82, 137, 165, 34, 1, 106],
            Cosmetic {
                guid: Guid([208, 111, 121, 61, 132, 3, 4, 78, 191, 143, 82, 137, 165, 34, 1, 106]),
                name: String::from("Furious Clown"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [101, 121, 239, 17, 192, 6, 252, 67, 166, 26, 60, 93, 189, 143, 179, 32],
            Cosmetic {
                guid: Guid([
                    101, 121, 239, 17, 192, 6, 252, 67, 166, 26, 60, 93, 189, 143, 179, 32
                ]),
                name: String::from("Furious Clown"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [95, 156, 36, 153, 128, 183, 241, 65, 136, 64, 88, 118, 182, 19, 233, 90],
            Cosmetic {
                guid: Guid([
                    95, 156, 36, 153, 128, 183, 241, 65, 136, 64, 88, 118, 182, 19, 233, 90
                ]),
                name: String::from("Furious Clown"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [26, 195, 29, 121, 238, 118, 205, 79, 176, 195, 15, 94, 75, 107, 27, 7],
            Cosmetic {
                guid: Guid([26, 195, 29, 121, 238, 118, 205, 79, 176, 195, 15, 94, 75, 107, 27, 7]),
                name: String::from("Furious Clown"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [31, 42, 14, 20, 128, 8, 96, 71, 190, 22, 172, 7, 91, 96, 8, 144],
            Cosmetic {
                guid: Guid([31, 42, 14, 20, 128, 8, 96, 71, 190, 22, 172, 7, 91, 96, 8, 144]),
                name: String::from("Crystal Lover"),
                dwarf: Driller,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [222, 56, 198, 159, 204, 74, 96, 79, 166, 197, 177, 43, 153, 39, 87, 214],
            Cosmetic {
                guid: Guid([
                    222, 56, 198, 159, 204, 74, 96, 79, 166, 197, 177, 43, 153, 39, 87, 214
                ]),
                name: String::from("Crystal Lover"),
                dwarf: Engineer,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [176, 100, 187, 12, 120, 11, 235, 73, 154, 138, 60, 116, 38, 133, 189, 193],
            Cosmetic {
                guid: Guid([
                    176, 100, 187, 12, 120, 11, 235, 73, 154, 138, 60, 116, 38, 133, 189, 193
                ]),
                name: String::from("Crystal Lover"),
                dwarf: Gunner,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [232, 138, 66, 161, 218, 105, 157, 68, 153, 171, 102, 196, 66, 55, 165, 173],
            Cosmetic {
                guid: Guid([
                    232, 138, 66, 161, 218, 105, 157, 68, 153, 171, 102, 196, 66, 55, 165, 173
                ]),
                name: String::from("Crystal Lover"),
                dwarf: Scout,
                ty: VictoryMove,
                status: None
            }
        ),
        (
            [228, 22, 129, 166, 162, 234, 132, 68, 178, 231, 91, 223, 137, 112, 218, 218],
            Cosmetic {
                guid: Guid([
                    228, 22, 129, 166, 162, 234, 132, 68, 178, 231, 91, 223, 137, 112, 218, 218
                ]),
                name: String::from("Warmonger"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [209, 91, 44, 12, 47, 134, 92, 74, 143, 94, 179, 84, 203, 17, 199, 143],
            Cosmetic {
                guid: Guid([209, 91, 44, 12, 47, 134, 92, 74, 143, 94, 179, 84, 203, 17, 199, 143]),
                name: String::from("Warmonger"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [232, 128, 197, 246, 231, 147, 164, 79, 186, 201, 127, 173, 24, 138, 207, 95],
            Cosmetic {
                guid: Guid([
                    232, 128, 197, 246, 231, 147, 164, 79, 186, 201, 127, 173, 24, 138, 207, 95
                ]),
                name: String::from("Warmonger"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [236, 50, 115, 135, 90, 78, 114, 68, 130, 207, 98, 215, 216, 238, 207, 22],
            Cosmetic {
                guid: Guid([
                    236, 50, 115, 135, 90, 78, 114, 68, 130, 207, 98, 215, 216, 238, 207, 22
                ]),
                name: String::from("Warmonger"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [194, 148, 107, 207, 19, 14, 103, 69, 139, 250, 54, 41, 117, 202, 80, 65],
            Cosmetic {
                guid: Guid([
                    194, 148, 107, 207, 19, 14, 103, 69, 139, 250, 54, 41, 117, 202, 80, 65
                ]),
                name: String::from("Trusty Rusty"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [42, 128, 32, 12, 22, 219, 10, 78, 135, 84, 17, 182, 221, 6, 166, 72],
            Cosmetic {
                guid: Guid([42, 128, 32, 12, 22, 219, 10, 78, 135, 84, 17, 182, 221, 6, 166, 72]),
                name: String::from("Trusty Rusty"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [126, 218, 225, 145, 198, 58, 245, 78, 148, 188, 147, 68, 108, 133, 94, 115],
            Cosmetic {
                guid: Guid([
                    126, 218, 225, 145, 198, 58, 245, 78, 148, 188, 147, 68, 108, 133, 94, 115
                ]),
                name: String::from("Trusty Rusty"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [120, 18, 93, 17, 245, 46, 252, 68, 148, 175, 163, 244, 206, 238, 130, 104],
            Cosmetic {
                guid: Guid([
                    120, 18, 93, 17, 245, 46, 252, 68, 148, 175, 163, 244, 206, 238, 130, 104
                ]),
                name: String::from("Trusty Rusty"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [68, 138, 6, 158, 26, 46, 97, 70, 164, 215, 161, 40, 191, 185, 91, 218],
            Cosmetic {
                guid: Guid([68, 138, 6, 158, 26, 46, 97, 70, 164, 215, 161, 40, 191, 185, 91, 218]),
                name: String::from("Tool of Destruction"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [15, 212, 75, 27, 202, 61, 82, 71, 147, 44, 182, 231, 31, 62, 150, 239],
            Cosmetic {
                guid: Guid([15, 212, 75, 27, 202, 61, 82, 71, 147, 44, 182, 231, 31, 62, 150, 239]),
                name: String::from("Tool of Destruction"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [44, 169, 41, 157, 180, 231, 181, 73, 135, 254, 213, 218, 131, 57, 197, 218],
            Cosmetic {
                guid: Guid([
                    44, 169, 41, 157, 180, 231, 181, 73, 135, 254, 213, 218, 131, 57, 197, 218
                ]),
                name: String::from("Tool of Destruction"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [182, 0, 93, 103, 249, 72, 64, 75, 180, 236, 93, 142, 101, 237, 197, 217],
            Cosmetic {
                guid: Guid([
                    182, 0, 93, 103, 249, 72, 64, 75, 180, 236, 93, 142, 101, 237, 197, 217
                ]),
                name: String::from("Tool of Destruction"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [136, 88, 44, 172, 59, 174, 17, 70, 129, 1, 169, 3, 101, 169, 212, 8],
            Cosmetic {
                guid: Guid([136, 88, 44, 172, 59, 174, 17, 70, 129, 1, 169, 3, 101, 169, 212, 8]),
                name: String::from("Primal Blood"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [46, 61, 118, 248, 22, 161, 2, 69, 140, 183, 236, 228, 10, 62, 227, 121],
            Cosmetic {
                guid: Guid([
                    46, 61, 118, 248, 22, 161, 2, 69, 140, 183, 236, 228, 10, 62, 227, 121
                ]),
                name: String::from("Primal Blood"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [166, 30, 209, 163, 66, 182, 205, 70, 144, 79, 176, 231, 43, 20, 162, 164],
            Cosmetic {
                guid: Guid([
                    166, 30, 209, 163, 66, 182, 205, 70, 144, 79, 176, 231, 43, 20, 162, 164
                ]),
                name: String::from("Primal Blood"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [131, 114, 97, 190, 19, 51, 193, 69, 180, 235, 90, 81, 107, 32, 81, 168],
            Cosmetic {
                guid: Guid([
                    131, 114, 97, 190, 19, 51, 193, 69, 180, 235, 90, 81, 107, 32, 81, 168
                ]),
                name: String::from("Primal Blood"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [38, 140, 216, 144, 198, 251, 250, 75, 164, 82, 118, 234, 47, 118, 91, 67],
            Cosmetic {
                guid: Guid([
                    38, 140, 216, 144, 198, 251, 250, 75, 164, 82, 118, 234, 47, 118, 91, 67
                ]),
                name: String::from("Mint Assault"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [215, 61, 242, 148, 42, 40, 172, 68, 135, 201, 66, 173, 160, 176, 227, 201],
            Cosmetic {
                guid: Guid([
                    215, 61, 242, 148, 42, 40, 172, 68, 135, 201, 66, 173, 160, 176, 227, 201
                ]),
                name: String::from("Mint Assault"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [137, 143, 85, 110, 127, 176, 226, 74, 130, 186, 225, 182, 50, 182, 63, 220],
            Cosmetic {
                guid: Guid([
                    137, 143, 85, 110, 127, 176, 226, 74, 130, 186, 225, 182, 50, 182, 63, 220
                ]),
                name: String::from("Mint Assault"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [10, 213, 134, 117, 235, 190, 157, 77, 159, 116, 253, 168, 240, 26, 10, 179],
            Cosmetic {
                guid: Guid([
                    10, 213, 134, 117, 235, 190, 157, 77, 159, 116, 253, 168, 240, 26, 10, 179
                ]),
                name: String::from("Mint Assault"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [84, 250, 101, 107, 181, 193, 48, 69, 176, 20, 252, 63, 166, 180, 79, 43],
            Cosmetic {
                guid: Guid([
                    84, 250, 101, 107, 181, 193, 48, 69, 176, 20, 252, 63, 166, 180, 79, 43
                ]),
                name: String::from("Metallic Vintage"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [51, 232, 188, 164, 45, 215, 65, 79, 161, 23, 163, 186, 145, 233, 3, 59],
            Cosmetic {
                guid: Guid([
                    51, 232, 188, 164, 45, 215, 65, 79, 161, 23, 163, 186, 145, 233, 3, 59
                ]),
                name: String::from("Metallic Vintage"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [0, 67, 62, 219, 206, 44, 124, 66, 158, 115, 192, 131, 245, 227, 67, 251],
            Cosmetic {
                guid: Guid([
                    0, 67, 62, 219, 206, 44, 124, 66, 158, 115, 192, 131, 245, 227, 67, 251
                ]),
                name: String::from("Metallic Vintage"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [105, 178, 130, 151, 30, 124, 124, 74, 172, 84, 89, 168, 102, 221, 165, 8],
            Cosmetic {
                guid: Guid([
                    105, 178, 130, 151, 30, 124, 124, 74, 172, 84, 89, 168, 102, 221, 165, 8
                ]),
                name: String::from("Metallic Vintage"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [241, 175, 28, 15, 250, 47, 14, 65, 187, 171, 229, 195, 225, 250, 47, 245],
            Cosmetic {
                guid: Guid([
                    241, 175, 28, 15, 250, 47, 14, 65, 187, 171, 229, 195, 225, 250, 47, 245
                ]),
                name: String::from("Jungle Raid"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [25, 24, 246, 220, 91, 131, 4, 64, 138, 133, 41, 223, 193, 8, 40, 38],
            Cosmetic {
                guid: Guid([25, 24, 246, 220, 91, 131, 4, 64, 138, 133, 41, 223, 193, 8, 40, 38]),
                name: String::from("Jungle Raid"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [44, 150, 167, 241, 165, 141, 212, 64, 132, 195, 14, 193, 62, 146, 122, 207],
            Cosmetic {
                guid: Guid([
                    44, 150, 167, 241, 165, 141, 212, 64, 132, 195, 14, 193, 62, 146, 122, 207
                ]),
                name: String::from("Jungle Raid"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [3, 221, 148, 126, 36, 126, 49, 73, 138, 70, 93, 228, 225, 197, 222, 253],
            Cosmetic {
                guid: Guid([
                    3, 221, 148, 126, 36, 126, 49, 73, 138, 70, 93, 228, 225, 197, 222, 253
                ]),
                name: String::from("Jungle Raid"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [124, 74, 128, 65, 250, 181, 55, 68, 131, 87, 130, 220, 188, 104, 11, 243],
            Cosmetic {
                guid: Guid([
                    124, 74, 128, 65, 250, 181, 55, 68, 131, 87, 130, 220, 188, 104, 11, 243
                ]),
                name: String::from("Ghostly Pale"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [133, 0, 183, 85, 68, 238, 172, 65, 173, 72, 72, 111, 54, 102, 35, 82],
            Cosmetic {
                guid: Guid([133, 0, 183, 85, 68, 238, 172, 65, 173, 72, 72, 111, 54, 102, 35, 82]),
                name: String::from("Ghostly Pale"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [170, 41, 197, 45, 200, 227, 193, 68, 166, 74, 245, 8, 83, 84, 217, 93],
            Cosmetic {
                guid: Guid([170, 41, 197, 45, 200, 227, 193, 68, 166, 74, 245, 8, 83, 84, 217, 93]),
                name: String::from("Ghostly Pale"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [174, 179, 31, 52, 196, 103, 171, 71, 177, 163, 23, 29, 1, 32, 66, 128],
            Cosmetic {
                guid: Guid([174, 179, 31, 52, 196, 103, 171, 71, 177, 163, 23, 29, 1, 32, 66, 128]),
                name: String::from("Ghostly Pale"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [221, 9, 158, 24, 169, 119, 133, 75, 141, 89, 141, 198, 245, 113, 207, 209],
            Cosmetic {
                guid: Guid([
                    221, 9, 158, 24, 169, 119, 133, 75, 141, 89, 141, 198, 245, 113, 207, 209
                ]),
                name: String::from("Digital Danger"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [173, 102, 55, 203, 220, 65, 41, 68, 138, 213, 234, 213, 80, 219, 216, 33],
            Cosmetic {
                guid: Guid([
                    173, 102, 55, 203, 220, 65, 41, 68, 138, 213, 234, 213, 80, 219, 216, 33
                ]),
                name: String::from("Digital Danger"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [203, 22, 242, 17, 209, 113, 203, 74, 175, 11, 202, 102, 84, 233, 15, 52],
            Cosmetic {
                guid: Guid([
                    203, 22, 242, 17, 209, 113, 203, 74, 175, 11, 202, 102, 84, 233, 15, 52
                ]),
                name: String::from("Digital Danger"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [227, 117, 125, 151, 25, 209, 213, 79, 152, 18, 214, 135, 96, 254, 231, 56],
            Cosmetic {
                guid: Guid([
                    227, 117, 125, 151, 25, 209, 213, 79, 152, 18, 214, 135, 96, 254, 231, 56
                ]),
                name: String::from("Digital Danger"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [233, 10, 136, 255, 28, 95, 33, 70, 155, 5, 193, 39, 233, 30, 116, 46],
            Cosmetic {
                guid: Guid([233, 10, 136, 255, 28, 95, 33, 70, 155, 5, 193, 39, 233, 30, 116, 46]),
                name: String::from("Desert Ranger"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [21, 90, 243, 139, 190, 155, 88, 69, 149, 46, 81, 96, 87, 251, 88, 167],
            Cosmetic {
                guid: Guid([21, 90, 243, 139, 190, 155, 88, 69, 149, 46, 81, 96, 87, 251, 88, 167]),
                name: String::from("Desert Ranger"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [124, 36, 233, 179, 105, 88, 145, 74, 152, 139, 68, 34, 139, 40, 148, 122],
            Cosmetic {
                guid: Guid([
                    124, 36, 233, 179, 105, 88, 145, 74, 152, 139, 68, 34, 139, 40, 148, 122
                ]),
                name: String::from("Desert Ranger"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [48, 245, 184, 74, 182, 193, 99, 70, 154, 129, 122, 127, 178, 76, 4, 217],
            Cosmetic {
                guid: Guid([
                    48, 245, 184, 74, 182, 193, 99, 70, 154, 129, 122, 127, 178, 76, 4, 217
                ]),
                name: String::from("Desert Ranger"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [39, 30, 147, 61, 2, 122, 193, 67, 180, 166, 109, 234, 239, 119, 81, 242],
            Cosmetic {
                guid: Guid([
                    39, 30, 147, 61, 2, 122, 193, 67, 180, 166, 109, 234, 239, 119, 81, 242
                ]),
                name: String::from("Dark Descent"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [151, 188, 150, 218, 219, 201, 69, 64, 134, 250, 90, 216, 105, 164, 178, 103],
            Cosmetic {
                guid: Guid([
                    151, 188, 150, 218, 219, 201, 69, 64, 134, 250, 90, 216, 105, 164, 178, 103
                ]),
                name: String::from("Dark Descent"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [64, 237, 207, 2, 112, 193, 77, 73, 166, 217, 162, 171, 191, 185, 199, 43],
            Cosmetic {
                guid: Guid([
                    64, 237, 207, 2, 112, 193, 77, 73, 166, 217, 162, 171, 191, 185, 199, 43
                ]),
                name: String::from("Dark Descent"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [66, 54, 81, 238, 75, 210, 24, 71, 141, 170, 192, 230, 237, 164, 84, 164],
            Cosmetic {
                guid: Guid([
                    66, 54, 81, 238, 75, 210, 24, 71, 141, 170, 192, 230, 237, 164, 84, 164
                ]),
                name: String::from("Dark Descent"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [120, 253, 99, 10, 110, 23, 25, 72, 174, 121, 113, 250, 110, 45, 137, 56],
            Cosmetic {
                guid: Guid([
                    120, 253, 99, 10, 110, 23, 25, 72, 174, 121, 113, 250, 110, 45, 137, 56
                ]),
                name: String::from("Beyond The Circuit"),
                dwarf: Driller,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [109, 16, 68, 159, 36, 221, 102, 75, 154, 116, 187, 197, 242, 122, 203, 69],
            Cosmetic {
                guid: Guid([
                    109, 16, 68, 159, 36, 221, 102, 75, 154, 116, 187, 197, 242, 122, 203, 69
                ]),
                name: String::from("Beyond The Circuit"),
                dwarf: Engineer,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [24, 164, 108, 209, 6, 219, 145, 78, 169, 126, 131, 150, 156, 178, 245, 170],
            Cosmetic {
                guid: Guid([
                    24, 164, 108, 209, 6, 219, 145, 78, 169, 126, 131, 150, 156, 178, 245, 170
                ]),
                name: String::from("Beyond The Circuit"),
                dwarf: Gunner,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [0, 167, 113, 255, 93, 14, 10, 74, 153, 234, 136, 116, 34, 22, 151, 159],
            Cosmetic {
                guid: Guid([
                    0, 167, 113, 255, 93, 14, 10, 74, 153, 234, 136, 116, 34, 22, 151, 159
                ]),
                name: String::from("Beyond The Circuit"),
                dwarf: Scout,
                ty: PaintJob,
                status: None
            }
        ),
        (
            [175, 148, 91, 147, 167, 185, 214, 76, 166, 221, 0, 104, 54, 39, 188, 128],
            Overclock {
                guid: Guid([
                    175, 148, 91, 147, 167, 185, 214, 76, 166, 221, 0, 104, 54, 39, 188, 128
                ]),
                name: String::from("Compact Ammo"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [255, 148, 185, 231, 131, 77, 119, 66, 146, 218, 204, 202, 62, 160, 35, 185],
            Overclock {
                guid: Guid([
                    255, 148, 185, 231, 131, 77, 119, 66, 146, 218, 204, 202, 62, 160, 35, 185
                ]),
                name: String::from("Electrifying Reload"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [17, 241, 35, 225, 116, 16, 49, 64, 149, 121, 87, 80, 107, 213, 118, 215],
            Overclock {
                guid: Guid([
                    17, 241, 35, 225, 116, 16, 49, 64, 149, 121, 87, 80, 107, 213, 118, 215
                ]),
                name: String::from("AI Stability Engine"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [76, 223, 65, 243, 160, 248, 228, 73, 157, 27, 172, 97, 104, 52, 119, 153],
            Overclock {
                guid: Guid([
                    76, 223, 65, 243, 160, 248, 228, 73, 157, 27, 172, 97, 104, 52, 119, 153
                ]),
                name: String::from("Homebrew Powder"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [195, 6, 3, 36, 206, 72, 44, 74, 182, 220, 68, 237, 73, 140, 186, 57],
            Overclock {
                guid: Guid([195, 6, 3, 36, 206, 72, 44, 74, 182, 220, 68, 237, 73, 140, 186, 57]),
                name: String::from("Overclocked Firing Mechanism"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [246, 226, 229, 71, 242, 235, 103, 77, 189, 165, 145, 244, 125, 230, 208, 23],
            Overclock {
                guid: Guid([
                    246, 226, 229, 71, 242, 235, 103, 77, 189, 165, 145, 244, 125, 230, 208, 23
                ]),
                name: String::from("Gas Rerouting"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [115, 6, 34, 31, 18, 100, 176, 75, 140, 2, 93, 180, 222, 157, 22, 103],
            Overclock {
                guid: Guid([115, 6, 34, 31, 18, 100, 176, 75, 140, 2, 93, 180, 222, 157, 22, 103]),
                name: String::from("Bullets of Mercy"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [134, 170, 13, 209, 63, 211, 126, 67, 178, 251, 241, 118, 238, 93, 232, 21],
            Overclock {
                guid: Guid([
                    134, 170, 13, 209, 63, 211, 126, 67, 178, 251, 241, 118, 238, 93, 232, 21
                ]),
                name: String::from("Big Bertha"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [140, 1, 66, 166, 135, 99, 122, 78, 164, 204, 138, 127, 132, 185, 97, 10],
            Overclock {
                guid: Guid([
                    140, 1, 66, 166, 135, 99, 122, 78, 164, 204, 138, 127, 132, 185, 97, 10
                ]),
                name: String::from("Carpet Bomber"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [1, 226, 48, 103, 203, 185, 166, 66, 138, 228, 57, 76, 153, 241, 210, 187],
            Overclock {
                guid: Guid([
                    1, 226, 48, 103, 203, 185, 166, 66, 138, 228, 57, 76, 153, 241, 210, 187
                ]),
                name: String::from("Composite Drums"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [61, 213, 232, 20, 28, 72, 62, 67, 170, 62, 7, 8, 206, 117, 214, 99],
            Overclock {
                guid: Guid([61, 213, 232, 20, 28, 72, 62, 67, 170, 62, 7, 8, 206, 117, 214, 99]),
                name: String::from("Combat Mobility"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [50, 101, 78, 148, 120, 22, 94, 77, 163, 240, 219, 211, 59, 24, 3, 65],
            Overclock {
                guid: Guid([50, 101, 78, 148, 120, 22, 94, 77, 163, 240, 219, 211, 59, 24, 3, 65]),
                name: String::from("Neurotoxin Payload"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [203, 131, 253, 108, 19, 198, 63, 73, 130, 220, 135, 185, 121, 99, 17, 200],
            Overclock {
                guid: Guid([
                    203, 131, 253, 108, 19, 198, 63, 73, 130, 220, 135, 185, 121, 99, 17, 200
                ]),
                name: String::from("Splintering Shells"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [208, 6, 227, 238, 224, 84, 2, 66, 162, 176, 188, 168, 171, 205, 179, 135],
            Overclock {
                guid: Guid([
                    208, 6, 227, 238, 224, 84, 2, 66, 162, 176, 188, 168, 171, 205, 179, 135
                ]),
                name: String::from("Minimal Clips"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [16, 115, 206, 235, 236, 188, 1, 78, 131, 112, 226, 89, 4, 14, 183, 28],
            Overclock {
                guid: Guid([16, 115, 206, 235, 236, 188, 1, 78, 131, 112, 226, 89, 4, 14, 183, 28]),
                name: String::from("Supercooling Chamber"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [170, 201, 182, 228, 69, 132, 88, 71, 142, 80, 219, 135, 50, 64, 151, 235],
            Overclock {
                guid: Guid([
                    170, 201, 182, 228, 69, 132, 88, 71, 142, 80, 219, 135, 50, 64, 151, 235
                ]),
                name: String::from("Electrocuting Focus Shots"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [42, 18, 53, 143, 117, 11, 112, 73, 150, 195, 125, 70, 126, 207, 29, 54],
            Overclock {
                guid: Guid([
                    42, 18, 53, 143, 117, 11, 112, 73, 150, 195, 125, 70, 126, 207, 29, 54
                ]),
                name: String::from("Hipster"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [138, 1, 34, 228, 78, 66, 16, 75, 170, 173, 107, 69, 52, 169, 222, 143],
            Overclock {
                guid: Guid([138, 1, 34, 228, 78, 66, 16, 75, 170, 173, 107, 69, 52, 169, 222, 143]),
                name: String::from("Hoverclock"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [209, 84, 75, 53, 40, 147, 138, 72, 150, 78, 141, 154, 99, 9, 209, 218],
            Overclock {
                guid: Guid([209, 84, 75, 53, 40, 147, 138, 72, 150, 78, 141, 154, 99, 9, 209, 218]),
                name: String::from("Active Stability System"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [111, 38, 168, 180, 159, 150, 124, 77, 153, 159, 115, 70, 69, 234, 226, 196],
            Overclock {
                guid: Guid([
                    111, 38, 168, 180, 159, 150, 124, 77, 153, 159, 115, 70, 69, 234, 226, 196
                ]),
                name: String::from("Composite Casings"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [47, 243, 212, 201, 45, 89, 48, 69, 146, 0, 159, 162, 227, 155, 255, 198],
            Overclock {
                guid: Guid([
                    47, 243, 212, 201, 45, 89, 48, 69, 146, 0, 159, 162, 227, 155, 255, 198
                ]),
                name: String::from("Compact Mags"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [74, 79, 147, 254, 26, 195, 30, 65, 184, 151, 193, 241, 178, 100, 166, 114],
            Overclock {
                guid: Guid([
                    74, 79, 147, 254, 26, 195, 30, 65, 184, 151, 193, 241, 178, 100, 166, 114
                ]),
                name: String::from("Electro Minelets"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [65, 73, 22, 149, 122, 211, 208, 64, 155, 95, 227, 105, 42, 70, 221, 25],
            Overclock {
                guid: Guid([
                    65, 73, 22, 149, 122, 211, 208, 64, 155, 95, 227, 105, 42, 70, 221, 25
                ]),
                name: String::from("Full Chamber Seal"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [136, 171, 212, 10, 120, 50, 153, 66, 136, 76, 0, 215, 242, 45, 172, 144],
            Overclock {
                guid: Guid([
                    136, 171, 212, 10, 120, 50, 153, 66, 136, 76, 0, 215, 242, 45, 172, 144
                ]),
                name: String::from("Experimental Rounds"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [143, 28, 106, 26, 35, 166, 223, 76, 188, 165, 81, 12, 74, 25, 242, 33],
            Overclock {
                guid: Guid([143, 28, 106, 26, 35, 166, 223, 76, 188, 165, 81, 12, 74, 25, 242, 33]),
                name: String::from("Micro Flechettes"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [241, 174, 85, 207, 89, 46, 119, 70, 160, 0, 29, 97, 88, 75, 223, 42],
            Overclock {
                guid: Guid([241, 174, 85, 207, 89, 46, 119, 70, 160, 0, 29, 97, 88, 75, 223, 42]),
                name: String::from("Lead Spray"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [160, 224, 24, 196, 95, 230, 121, 65, 143, 211, 115, 14, 110, 155, 99, 29],
            Overclock {
                guid: Guid([
                    160, 224, 24, 196, 95, 230, 121, 65, 143, 211, 115, 14, 110, 155, 99, 29
                ]),
                name: String::from("Heat Pipe"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [85, 187, 82, 160, 236, 250, 124, 67, 179, 123, 187, 230, 64, 17, 65, 150],
            Overclock {
                guid: Guid([
                    85, 187, 82, 160, 236, 250, 124, 67, 179, 123, 187, 230, 64, 17, 65, 150
                ]),
                name: String::from("Overcharger"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [151, 130, 32, 113, 213, 134, 206, 69, 172, 214, 72, 151, 130, 183, 0, 250],
            Overclock {
                guid: Guid([
                    151, 130, 32, 113, 213, 134, 206, 69, 172, 214, 72, 151, 130, 183, 0, 250
                ]),
                name: String::from("Energy Rerouting"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [241, 230, 204, 48, 124, 17, 222, 69, 134, 184, 139, 65, 158, 83, 10, 191],
            Overclock {
                guid: Guid([
                    241, 230, 204, 48, 124, 17, 222, 69, 134, 184, 139, 65, 158, 83, 10, 191
                ]),
                name: String::from("Magnetic Cooling Unit"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [57, 228, 62, 19, 204, 13, 174, 70, 145, 221, 81, 161, 19, 131, 233, 225],
            Overclock {
                guid: Guid([
                    57, 228, 62, 19, 204, 13, 174, 70, 145, 221, 81, 161, 19, 131, 233, 225
                ]),
                name: String::from("Heavy Hitter"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [22, 92, 217, 62, 102, 11, 129, 71, 188, 162, 248, 108, 222, 92, 248, 247],
            Overclock {
                guid: Guid([
                    22, 92, 217, 62, 102, 11, 129, 71, 188, 162, 248, 108, 222, 92, 248, 247
                ]),
                name: String::from("Persistent Plasma"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [128, 222, 169, 10, 213, 243, 23, 70, 149, 19, 143, 244, 122, 50, 169, 106],
            Overclock {
                guid: Guid([
                    128, 222, 169, 10, 213, 243, 23, 70, 149, 19, 143, 244, 122, 50, 169, 106
                ]),
                name: String::from("Hellfire"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [248, 47, 50, 101, 118, 232, 167, 72, 153, 117, 60, 182, 49, 107, 124, 32],
            Overclock {
                guid: Guid([
                    248, 47, 50, 101, 118, 232, 167, 72, 153, 117, 60, 182, 49, 107, 124, 32
                ]),
                name: String::from("Backfeeding Module"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [86, 71, 31, 33, 155, 224, 20, 68, 141, 254, 95, 8, 124, 171, 212, 71],
            Overclock {
                guid: Guid([86, 71, 31, 33, 155, 224, 20, 68, 141, 254, 95, 8, 124, 171, 212, 71]),
                name: String::from("The Mole"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [140, 93, 36, 189, 104, 213, 241, 74, 137, 166, 184, 122, 133, 124, 205, 155],
            Overclock {
                guid: Guid([
                    140, 93, 36, 189, 104, 213, 241, 74, 137, 166, 184, 122, 133, 124, 205, 155
                ]),
                name: String::from("Re-atomizer"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [38, 132, 30, 181, 130, 58, 40, 75, 129, 191, 125, 163, 121, 39, 8, 215],
            Overclock {
                guid: Guid([
                    38, 132, 30, 181, 130, 58, 40, 75, 129, 191, 125, 163, 121, 39, 8, 215
                ]),
                name: String::from("Ultra-magnetic Coils"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [166, 117, 29, 161, 184, 124, 19, 78, 188, 87, 16, 199, 67, 28, 20, 83],
            Overclock {
                guid: Guid([166, 117, 29, 161, 184, 124, 19, 78, 188, 87, 16, 199, 67, 28, 20, 83]),
                name: String::from("Triple-Tech Chambers"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [205, 78, 236, 56, 192, 41, 114, 78, 131, 23, 204, 220, 34, 208, 96, 87],
            Overclock {
                guid: Guid([
                    205, 78, 236, 56, 192, 41, 114, 78, 131, 23, 204, 220, 34, 208, 96, 87
                ]),
                name: String::from("Light-weight Magazines"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [51, 141, 210, 56, 51, 42, 46, 78, 165, 73, 2, 8, 170, 215, 245, 2],
            Overclock {
                guid: Guid([51, 141, 210, 56, 51, 42, 46, 78, 165, 73, 2, 8, 170, 215, 245, 2]),
                name: String::from("Stunner"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [98, 212, 201, 76, 19, 103, 232, 65, 150, 145, 182, 119, 131, 125, 164, 251],
            Overclock {
                guid: Guid([
                    98, 212, 201, 76, 19, 103, 232, 65, 150, 145, 182, 119, 131, 125, 164, 251
                ]),
                name: String::from("Mini Shells"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [112, 18, 43, 162, 251, 218, 191, 72, 149, 220, 9, 201, 2, 148, 230, 133],
            Overclock {
                guid: Guid([
                    112, 18, 43, 162, 251, 218, 191, 72, 149, 220, 9, 201, 2, 148, 230, 133
                ]),
                name: String::from("Cycle Overload"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [7, 233, 39, 153, 86, 191, 156, 65, 142, 234, 49, 62, 45, 197, 159, 141],
            Overclock {
                guid: Guid([
                    7, 233, 39, 153, 86, 191, 156, 65, 142, 234, 49, 62, 45, 197, 159, 141
                ]),
                name: String::from("Magnetic Pellet Alignment"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [174, 203, 15, 36, 29, 250, 25, 74, 176, 75, 187, 134, 86, 63, 168, 64],
            Overclock {
                guid: Guid([174, 203, 15, 36, 29, 250, 25, 74, 176, 75, 187, 134, 86, 63, 168, 64]),
                name: String::from("Bodkin Points"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [117, 207, 208, 169, 15, 49, 201, 74, 157, 106, 141, 2, 20, 125, 225, 252],
            Overclock {
                guid: Guid([
                    117, 207, 208, 169, 15, 49, 201, 74, 157, 106, 141, 2, 20, 125, 225, 252
                ]),
                name: String::from("Cryo Bolt"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [221, 202, 129, 226, 156, 107, 232, 72, 140, 45, 244, 240, 176, 199, 42, 114],
            Overclock {
                guid: Guid([
                    221, 202, 129, 226, 156, 107, 232, 72, 140, 45, 244, 240, 176, 199, 42, 114
                ]),
                name: String::from("Fire Bolt"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [28, 250, 87, 250, 53, 208, 219, 73, 169, 170, 248, 124, 170, 1, 30, 244],
            Overclock {
                guid: Guid([
                    28, 250, 87, 250, 53, 208, 219, 73, 169, 170, 248, 124, 170, 1, 30, 244
                ]),
                name: String::from("Quick Fire"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [108, 70, 69, 49, 130, 14, 168, 73, 187, 60, 128, 192, 241, 144, 167, 79],
            Overclock {
                guid: Guid([
                    108, 70, 69, 49, 130, 14, 168, 73, 187, 60, 128, 192, 241, 144, 167, 79
                ]),
                name: String::from("The Specialist"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [98, 254, 150, 222, 50, 165, 69, 78, 176, 241, 149, 43, 214, 141, 186, 244],
            Overclock {
                guid: Guid([
                    98, 254, 150, 222, 50, 165, 69, 78, 176, 241, 149, 43, 214, 141, 186, 244
                ]),
                name: String::from("Trifork Volley"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [206, 102, 12, 77, 217, 152, 0, 67, 168, 2, 161, 122, 69, 105, 192, 172],
            Overclock {
                guid: Guid([
                    206, 102, 12, 77, 217, 152, 0, 67, 168, 2, 161, 122, 69, 105, 192, 172
                ]),
                name: String::from("Improved Thermal Efficiency"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [106, 219, 35, 179, 200, 237, 136, 64, 134, 168, 139, 129, 7, 201, 60, 190],
            Overclock {
                guid: Guid([
                    106, 219, 35, 179, 200, 237, 136, 64, 134, 168, 139, 129, 7, 201, 60, 190
                ]),
                name: String::from("Tuned Cooler"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [136, 248, 234, 178, 6, 236, 123, 70, 163, 28, 143, 3, 75, 68, 147, 26],
            Overclock {
                guid: Guid([136, 248, 234, 178, 6, 236, 123, 70, 163, 28, 143, 3, 75, 68, 147, 26]),
                name: String::from("Flow Rate Expansion"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [123, 17, 117, 89, 151, 182, 86, 78, 151, 177, 3, 42, 130, 142, 185, 15],
            Overclock {
                guid: Guid([
                    123, 17, 117, 89, 151, 182, 86, 78, 151, 177, 3, 42, 130, 142, 185, 15
                ]),
                name: String::from("Ice Storm"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [213, 24, 203, 55, 26, 50, 160, 68, 155, 52, 94, 79, 88, 211, 76, 145],
            Overclock {
                guid: Guid([213, 24, 203, 55, 26, 50, 160, 68, 155, 52, 94, 79, 88, 211, 76, 145]),
                name: String::from("Ice Spear"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [193, 241, 111, 151, 12, 156, 16, 71, 175, 131, 107, 246, 203, 254, 188, 173],
            Overclock {
                guid: Guid([
                    193, 241, 111, 151, 12, 156, 16, 71, 175, 131, 107, 246, 203, 254, 188, 173
                ]),
                name: String::from("Snowball"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [67, 205, 141, 39, 234, 234, 170, 74, 142, 109, 196, 62, 4, 56, 161, 12],
            Overclock {
                guid: Guid([
                    67, 205, 141, 39, 234, 234, 170, 74, 142, 109, 196, 62, 4, 56, 161, 12
                ]),
                name: String::from("Cryo Minelets"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [36, 162, 134, 195, 31, 69, 234, 70, 154, 32, 16, 42, 244, 215, 65, 211],
            Overclock {
                guid: Guid([
                    36, 162, 134, 195, 31, 69, 234, 70, 154, 32, 16, 42, 244, 215, 65, 211
                ]),
                name: String::from("Custom Casings"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [130, 139, 43, 188, 80, 236, 66, 67, 133, 239, 208, 14, 166, 133, 69, 151],
            Overclock {
                guid: Guid([
                    130, 139, 43, 188, 80, 236, 66, 67, 133, 239, 208, 14, 166, 133, 69, 151
                ]),
                name: String::from("Gas Recycling"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [250, 243, 80, 113, 186, 99, 178, 66, 152, 85, 251, 200, 160, 155, 255, 208],
            Overclock {
                guid: Guid([
                    250, 243, 80, 113, 186, 99, 178, 66, 152, 85, 251, 200, 160, 155, 255, 208
                ]),
                name: String::from("Embedded Detonators"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [152, 160, 69, 43, 35, 84, 2, 69, 156, 255, 92, 195, 179, 76, 204, 105],
            Overclock {
                guid: Guid([152, 160, 69, 43, 35, 84, 2, 69, 156, 255, 92, 195, 179, 76, 204, 105]),
                name: String::from("Minimal Magazines"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [113, 88, 141, 67, 167, 88, 154, 74, 190, 97, 168, 222, 46, 180, 73, 191],
            Overclock {
                guid: Guid([
                    113, 88, 141, 67, 167, 88, 154, 74, 190, 97, 168, 222, 46, 180, 73, 191
                ]),
                name: String::from("Compact Feed Valves"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [100, 49, 168, 221, 234, 166, 137, 75, 144, 197, 123, 172, 117, 82, 165, 255],
            Overclock {
                guid: Guid([
                    100, 49, 168, 221, 234, 166, 137, 75, 144, 197, 123, 172, 117, 82, 165, 255
                ]),
                name: String::from("Face Melter"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [197, 5, 122, 150, 8, 207, 94, 75, 164, 170, 179, 42, 214, 19, 186, 62],
            Overclock {
                guid: Guid([197, 5, 122, 150, 8, 207, 94, 75, 164, 170, 179, 42, 214, 19, 186, 62]),
                name: String::from("Lighter Tanks"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [25, 232, 93, 28, 53, 133, 179, 73, 177, 81, 119, 156, 189, 115, 3, 217],
            Overclock {
                guid: Guid([
                    25, 232, 93, 28, 53, 133, 179, 73, 177, 81, 119, 156, 189, 115, 3, 217
                ]),
                name: String::from("Fuel Stream Diffuser"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [242, 140, 62, 227, 138, 150, 13, 72, 150, 94, 159, 105, 245, 107, 63, 25],
            Overclock {
                guid: Guid([
                    242, 140, 62, 227, 138, 150, 13, 72, 150, 94, 159, 105, 245, 107, 63, 25
                ]),
                name: String::from("Sticky Fuel"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [65, 95, 29, 131, 167, 126, 122, 68, 141, 191, 96, 56, 49, 3, 253, 108],
            Overclock {
                guid: Guid([65, 95, 29, 131, 167, 126, 122, 68, 141, 191, 96, 56, 49, 3, 253, 108]),
                name: String::from("Sticky Additive"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [46, 159, 191, 180, 59, 123, 85, 79, 179, 114, 135, 128, 241, 51, 72, 36],
            Overclock {
                guid: Guid([
                    46, 159, 191, 180, 59, 123, 85, 79, 179, 114, 135, 128, 241, 51, 72, 36
                ]),
                name: String::from("Compact Feed Mechanism"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [1, 212, 101, 57, 3, 131, 47, 73, 140, 80, 247, 113, 157, 88, 232, 155],
            Overclock {
                guid: Guid([1, 212, 101, 57, 3, 131, 47, 73, 140, 80, 247, 113, 157, 88, 232, 155]),
                name: String::from("Thinned Drum Walls"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [124, 209, 14, 44, 85, 88, 48, 65, 181, 119, 20, 28, 133, 75, 238, 98],
            Overclock {
                guid: Guid([124, 209, 14, 44, 85, 88, 48, 65, 181, 119, 20, 28, 133, 75, 238, 98]),
                name: String::from("Lead Storm"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [251, 177, 1, 71, 88, 96, 106, 65, 136, 142, 55, 143, 105, 227, 205, 97],
            Overclock {
                guid: Guid([
                    251, 177, 1, 71, 88, 96, 106, 65, 136, 142, 55, 143, 105, 227, 205, 97
                ]),
                name: String::from("Exhaust Vectoring"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [181, 93, 97, 112, 238, 165, 215, 67, 172, 75, 40, 41, 60, 12, 70, 115],
            Overclock {
                guid: Guid([181, 93, 97, 112, 238, 165, 215, 67, 172, 75, 40, 41, 60, 12, 70, 115]),
                name: String::from("A little more oomph!"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [99, 21, 102, 115, 242, 23, 79, 68, 159, 89, 143, 135, 41, 172, 12, 101],
            Overclock {
                guid: Guid([
                    99, 21, 102, 115, 242, 23, 79, 68, 159, 89, 143, 135, 41, 172, 12, 101
                ]),
                name: String::from("Burning Hell"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [237, 64, 54, 128, 106, 105, 232, 72, 188, 114, 221, 227, 205, 234, 196, 86],
            Overclock {
                guid: Guid([
                    237, 64, 54, 128, 106, 105, 232, 72, 188, 114, 221, 227, 205, 234, 196, 86
                ]),
                name: String::from("Bullet Hell"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [193, 215, 122, 139, 175, 212, 51, 64, 142, 74, 131, 151, 241, 166, 58, 236],
            Overclock {
                guid: Guid([
                    193, 215, 122, 139, 175, 212, 51, 64, 142, 74, 131, 151, 241, 166, 58, 236
                ]),
                name: String::from("Disperser Compound"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [76, 204, 126, 153, 65, 99, 161, 74, 170, 201, 122, 179, 119, 127, 241, 249],
            Overclock {
                guid: Guid([
                    76, 204, 126, 153, 65, 99, 161, 74, 170, 201, 122, 179, 119, 127, 241, 249
                ]),
                name: String::from("Goo Bomber Special"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [73, 86, 103, 43, 166, 141, 58, 67, 157, 78, 194, 96, 240, 185, 140, 228],
            Overclock {
                guid: Guid([
                    73, 86, 103, 43, 166, 141, 58, 67, 157, 78, 194, 96, 240, 185, 140, 228
                ]),
                name: String::from("AG Mixture"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [236, 202, 178, 85, 20, 5, 240, 68, 169, 63, 251, 108, 81, 255, 155, 57],
            Overclock {
                guid: Guid([
                    236, 202, 178, 85, 20, 5, 240, 68, 169, 63, 251, 108, 81, 255, 155, 57
                ]),
                name: String::from("Hydrogen Ion Additive"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [83, 159, 22, 208, 236, 165, 4, 71, 178, 27, 211, 177, 173, 212, 57, 203],
            Overclock {
                guid: Guid([
                    83, 159, 22, 208, 236, 165, 4, 71, 178, 27, 211, 177, 173, 212, 57, 203
                ]),
                name: String::from("Volatile Impact Mixture"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [234, 125, 39, 59, 86, 9, 155, 75, 159, 68, 118, 138, 9, 154, 140, 28],
            Overclock {
                guid: Guid([234, 125, 39, 59, 86, 9, 155, 75, 159, 68, 118, 138, 9, 154, 140, 28]),
                name: String::from("Sludge Blast"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [169, 118, 148, 225, 169, 183, 72, 78, 182, 110, 20, 219, 248, 48, 219, 107],
            Overclock {
                guid: Guid([
                    169, 118, 148, 225, 169, 183, 72, 78, 182, 110, 20, 219, 248, 48, 219, 107
                ]),
                name: String::from("Pack Rat"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [238, 87, 19, 250, 145, 160, 226, 69, 161, 192, 191, 206, 21, 33, 32, 116],
            Overclock {
                guid: Guid([
                    238, 87, 19, 250, 145, 160, 226, 69, 161, 192, 191, 206, 21, 33, 32, 116
                ]),
                name: String::from("Compact Rounds"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [135, 210, 67, 67, 136, 211, 252, 69, 136, 78, 53, 29, 203, 231, 15, 124],
            Overclock {
                guid: Guid([
                    135, 210, 67, 67, 136, 211, 252, 69, 136, 78, 53, 29, 203, 231, 15, 124
                ]),
                name: String::from("Fat Boy"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [185, 18, 5, 154, 63, 134, 228, 75, 162, 152, 84, 80, 209, 54, 204, 181],
            Overclock {
                guid: Guid([
                    185, 18, 5, 154, 63, 134, 228, 75, 162, 152, 84, 80, 209, 54, 204, 181
                ]),
                name: String::from("Clean Sweep"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [219, 126, 199, 136, 180, 111, 206, 76, 152, 0, 162, 80, 16, 160, 232, 234],
            Overclock {
                guid: Guid([
                    219, 126, 199, 136, 180, 111, 206, 76, 152, 0, 162, 80, 16, 160, 232, 234
                ]),
                name: String::from("RJ250 Compound"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [188, 76, 214, 132, 98, 27, 246, 68, 150, 7, 55, 58, 83, 74, 87, 79],
            Overclock {
                guid: Guid([188, 76, 214, 132, 98, 27, 246, 68, 150, 7, 55, 58, 83, 74, 87, 79]),
                name: String::from("Hyper Propellant"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [241, 136, 12, 111, 123, 239, 16, 71, 142, 119, 238, 79, 198, 52, 129, 246],
            Overclock {
                guid: Guid([
                    241, 136, 12, 111, 123, 239, 16, 71, 142, 119, 238, 79, 198, 52, 129, 246
                ]),
                name: String::from("Feedback Loop"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [113, 216, 27, 195, 219, 194, 143, 77, 144, 90, 108, 159, 74, 139, 85, 174],
            Overclock {
                guid: Guid([
                    113, 216, 27, 195, 219, 194, 143, 77, 144, 90, 108, 159, 74, 139, 85, 174
                ]),
                name: String::from("Automated Beam Controller"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [211, 48, 114, 57, 105, 188, 228, 76, 177, 235, 233, 0, 154, 47, 110, 13],
            Overclock {
                guid: Guid([
                    211, 48, 114, 57, 105, 188, 228, 76, 177, 235, 233, 0, 154, 47, 110, 13
                ]),
                name: String::from("Plastcrete Catalist"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [241, 67, 47, 18, 31, 104, 224, 77, 160, 197, 221, 142, 80, 207, 113, 125],
            Overclock {
                guid: Guid([
                    241, 67, 47, 18, 31, 104, 224, 77, 160, 197, 221, 142, 80, 207, 113, 125
                ]),
                name: String::from("Efficiency Tweaks"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [24, 122, 24, 186, 82, 131, 210, 76, 144, 90, 55, 244, 251, 18, 59, 160],
            Overclock {
                guid: Guid([
                    24, 122, 24, 186, 82, 131, 210, 76, 144, 90, 55, 244, 251, 18, 59, 160
                ]),
                name: String::from("Overdrive Booster"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [123, 183, 194, 31, 161, 209, 230, 74, 140, 0, 124, 230, 160, 35, 164, 18],
            Overclock {
                guid: Guid([
                    123, 183, 194, 31, 161, 209, 230, 74, 140, 0, 124, 230, 160, 35, 164, 18
                ]),
                name: String::from("Volatile Impact Reactor"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [150, 148, 12, 124, 220, 239, 168, 64, 132, 221, 77, 96, 210, 232, 27, 45],
            Overclock {
                guid: Guid([
                    150, 148, 12, 124, 220, 239, 168, 64, 132, 221, 77, 96, 210, 232, 27, 45
                ]),
                name: String::from("Light-weight Cases"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [61, 124, 156, 133, 246, 211, 165, 73, 162, 214, 237, 176, 153, 24, 49, 75],
            Overclock {
                guid: Guid([
                    61, 124, 156, 133, 246, 211, 165, 73, 162, 214, 237, 176, 153, 24, 49, 75
                ]),
                name: String::from("Stronger Plasma Current"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [127, 161, 132, 203, 157, 33, 135, 77, 130, 191, 157, 218, 252, 124, 145, 215],
            Overclock {
                guid: Guid([
                    127, 161, 132, 203, 157, 33, 135, 77, 130, 191, 157, 218, 252, 124, 145, 215
                ]),
                name: String::from("High Voltage Crossover"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [142, 217, 18, 248, 38, 123, 132, 68, 130, 238, 218, 134, 96, 68, 81, 165],
            Overclock {
                guid: Guid([
                    142, 217, 18, 248, 38, 123, 132, 68, 130, 238, 218, 134, 96, 68, 81, 165
                ]),
                name: String::from("Inferno"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [116, 66, 81, 237, 37, 212, 72, 74, 163, 141, 251, 219, 244, 59, 197, 185],
            Overclock {
                guid: Guid([
                    116, 66, 81, 237, 37, 212, 72, 74, 163, 141, 251, 219, 244, 59, 197, 185
                ]),
                name: String::from("Return to Sender"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [5, 219, 47, 163, 185, 105, 224, 79, 134, 68, 245, 190, 108, 32, 64, 32],
            Overclock {
                guid: Guid([
                    5, 219, 47, 163, 185, 105, 224, 79, 134, 68, 245, 190, 108, 32, 64, 32
                ]),
                name: String::from("Roll Control"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [52, 10, 251, 198, 195, 6, 4, 66, 129, 1, 49, 129, 224, 2, 110, 27],
            Overclock {
                guid: Guid([52, 10, 251, 198, 195, 6, 4, 66, 129, 1, 49, 129, 224, 2, 110, 27]),
                name: String::from("Spinning Death"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [182, 61, 19, 118, 217, 13, 221, 76, 152, 104, 81, 215, 145, 231, 55, 171],
            Overclock {
                guid: Guid([
                    182, 61, 19, 118, 217, 13, 221, 76, 152, 104, 81, 215, 145, 231, 55, 171
                ]),
                name: String::from("Armor Break Module"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [83, 138, 82, 231, 116, 252, 178, 76, 133, 229, 247, 241, 206, 99, 227, 250],
            Overclock {
                guid: Guid([
                    83, 138, 82, 231, 116, 252, 178, 76, 133, 229, 247, 241, 206, 99, 227, 250
                ]),
                name: String::from("Eraser"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [136, 132, 237, 105, 176, 147, 228, 70, 129, 243, 108, 116, 253, 30, 119, 47],
            Overclock {
                guid: Guid([
                    136, 132, 237, 105, 176, 147, 228, 70, 129, 243, 108, 116, 253, 30, 119, 47
                ]),
                name: String::from("Executioner"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [111, 168, 62, 120, 53, 81, 245, 74, 151, 188, 4, 240, 157, 241, 168, 66],
            Overclock {
                guid: Guid([
                    111, 168, 62, 120, 53, 81, 245, 74, 151, 188, 4, 240, 157, 241, 168, 66
                ]),
                name: String::from("Explosive Chemical Rounds"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [121, 139, 149, 70, 171, 22, 179, 76, 173, 118, 109, 171, 85, 65, 80, 217],
            Overclock {
                guid: Guid([
                    121, 139, 149, 70, 171, 22, 179, 76, 173, 118, 109, 171, 85, 65, 80, 217
                ]),
                name: String::from("Seeker Rounds"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [149, 107, 181, 31, 77, 143, 37, 66, 145, 160, 192, 231, 64, 189, 7, 207],
            Overclock {
                guid: Guid([
                    149, 107, 181, 31, 77, 143, 37, 66, 145, 160, 192, 231, 64, 189, 7, 207
                ]),
                name: String::from("Neuro-Lasso"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [30, 65, 136, 12, 158, 24, 252, 67, 155, 130, 182, 159, 25, 240, 161, 145],
            Overclock {
                guid: Guid([
                    30, 65, 136, 12, 158, 24, 252, 67, 155, 130, 182, 159, 25, 240, 161, 145
                ]),
                name: String::from("Fragmentation Missiles"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [82, 222, 71, 181, 162, 87, 158, 65, 133, 100, 241, 147, 130, 36, 8, 102],
            Overclock {
                guid: Guid([
                    82, 222, 71, 181, 162, 87, 158, 65, 133, 100, 241, 147, 130, 36, 8, 102
                ]),
                name: String::from("Salvo Module"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [142, 104, 22, 195, 218, 238, 44, 73, 185, 62, 143, 166, 124, 241, 59, 26],
            Overclock {
                guid: Guid([
                    142, 104, 22, 195, 218, 238, 44, 73, 185, 62, 143, 166, 124, 241, 59, 26
                ]),
                name: String::from("Manual Guidance Cutoff"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [194, 70, 57, 251, 4, 160, 9, 73, 190, 143, 0, 205, 56, 70, 43, 132],
            Overclock {
                guid: Guid([194, 70, 57, 251, 4, 160, 9, 73, 190, 143, 0, 205, 56, 70, 43, 132]),
                name: String::from("Jet Fuel Homebrew"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [200, 115, 226, 154, 70, 62, 79, 77, 173, 72, 33, 217, 43, 195, 31, 180],
            Overclock {
                guid: Guid([
                    200, 115, 226, 154, 70, 62, 79, 77, 173, 72, 33, 217, 43, 195, 31, 180
                ]),
                name: String::from("Minelayer System"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [1, 205, 154, 173, 35, 152, 251, 70, 129, 138, 63, 111, 106, 150, 56, 74],
            Overclock {
                guid: Guid([
                    1, 205, 154, 173, 35, 152, 251, 70, 129, 138, 63, 111, 106, 150, 56, 74
                ]),
                name: String::from("Plasma Burster Missiles"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [42, 37, 118, 72, 223, 104, 136, 68, 186, 61, 176, 145, 163, 117, 107, 165],
            Overclock {
                guid: Guid([
                    42, 37, 118, 72, 223, 104, 136, 68, 186, 61, 176, 145, 163, 117, 107, 165
                ]),
                name: String::from("Overtuned Feed Mechanism"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [72, 100, 136, 50, 170, 17, 224, 77, 174, 159, 46, 138, 223, 137, 173, 141],
            Overclock {
                guid: Guid([
                    72, 100, 136, 50, 170, 17, 224, 77, 174, 159, 46, 138, 223, 137, 173, 141
                ]),
                name: String::from("Blistering Necrosis"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [12, 217, 225, 139, 133, 226, 113, 68, 153, 1, 211, 141, 80, 186, 98, 180],
            Overclock {
                guid: Guid([
                    12, 217, 225, 139, 133, 226, 113, 68, 153, 1, 211, 141, 80, 186, 98, 180
                ]),
                name: String::from("Diffusion Ray"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [95, 97, 64, 89, 179, 173, 161, 78, 147, 8, 49, 239, 75, 96, 152, 2],
            Overclock {
                guid: Guid([95, 97, 64, 89, 179, 173, 161, 78, 147, 8, 49, 239, 75, 96, 152, 2]),
                name: String::from("Liquid Cooling System"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [94, 221, 41, 135, 219, 4, 127, 73, 138, 221, 114, 252, 235, 108, 30, 227],
            Overclock {
                guid: Guid([
                    94, 221, 41, 135, 219, 4, 127, 73, 138, 221, 114, 252, 235, 108, 30, 227
                ]),
                name: String::from("Gamma Contamination"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [163, 232, 87, 144, 171, 203, 199, 74, 188, 203, 60, 198, 54, 80, 154, 27],
            Overclock {
                guid: Guid([
                    163, 232, 87, 144, 171, 203, 199, 74, 188, 203, 60, 198, 54, 80, 154, 27
                ]),
                name: String::from("Mega Power Supply"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [121, 168, 112, 237, 0, 201, 129, 74, 148, 43, 228, 173, 73, 161, 11, 99],
            Overclock {
                guid: Guid([
                    121, 168, 112, 237, 0, 201, 129, 74, 148, 43, 228, 173, 73, 161, 11, 99
                ]),
                name: String::from("Super Focus Lens"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [164, 171, 124, 98, 122, 247, 254, 79, 162, 53, 227, 170, 195, 152, 237, 31],
            Overclock {
                guid: Guid([
                    164, 171, 124, 98, 122, 247, 254, 79, 162, 53, 227, 170, 195, 152, 237, 31
                ]),
                name: String::from("Chain Hit"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [241, 62, 32, 196, 231, 73, 118, 67, 142, 133, 194, 55, 174, 7, 24, 48],
            Overclock {
                guid: Guid([241, 62, 32, 196, 231, 73, 118, 67, 142, 133, 194, 55, 174, 7, 24, 48]),
                name: String::from("Oversized Magazine"),
                dwarf: Driller,
                ty: Balanced,
                status: None
            }
        ),
        (
            [41, 73, 163, 190, 104, 234, 42, 65, 151, 213, 50, 175, 128, 84, 145, 1],
            Overclock {
                guid: Guid([
                    41, 73, 163, 190, 104, 234, 42, 65, 151, 213, 50, 175, 128, 84, 145, 1
                ]),
                name: String::from("Explosive Reload"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [40, 168, 232, 23, 229, 10, 115, 70, 150, 62, 65, 139, 160, 141, 254, 24],
            Overclock {
                guid: Guid([
                    40, 168, 232, 23, 229, 10, 115, 70, 150, 62, 65, 139, 160, 141, 254, 24
                ]),
                name: String::from("Automatic Fire"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [213, 123, 84, 47, 188, 121, 53, 66, 182, 187, 86, 23, 193, 166, 114, 41],
            Overclock {
                guid: Guid([
                    213, 123, 84, 47, 188, 121, 53, 66, 182, 187, 86, 23, 193, 166, 114, 41
                ]),
                name: String::from("Homebrew Powder"),
                dwarf: Driller,
                ty: Clean,
                status: None
            }
        ),
        (
            [88, 116, 136, 186, 119, 178, 141, 77, 161, 119, 64, 13, 153, 87, 55, 167],
            Overclock {
                guid: Guid([
                    88, 116, 136, 186, 119, 178, 141, 77, 161, 119, 64, 13, 153, 87, 55, 167
                ]),
                name: String::from("Tranquilizer Rounds"),
                dwarf: Driller,
                ty: Unstable,
                status: None
            }
        ),
        (
            [90, 191, 28, 72, 123, 243, 189, 75, 175, 218, 242, 164, 66, 135, 32, 236],
            Overclock {
                guid: Guid([
                    90, 191, 28, 72, 123, 243, 189, 75, 175, 218, 242, 164, 66, 135, 32, 236
                ]),
                name: String::from("Rewiring Mod"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [27, 215, 64, 89, 115, 158, 9, 75, 190, 10, 59, 121, 242, 233, 75, 70],
            Overclock {
                guid: Guid([27, 215, 64, 89, 115, 158, 9, 75, 190, 10, 59, 121, 242, 233, 75, 70]),
                name: String::from("Impact Deflection"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [0, 234, 9, 218, 20, 218, 118, 70, 152, 45, 213, 161, 178, 70, 122, 8],
            Overclock {
                guid: Guid([0, 234, 9, 218, 20, 218, 118, 70, 152, 45, 213, 161, 178, 70, 122, 8]),
                name: String::from("Thermal Liquid Coolant"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [191, 254, 12, 52, 54, 172, 88, 77, 137, 11, 110, 254, 145, 36, 170, 241],
            Overclock {
                guid: Guid([
                    191, 254, 12, 52, 54, 172, 88, 77, 137, 11, 110, 254, 145, 36, 170, 241
                ]),
                name: String::from("Thermal Exhaust Feedback"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [146, 37, 118, 151, 119, 53, 80, 65, 189, 235, 112, 202, 205, 148, 106, 39],
            Overclock {
                guid: Guid([
                    146, 37, 118, 151, 119, 53, 80, 65, 189, 235, 112, 202, 205, 148, 106, 39
                ]),
                name: String::from("Overtuned Particle Accelerator"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [216, 49, 176, 0, 242, 162, 200, 76, 135, 73, 165, 177, 130, 52, 133, 116],
            Overclock {
                guid: Guid([
                    216, 49, 176, 0, 242, 162, 200, 76, 135, 73, 165, 177, 130, 52, 133, 116
                ]),
                name: String::from("Aggressive Venting"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [83, 39, 20, 184, 3, 134, 124, 79, 138, 166, 234, 120, 250, 78, 170, 234],
            Overclock {
                guid: Guid([
                    83, 39, 20, 184, 3, 134, 124, 79, 138, 166, 234, 120, 250, 78, 170, 234
                ]),
                name: String::from("Shield Battery Booster"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [169, 55, 220, 57, 56, 222, 232, 65, 140, 208, 100, 28, 206, 25, 180, 106],
            Overclock {
                guid: Guid([
                    169, 55, 220, 57, 56, 222, 232, 65, 140, 208, 100, 28, 206, 25, 180, 106
                ]),
                name: String::from("Six Shooter"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [213, 63, 148, 251, 192, 94, 4, 72, 181, 123, 201, 223, 104, 70, 38, 123],
            Overclock {
                guid: Guid([
                    213, 63, 148, 251, 192, 94, 4, 72, 181, 123, 201, 223, 104, 70, 38, 123
                ]),
                name: String::from("Elephant Rounds"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [88, 133, 163, 59, 21, 174, 132, 69, 145, 166, 107, 101, 162, 229, 73, 78],
            Overclock {
                guid: Guid([
                    88, 133, 163, 59, 21, 174, 132, 69, 145, 166, 107, 101, 162, 229, 73, 78
                ]),
                name: String::from("Chain Hit"),
                dwarf: Gunner,
                ty: Clean,
                status: None
            }
        ),
        (
            [219, 2, 180, 231, 125, 67, 191, 68, 164, 116, 153, 130, 192, 135, 155, 119],
            Overclock {
                guid: Guid([
                    219, 2, 180, 231, 125, 67, 191, 68, 164, 116, 153, 130, 192, 135, 155, 119
                ]),
                name: String::from("Magic Bullets"),
                dwarf: Gunner,
                ty: Unstable,
                status: None
            }
        ),
        (
            [205, 105, 147, 249, 56, 226, 124, 73, 172, 105, 41, 59, 249, 66, 168, 240],
            Overclock {
                guid: Guid([
                    205, 105, 147, 249, 56, 226, 124, 73, 172, 105, 41, 59, 249, 66, 168, 240
                ]),
                name: String::from("Homebrew Powder"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [209, 48, 108, 188, 132, 33, 178, 72, 164, 185, 91, 51, 45, 243, 224, 86],
            Overclock {
                guid: Guid([
                    209, 48, 108, 188, 132, 33, 178, 72, 164, 185, 91, 51, 45, 243, 224, 86
                ]),
                name: String::from("Volatile Bullets"),
                dwarf: Gunner,
                ty: Balanced,
                status: None
            }
        ),
        (
            [239, 12, 172, 42, 239, 87, 189, 65, 167, 33, 85, 180, 218, 57, 93, 71],
            Overclock {
                guid: Guid([239, 12, 172, 42, 239, 87, 189, 65, 167, 33, 85, 180, 218, 57, 93, 71]),
                name: String::from("Shaped Shells"),
                dwarf: Scout,
                ty: Balanced,
                status: None
            }
        ),
        (
            [11, 26, 46, 198, 1, 104, 197, 78, 158, 71, 202, 118, 76, 59, 112, 189],
            Overclock {
                guid: Guid([11, 26, 46, 198, 1, 104, 197, 78, 158, 71, 202, 118, 76, 59, 112, 189]),
                name: String::from("Jumbo Shells"),
                dwarf: Scout,
                ty: Unstable,
                status: None
            }
        ),
        (
            [63, 104, 203, 124, 47, 171, 40, 67, 177, 68, 109, 19, 59, 217, 44, 29],
            Overclock {
                guid: Guid([63, 104, 203, 124, 47, 171, 40, 67, 177, 68, 109, 19, 59, 217, 44, 29]),
                name: String::from("Stuffed Shells"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [95, 9, 11, 179, 239, 201, 111, 66, 151, 241, 191, 195, 241, 82, 198, 99],
            Overclock {
                guid: Guid([
                    95, 9, 11, 179, 239, 201, 111, 66, 151, 241, 191, 195, 241, 82, 198, 99
                ]),
                name: String::from("Double Barrel"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [250, 64, 238, 74, 31, 140, 176, 79, 191, 175, 145, 100, 105, 228, 177, 103],
            Overclock {
                guid: Guid([
                    250, 64, 238, 74, 31, 140, 176, 79, 191, 175, 145, 100, 105, 228, 177, 103
                ]),
                name: String::from("Compact Shells"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [230, 218, 247, 214, 6, 80, 4, 67, 158, 249, 102, 214, 253, 189, 105, 166],
            Overclock {
                guid: Guid([
                    230, 218, 247, 214, 6, 80, 4, 67, 158, 249, 102, 214, 253, 189, 105, 166
                ]),
                name: String::from("Special Powder"),
                dwarf: Scout,
                ty: Clean,
                status: None
            }
        ),
        (
            [5, 177, 87, 160, 117, 224, 115, 75, 180, 113, 239, 141, 174, 105, 40, 101],
            Overclock {
                guid: Guid([
                    5, 177, 87, 160, 117, 224, 115, 75, 180, 113, 239, 141, 174, 105, 40, 101
                ]),
                name: String::from("Light-weight Rounds"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [206, 148, 144, 68, 91, 3, 107, 73, 184, 157, 178, 188, 141, 24, 253, 18],
            Overclock {
                guid: Guid([
                    206, 148, 144, 68, 91, 3, 107, 73, 184, 157, 178, 188, 141, 24, 253, 18
                ]),
                name: String::from("Well Oiled Machine"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [129, 184, 66, 49, 14, 1, 184, 71, 191, 153, 119, 77, 188, 61, 192, 221],
            Overclock {
                guid: Guid([
                    129, 184, 66, 49, 14, 1, 184, 71, 191, 153, 119, 77, 188, 61, 192, 221
                ]),
                name: String::from("EM Refire Booster"),
                dwarf: Engineer,
                ty: Balanced,
                status: None
            }
        ),
        (
            [19, 207, 28, 74, 105, 146, 146, 74, 154, 255, 92, 234, 240, 91, 113, 6],
            Overclock {
                guid: Guid([
                    19, 207, 28, 74, 105, 146, 146, 74, 154, 255, 92, 234, 240, 91, 113, 6
                ]),
                name: String::from("Super-Slim Rounds"),
                dwarf: Engineer,
                ty: Clean,
                status: None
            }
        ),
        (
            [12, 27, 170, 100, 176, 60, 185, 78, 187, 119, 183, 181, 94, 124, 108, 135],
            Overclock {
                guid: Guid([
                    12, 27, 170, 100, 176, 60, 185, 78, 187, 119, 183, 181, 94, 124, 108, 135
                ]),
                name: String::from("Turret EM Discharge"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
        (
            [60, 66, 219, 234, 48, 33, 238, 68, 175, 109, 148, 74, 145, 49, 77, 117],
            Overclock {
                guid: Guid([
                    60, 66, 219, 234, 48, 33, 238, 68, 175, 109, 148, 74, 145, 49, 77, 117
                ]),
                name: String::from("Turret Arc"),
                dwarf: Engineer,
                ty: Unstable,
                status: None
            }
        ),
    ]);
}
