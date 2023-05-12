use std::borrow::Cow;

use crate::save_file::dwarfs::Dwarf;
use gvas::types::Guid;

use CostInResource::*;
use Dwarf::*;
use OverclockType::*;

#[derive(Debug)]
pub enum CostInResource {
    Credits(i32),
    Croppa(f32),
    Umanite(f32),
    Bismor(f32),
    Jadiz(f32),
    Magnite(f32),
    EnorPearl(f32),
}

#[derive(Debug)]
pub enum OverclockType {
    Clean,
    Balanced,
    Unstable,
}

#[derive(Debug)]
pub struct Overclock<'a> {
    pub name: &'a str,
    pub cost: [CostInResource; 4],
    pub guid: Guid,
    pub ty: OverclockType,
    pub dwarf: Dwarf,
}

pub fn get_all_overclocks<'a>() -> Vec<Overclock<'a>> {
    vec![
        Overclock {
            name: "Compact Ammo",
            cost: [Credits(7250), Bismor(125.0), EnorPearl(80.0), Jadiz(105.0)],
            guid: Guid([
                175, 148, 91, 147, 167, 185, 214, 76, 166, 221, 0, 104, 54, 39, 188, 128,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Electrifying Reload",
            cost: [Credits(7750), Umanite(135.0), Magnite(65.0), Bismor(105.0)],
            guid: Guid([
                255, 148, 185, 231, 131, 77, 119, 66, 146, 218, 204, 202, 62, 160, 35, 185,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "AI Stability Engine",
            cost: [
                Credits(8250),
                EnorPearl(125.0),
                Croppa(60.0),
                Umanite(100.0),
            ],
            guid: Guid([
                17, 241, 35, 225, 116, 16, 49, 64, 149, 121, 87, 80, 107, 213, 118, 215,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Homebrew Powder",
            cost: [Credits(8100), Jadiz(140.0), Umanite(65.0), Bismor(95.0)],
            guid: Guid([
                76, 223, 65, 243, 160, 248, 228, 73, 157, 27, 172, 97, 104, 52, 119, 153,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Overclocked Firing Mechanism",
            cost: [Credits(7950), EnorPearl(120.0), Magnite(65.0), Bismor(95.0)],
            guid: Guid([
                195, 6, 3, 36, 206, 72, 44, 74, 182, 220, 68, 237, 73, 140, 186, 57,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Gas Rerouting",
            cost: [Credits(7800), Jadiz(125.0), Croppa(60.0), Magnite(105.0)],
            guid: Guid([
                246, 226, 229, 71, 242, 235, 103, 77, 189, 165, 145, 244, 125, 230, 208, 23,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Bullets of Mercy",
            cost: [Credits(8100), Magnite(125.0), Croppa(80.0), Bismor(90.0)],
            guid: Guid([
                115, 6, 34, 31, 18, 100, 176, 75, 140, 2, 93, 180, 222, 157, 22, 103,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Big Bertha",
            cost: [Credits(8400), Bismor(125.0), Umanite(80.0), Croppa(105.0)],
            guid: Guid([
                134, 170, 13, 209, 63, 211, 126, 67, 178, 251, 241, 118, 238, 93, 232, 21,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Carpet Bomber",
            cost: [Credits(7350), Croppa(120.0), Umanite(70.0), Magnite(105.0)],
            guid: Guid([
                140, 1, 66, 166, 135, 99, 122, 78, 164, 204, 138, 127, 132, 185, 97, 10,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Composite Drums",
            cost: [
                Credits(7850),
                Croppa(135.0),
                EnorPearl(70.0),
                Magnite(105.0),
            ],
            guid: Guid([
                1, 226, 48, 103, 203, 185, 166, 66, 138, 228, 57, 76, 153, 241, 210, 187,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Combat Mobility",
            cost: [Credits(7650), Jadiz(120.0), Croppa(70.0), Magnite(95.0)],
            guid: Guid([
                61, 213, 232, 20, 28, 72, 62, 67, 170, 62, 7, 8, 206, 117, 214, 99,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Neurotoxin Payload",
            cost: [Credits(8100), Magnite(135.0), Jadiz(75.0), Croppa(100.0)],
            guid: Guid([
                50, 101, 78, 148, 120, 22, 94, 77, 163, 240, 219, 211, 59, 24, 3, 65,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Splintering Shells",
            cost: [Credits(7300), Jadiz(125.0), Magnite(65.0), Croppa(95.0)],
            guid: Guid([
                203, 131, 253, 108, 19, 198, 63, 73, 130, 220, 135, 185, 121, 99, 17, 200,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Minimal Clips",
            cost: [Credits(8200), Jadiz(130.0), Magnite(75.0), EnorPearl(95.0)],
            guid: Guid([
                208, 6, 227, 238, 224, 84, 2, 66, 162, 176, 188, 168, 171, 205, 179, 135,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Supercooling Chamber",
            cost: [Credits(8500), Jadiz(130.0), Magnite(70.0), EnorPearl(90.0)],
            guid: Guid([
                16, 115, 206, 235, 236, 188, 1, 78, 131, 112, 226, 89, 4, 14, 183, 28,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Electrocuting Focus Shots",
            cost: [Credits(8850), Bismor(120.0), Umanite(75.0), Croppa(95.0)],
            guid: Guid([
                170, 201, 182, 228, 69, 132, 88, 71, 142, 80, 219, 135, 50, 64, 151, 235,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Hipster",
            cost: [
                Credits(8900),
                Croppa(125.0),
                Umanite(80.0),
                EnorPearl(105.0),
            ],
            guid: Guid([
                42, 18, 53, 143, 117, 11, 112, 73, 150, 195, 125, 70, 126, 207, 29, 54,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Hoverclock",
            cost: [Credits(7350), Croppa(135.0), Jadiz(65.0), Bismor(105.0)],
            guid: Guid([
                138, 1, 34, 228, 78, 66, 16, 75, 170, 173, 107, 69, 52, 169, 222, 143,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Active Stability System",
            cost: [Credits(8150), Umanite(135.0), Magnite(70.0), Bismor(90.0)],
            guid: Guid([
                209, 84, 75, 53, 40, 147, 138, 72, 150, 78, 141, 154, 99, 9, 209, 218,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Composite Casings",
            cost: [
                Credits(7950),
                Croppa(140.0),
                EnorPearl(75.0),
                Magnite(100.0),
            ],
            guid: Guid([
                111, 38, 168, 180, 159, 150, 124, 77, 153, 159, 115, 70, 69, 234, 226, 196,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Compact Mags",
            cost: [Credits(7350), Magnite(135.0), Jadiz(75.0), Umanite(105.0)],
            guid: Guid([
                47, 243, 212, 201, 45, 89, 48, 69, 146, 0, 159, 162, 227, 155, 255, 198,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Electro Minelets",
            cost: [Credits(7450), Umanite(120.0), EnorPearl(80.0), Jadiz(95.0)],
            guid: Guid([
                74, 79, 147, 254, 26, 195, 30, 65, 184, 151, 193, 241, 178, 100, 166, 114,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Full Chamber Seal",
            cost: [Credits(7850), Bismor(120.0), Jadiz(75.0), Magnite(110.0)],
            guid: Guid([
                65, 73, 22, 149, 122, 211, 208, 64, 155, 95, 227, 105, 42, 70, 221, 25,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Experimental Rounds",
            cost: [Credits(8550), Magnite(130.0), Jadiz(75.0), EnorPearl(100.0)],
            guid: Guid([
                136, 171, 212, 10, 120, 50, 153, 66, 136, 76, 0, 215, 242, 45, 172, 144,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Micro Flechettes",
            cost: [Credits(7650), Magnite(130.0), Bismor(80.0), Jadiz(100.0)],
            guid: Guid([
                143, 28, 106, 26, 35, 166, 223, 76, 188, 165, 81, 12, 74, 25, 242, 33,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Lead Spray",
            cost: [Credits(8050), Bismor(125.0), Magnite(75.0), Umanite(105.0)],
            guid: Guid([
                241, 174, 85, 207, 89, 46, 119, 70, 160, 0, 29, 97, 88, 75, 223, 42,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Heat Pipe",
            cost: [Credits(7450), Umanite(125.0), Bismor(60.0), Jadiz(95.0)],
            guid: Guid([
                160, 224, 24, 196, 95, 230, 121, 65, 143, 211, 115, 14, 110, 155, 99, 29,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Overcharger",
            cost: [Credits(7050), Bismor(120.0), EnorPearl(60.0), Croppa(95.0)],
            guid: Guid([
                85, 187, 82, 160, 236, 250, 124, 67, 179, 123, 187, 230, 64, 17, 65, 150,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Energy Rerouting",
            cost: [Credits(7300), Bismor(130.0), Umanite(65.0), Jadiz(100.0)],
            guid: Guid([
                151, 130, 32, 113, 213, 134, 206, 69, 172, 214, 72, 151, 130, 183, 0, 250,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Magnetic Cooling Unit",
            cost: [Credits(8900), Umanite(125.0), Jadiz(80.0), Croppa(95.0)],
            guid: Guid([
                241, 230, 204, 48, 124, 17, 222, 69, 134, 184, 139, 65, 158, 83, 10, 191,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Heavy Hitter",
            cost: [Credits(8100), Bismor(140.0), Magnite(60.0), Umanite(105.0)],
            guid: Guid([
                57, 228, 62, 19, 204, 13, 174, 70, 145, 221, 81, 161, 19, 131, 233, 225,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Persistent Plasma",
            cost: [Credits(8150), Jadiz(130.0), Croppa(75.0), Magnite(95.0)],
            guid: Guid([
                22, 92, 217, 62, 102, 11, 129, 71, 188, 162, 248, 108, 222, 92, 248, 247,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Hellfire",
            cost: [Credits(8700), Croppa(125.0), EnorPearl(80.0), Jadiz(100.0)],
            guid: Guid([
                128, 222, 169, 10, 213, 243, 23, 70, 149, 19, 143, 244, 122, 50, 169, 106,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Backfeeding Module",
            cost: [Credits(8100), Bismor(125.0), Magnite(70.0), Jadiz(105.0)],
            guid: Guid([
                248, 47, 50, 101, 118, 232, 167, 72, 153, 117, 60, 182, 49, 107, 124, 32,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "The Mole",
            cost: [Credits(8100), Jadiz(125.0), Magnite(70.0), EnorPearl(95.0)],
            guid: Guid([
                86, 71, 31, 33, 155, 224, 20, 68, 141, 254, 95, 8, 124, 171, 212, 71,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Re-atomizer",
            cost: [Credits(7750), Magnite(135.0), Jadiz(65.0), Bismor(90.0)],
            guid: Guid([
                140, 93, 36, 189, 104, 213, 241, 74, 137, 166, 184, 122, 133, 124, 205, 155,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Ultra-magnetic Coils",
            cost: [Credits(7900), EnorPearl(125.0), Jadiz(70.0), Magnite(110.0)],
            guid: Guid([
                38, 132, 30, 181, 130, 58, 40, 75, 129, 191, 125, 163, 121, 39, 8, 215,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Triple-Tech Chambers",
            cost: [Credits(8500), Croppa(125.0), Bismor(70.0), Magnite(95.0)],
            guid: Guid([
                166, 117, 29, 161, 184, 124, 19, 78, 188, 87, 16, 199, 67, 28, 20, 83,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Light-weight Magazines",
            cost: [Credits(7250), Croppa(125.0), Umanite(60.0), Magnite(105.0)],
            guid: Guid([
                205, 78, 236, 56, 192, 41, 114, 78, 131, 23, 204, 220, 34, 208, 96, 87,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Stunner",
            cost: [Credits(7350), Jadiz(135.0), Magnite(60.0), Bismor(100.0)],
            guid: Guid([
                51, 141, 210, 56, 51, 42, 46, 78, 165, 73, 2, 8, 170, 215, 245, 2,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Mini Shells",
            cost: [Credits(7700), Croppa(125.0), EnorPearl(65.0), Magnite(90.0)],
            guid: Guid([
                98, 212, 201, 76, 19, 103, 232, 65, 150, 145, 182, 119, 131, 125, 164, 251,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Cycle Overload",
            cost: [Credits(8050), Bismor(125.0), Umanite(80.0), Croppa(100.0)],
            guid: Guid([
                112, 18, 43, 162, 251, 218, 191, 72, 149, 220, 9, 201, 2, 148, 230, 133,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Magnetic Pellet Alignment",
            cost: [Credits(7900), EnorPearl(120.0), Umanite(75.0), Jadiz(105.0)],
            guid: Guid([
                7, 233, 39, 153, 86, 191, 156, 65, 142, 234, 49, 62, 45, 197, 159, 141,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Bodkin Points",
            cost: [Credits(8400), Magnite(140.0), Jadiz(80.0), Croppa(110.0)],
            guid: Guid([
                174, 203, 15, 36, 29, 250, 25, 74, 176, 75, 187, 134, 86, 63, 168, 64,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Cryo Bolt",
            cost: [Credits(7350), Magnite(135.0), Croppa(75.0), Jadiz(105.0)],
            guid: Guid([
                117, 207, 208, 169, 15, 49, 201, 74, 157, 106, 141, 2, 20, 125, 225, 252,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Fire Bolt",
            cost: [Credits(7100), Umanite(130.0), Magnite(70.0), Croppa(90.0)],
            guid: Guid([
                221, 202, 129, 226, 156, 107, 232, 72, 140, 45, 244, 240, 176, 199, 42, 114,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Quick Fire",
            cost: [Credits(8400), Umanite(125.0), EnorPearl(70.0), Croppa(95.0)],
            guid: Guid([
                28, 250, 87, 250, 53, 208, 219, 73, 169, 170, 248, 124, 170, 1, 30, 244,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "The Specialist",
            cost: [Credits(7900), Magnite(130.0), Croppa(65.0), Umanite(90.0)],
            guid: Guid([
                108, 70, 69, 49, 130, 14, 168, 73, 187, 60, 128, 192, 241, 144, 167, 79,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Trifork Volley",
            cost: [Credits(7650), Bismor(130.0), Magnite(65.0), Jadiz(100.0)],
            guid: Guid([
                98, 254, 150, 222, 50, 165, 69, 78, 176, 241, 149, 43, 214, 141, 186, 244,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Improved Thermal Efficiency",
            cost: [
                Credits(8350),
                Croppa(125.0),
                EnorPearl(70.0),
                Magnite(110.0),
            ],
            guid: Guid([
                206, 102, 12, 77, 217, 152, 0, 67, 168, 2, 161, 122, 69, 105, 192, 172,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Tuned Cooler",
            cost: [Credits(8750), Umanite(130.0), Magnite(65.0), Bismor(110.0)],
            guid: Guid([
                106, 219, 35, 179, 200, 237, 136, 64, 134, 168, 139, 129, 7, 201, 60, 190,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Flow Rate Expansion",
            cost: [Credits(8900), Magnite(125.0), EnorPearl(70.0), Jadiz(100.0)],
            guid: Guid([
                136, 248, 234, 178, 6, 236, 123, 70, 163, 28, 143, 3, 75, 68, 147, 26,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Ice Storm",
            cost: [
                Credits(7200),
                EnorPearl(130.0),
                Umanite(75.0),
                Magnite(105.0),
            ],
            guid: Guid([
                123, 17, 117, 89, 151, 182, 86, 78, 151, 177, 3, 42, 130, 142, 185, 15,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Ice Spear",
            cost: [Credits(8950), Jadiz(130.0), EnorPearl(60.0), Umanite(110.0)],
            guid: Guid([
                213, 24, 203, 55, 26, 50, 160, 68, 155, 52, 94, 79, 88, 211, 76, 145,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Snowball",
            cost: [Credits(8400), Umanite(130.0), Magnite(70.0), Jadiz(90.0)],
            guid: Guid([
                193, 241, 111, 151, 12, 156, 16, 71, 175, 131, 107, 246, 203, 254, 188, 173,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Cryo Minelets",
            cost: [Credits(7300), Magnite(135.0), Croppa(65.0), Umanite(95.0)],
            guid: Guid([
                67, 205, 141, 39, 234, 234, 170, 74, 142, 109, 196, 62, 4, 56, 161, 12,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Custom Casings",
            cost: [Credits(7700), EnorPearl(140.0), Croppa(75.0), Bismor(95.0)],
            guid: Guid([
                36, 162, 134, 195, 31, 69, 234, 70, 154, 32, 16, 42, 244, 215, 65, 211,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Gas Recycling",
            cost: [Credits(7800), Magnite(125.0), EnorPearl(70.0), Jadiz(105.0)],
            guid: Guid([
                130, 139, 43, 188, 80, 236, 66, 67, 133, 239, 208, 14, 166, 133, 69, 151,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Embedded Detonators",
            cost: [Credits(7550), Jadiz(135.0), Magnite(65.0), Umanite(90.0)],
            guid: Guid([
                250, 243, 80, 113, 186, 99, 178, 66, 152, 85, 251, 200, 160, 155, 255, 208,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Minimal Magazines",
            cost: [Credits(8450), Bismor(130.0), Jadiz(70.0), Croppa(100.0)],
            guid: Guid([
                152, 160, 69, 43, 35, 84, 2, 69, 156, 255, 92, 195, 179, 76, 204, 105,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Compact Feed Valves",
            cost: [Credits(7350), Umanite(130.0), Bismor(70.0), Jadiz(90.0)],
            guid: Guid([
                113, 88, 141, 67, 167, 88, 154, 74, 190, 97, 168, 222, 46, 180, 73, 191,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Face Melter",
            cost: [Credits(7000), Croppa(130.0), EnorPearl(70.0), Magnite(90.0)],
            guid: Guid([
                100, 49, 168, 221, 234, 166, 137, 75, 144, 197, 123, 172, 117, 82, 165, 255,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Lighter Tanks",
            cost: [Credits(7500), Bismor(125.0), Croppa(75.0), Umanite(90.0)],
            guid: Guid([
                197, 5, 122, 150, 8, 207, 94, 75, 164, 170, 179, 42, 214, 19, 186, 62,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Fuel Stream Diffuser",
            cost: [Credits(7100), Jadiz(125.0), EnorPearl(80.0), Bismor(100.0)],
            guid: Guid([
                25, 232, 93, 28, 53, 133, 179, 73, 177, 81, 119, 156, 189, 115, 3, 217,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Sticky Fuel",
            cost: [Credits(8800), Jadiz(140.0), Magnite(75.0), EnorPearl(110.0)],
            guid: Guid([
                242, 140, 62, 227, 138, 150, 13, 72, 150, 94, 159, 105, 245, 107, 63, 25,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Sticky Additive",
            cost: [Credits(8250), Jadiz(130.0), Bismor(80.0), Magnite(100.0)],
            guid: Guid([
                65, 95, 29, 131, 167, 126, 122, 68, 141, 191, 96, 56, 49, 3, 253, 108,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Compact Feed Mechanism",
            cost: [Credits(7450), Magnite(130.0), Bismor(70.0), Croppa(95.0)],
            guid: Guid([
                46, 159, 191, 180, 59, 123, 85, 79, 179, 114, 135, 128, 241, 51, 72, 36,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Thinned Drum Walls",
            cost: [Credits(7650), EnorPearl(125.0), Croppa(75.0), Jadiz(95.0)],
            guid: Guid([
                1, 212, 101, 57, 3, 131, 47, 73, 140, 80, 247, 113, 157, 88, 232, 155,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Lead Storm",
            cost: [Credits(8800), EnorPearl(130.0), Magnite(65.0), Jadiz(100.0)],
            guid: Guid([
                124, 209, 14, 44, 85, 88, 48, 65, 181, 119, 20, 28, 133, 75, 238, 98,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Exhaust Vectoring",
            cost: [Credits(7400), Bismor(140.0), Magnite(65.0), Croppa(95.0)],
            guid: Guid([
                251, 177, 1, 71, 88, 96, 106, 65, 136, 142, 55, 143, 105, 227, 205, 97,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "A little more oomph!",
            cost: [Credits(8700), Bismor(120.0), Umanite(75.0), Magnite(95.0)],
            guid: Guid([
                181, 93, 97, 112, 238, 165, 215, 67, 172, 75, 40, 41, 60, 12, 70, 115,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Burning Hell",
            cost: [Credits(8700), Magnite(140.0), Umanite(65.0), Croppa(110.0)],
            guid: Guid([
                99, 21, 102, 115, 242, 23, 79, 68, 159, 89, 143, 135, 41, 172, 12, 101,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Bullet Hell",
            cost: [
                Credits(7600),
                Magnite(140.0),
                Umanite(75.0),
                EnorPearl(105.0),
            ],
            guid: Guid([
                237, 64, 54, 128, 106, 105, 232, 72, 188, 114, 221, 227, 205, 234, 196, 86,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Disperser Compound",
            cost: [Credits(8800), Jadiz(135.0), Umanite(60.0), Magnite(100.0)],
            guid: Guid([
                193, 215, 122, 139, 175, 212, 51, 64, 142, 74, 131, 151, 241, 166, 58, 236,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Goo Bomber Special",
            cost: [Credits(7450), Jadiz(140.0), Umanite(65.0), EnorPearl(90.0)],
            guid: Guid([
                76, 204, 126, 153, 65, 99, 161, 74, 170, 201, 122, 179, 119, 127, 241, 249,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "AG Mixture",
            cost: [Credits(7800), Umanite(140.0), Croppa(60.0), Jadiz(105.0)],
            guid: Guid([
                73, 86, 103, 43, 166, 141, 58, 67, 157, 78, 194, 96, 240, 185, 140, 228,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Hydrogen Ion Additive",
            cost: [
                Credits(8800),
                Bismor(135.0),
                Umanite(70.0),
                EnorPearl(100.0),
            ],
            guid: Guid([
                236, 202, 178, 85, 20, 5, 240, 68, 169, 63, 251, 108, 81, 255, 155, 57,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Volatile Impact Mixture",
            cost: [Credits(8500), Bismor(120.0), Jadiz(75.0), Magnite(100.0)],
            guid: Guid([
                83, 159, 22, 208, 236, 165, 4, 71, 178, 27, 211, 177, 173, 212, 57, 203,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Sludge Blast",
            cost: [Credits(8200), Magnite(120.0), Umanite(60.0), Jadiz(100.0)],
            guid: Guid([
                234, 125, 39, 59, 86, 9, 155, 75, 159, 68, 118, 138, 9, 154, 140, 28,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Pack Rat",
            cost: [
                Credits(7950),
                Magnite(120.0),
                Bismor(80.0),
                EnorPearl(105.0),
            ],
            guid: Guid([
                169, 118, 148, 225, 169, 183, 72, 78, 182, 110, 20, 219, 248, 48, 219, 107,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Compact Rounds",
            cost: [
                Credits(7900),
                Bismor(120.0),
                Umanite(70.0),
                EnorPearl(100.0),
            ],
            guid: Guid([
                238, 87, 19, 250, 145, 160, 226, 69, 161, 192, 191, 206, 21, 33, 32, 116,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Fat Boy",
            cost: [
                Credits(8300),
                Bismor(120.0),
                EnorPearl(70.0),
                Magnite(105.0),
            ],
            guid: Guid([
                135, 210, 67, 67, 136, 211, 252, 69, 136, 78, 53, 29, 203, 231, 15, 124,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Clean Sweep",
            cost: [
                Credits(8100),
                Umanite(135.0),
                EnorPearl(70.0),
                Bismor(105.0),
            ],
            guid: Guid([
                185, 18, 5, 154, 63, 134, 228, 75, 162, 152, 84, 80, 209, 54, 204, 181,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "RJ250 Compound",
            cost: [
                Credits(8800),
                Umanite(120.0),
                Bismor(65.0),
                EnorPearl(110.0),
            ],
            guid: Guid([
                219, 126, 199, 136, 180, 111, 206, 76, 152, 0, 162, 80, 16, 160, 232, 234,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Hyper Propellant",
            cost: [Credits(8950), Magnite(130.0), Jadiz(70.0), Croppa(90.0)],
            guid: Guid([
                188, 76, 214, 132, 98, 27, 246, 68, 150, 7, 55, 58, 83, 74, 87, 79,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Feedback Loop",
            cost: [
                Credits(7200),
                EnorPearl(125.0),
                Magnite(65.0),
                Umanite(100.0),
            ],
            guid: Guid([
                241, 136, 12, 111, 123, 239, 16, 71, 142, 119, 238, 79, 198, 52, 129, 246,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Automated Beam Controller",
            cost: [Credits(8050), Jadiz(130.0), Magnite(65.0), Bismor(90.0)],
            guid: Guid([
                113, 216, 27, 195, 219, 194, 143, 77, 144, 90, 108, 159, 74, 139, 85, 174,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Plastcrete Catalyst",
            cost: [Credits(7950), Jadiz(125.0), Magnite(75.0), Croppa(110.0)],
            guid: Guid([
                211, 48, 114, 57, 105, 188, 228, 76, 177, 235, 233, 0, 154, 47, 110, 13,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Efficiency Tweaks",
            cost: [Credits(7650), Croppa(120.0), Magnite(70.0), Jadiz(90.0)],
            guid: Guid([
                241, 67, 47, 18, 31, 104, 224, 77, 160, 197, 221, 142, 80, 207, 113, 125,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Overdrive Booster",
            cost: [Credits(7900), Bismor(120.0), Jadiz(75.0), Croppa(110.0)],
            guid: Guid([
                24, 122, 24, 186, 82, 131, 210, 76, 144, 90, 55, 244, 251, 18, 59, 160,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Volatile Impact Reactor",
            cost: [
                Credits(8300),
                Magnite(125.0),
                Bismor(65.0),
                EnorPearl(105.0),
            ],
            guid: Guid([
                123, 183, 194, 31, 161, 209, 230, 74, 140, 0, 124, 230, 160, 35, 164, 18,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Light-weight Cases",
            cost: [Credits(8700), Bismor(130.0), Jadiz(80.0), Croppa(100.0)],
            guid: Guid([
                150, 148, 12, 124, 220, 239, 168, 64, 132, 221, 77, 96, 210, 232, 27, 45,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Stronger Plasma Current",
            cost: [Credits(8650), Jadiz(140.0), Magnite(75.0), Croppa(100.0)],
            guid: Guid([
                61, 124, 156, 133, 246, 211, 165, 73, 162, 214, 237, 176, 153, 24, 49, 75,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "High Voltage Crossover",
            cost: [
                Credits(8250),
                Bismor(120.0),
                EnorPearl(80.0),
                Magnite(100.0),
            ],
            guid: Guid([
                127, 161, 132, 203, 157, 33, 135, 77, 130, 191, 157, 218, 252, 124, 145, 215,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Inferno",
            cost: [Credits(7550), Magnite(135.0), Croppa(70.0), Jadiz(90.0)],
            guid: Guid([
                142, 217, 18, 248, 38, 123, 132, 68, 130, 238, 218, 134, 96, 68, 81, 165,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Return to Sender",
            cost: [
                Credits(7950),
                Bismor(140.0),
                Umanite(80.0),
                EnorPearl(100.0),
            ],
            guid: Guid([
                116, 66, 81, 237, 37, 212, 72, 74, 163, 141, 251, 219, 244, 59, 197, 185,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Roll Control",
            cost: [Credits(8150), Umanite(135.0), Magnite(80.0), Croppa(95.0)],
            guid: Guid([
                5, 219, 47, 163, 185, 105, 224, 79, 134, 68, 245, 190, 108, 32, 64, 32,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Spinning Death",
            cost: [Credits(7300), Umanite(120.0), Bismor(75.0), Magnite(95.0)],
            guid: Guid([
                52, 10, 251, 198, 195, 6, 4, 66, 129, 1, 49, 129, 224, 2, 110, 27,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Armor Break Module",
            cost: [Credits(7850), Croppa(125.0), Umanite(80.0), Magnite(90.0)],
            guid: Guid([
                182, 61, 19, 118, 217, 13, 221, 76, 152, 104, 81, 215, 145, 231, 55, 171,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Eraser",
            cost: [Credits(8000), Croppa(135.0), Magnite(70.0), EnorPearl(90.0)],
            guid: Guid([
                83, 138, 82, 231, 116, 252, 178, 76, 133, 229, 247, 241, 206, 99, 227, 250,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Executioner",
            cost: [Credits(8750), EnorPearl(130.0), Magnite(65.0), Jadiz(105.0)],
            guid: Guid([
                136, 132, 237, 105, 176, 147, 228, 70, 129, 243, 108, 116, 253, 30, 119, 47,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Explosive Chemical Rounds",
            cost: [Credits(8500), Jadiz(130.0), Croppa(65.0), Magnite(95.0)],
            guid: Guid([
                111, 168, 62, 120, 53, 81, 245, 74, 151, 188, 4, 240, 157, 241, 168, 66,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Seeker Rounds",
            cost: [Credits(8150), Magnite(135.0), Bismor(60.0), Umanite(95.0)],
            guid: Guid([
                121, 139, 149, 70, 171, 22, 179, 76, 173, 118, 109, 171, 85, 65, 80, 217,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Neuro-Lasso",
            cost: [Credits(8950), Croppa(125.0), Magnite(80.0), Umanite(100.0)],
            guid: Guid([
                149, 107, 181, 31, 77, 143, 37, 66, 145, 160, 192, 231, 64, 189, 7, 207,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Fragmentation Missiles",
            cost: [Credits(7150), Croppa(135.0), Jadiz(60.0), Bismor(100.0)],
            guid: Guid([
                30, 65, 136, 12, 158, 24, 252, 67, 155, 130, 182, 159, 25, 240, 161, 145,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Salvo Module",
            cost: [Credits(8650), Umanite(130.0), EnorPearl(65.0), Jadiz(110.0)],
            guid: Guid([
                82, 222, 71, 181, 162, 87, 158, 65, 133, 100, 241, 147, 130, 36, 8, 102,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Manual Guidance Cutoff",
            cost: [Credits(7050), Umanite(130.0), Bismor(75.0), Jadiz(105.0)],
            guid: Guid([
                142, 104, 22, 195, 218, 238, 44, 73, 185, 62, 143, 166, 124, 241, 59, 26,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Jet Fuel Homebrew",
            cost: [Credits(7250), Jadiz(130.0), Bismor(70.0), Umanite(100.0)],
            guid: Guid([
                194, 70, 57, 251, 4, 160, 9, 73, 190, 143, 0, 205, 56, 70, 43, 132,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Minelayer System",
            cost: [Credits(7500), Croppa(125.0), Bismor(80.0), EnorPearl(90.0)],
            guid: Guid([
                200, 115, 226, 154, 70, 62, 79, 77, 173, 72, 33, 217, 43, 195, 31, 180,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Plasma Burster Missiles",
            cost: [
                Credits(8500),
                Umanite(140.0),
                EnorPearl(80.0),
                Croppa(105.0),
            ],
            guid: Guid([
                1, 205, 154, 173, 35, 152, 251, 70, 129, 138, 63, 111, 106, 150, 56, 74,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Overtuned Feed Mechanism",
            cost: [Credits(8950), Magnite(130.0), Croppa(65.0), Jadiz(105.0)],
            guid: Guid([
                42, 37, 118, 72, 223, 104, 136, 68, 186, 61, 176, 145, 163, 117, 107, 165,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Blistering Necrosis",
            cost: [Credits(7850), Croppa(130.0), Bismor(70.0), Magnite(100.0)],
            guid: Guid([
                72, 100, 136, 50, 170, 17, 224, 77, 174, 159, 46, 138, 223, 137, 173, 141,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Diffusion Ray",
            cost: [Credits(8300), Magnite(130.0), Umanite(65.0), Croppa(105.0)],
            guid: Guid([
                12, 217, 225, 139, 133, 226, 113, 68, 153, 1, 211, 141, 80, 186, 98, 180,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Liquid Cooling System",
            cost: [Credits(7400), Bismor(125.0), Jadiz(65.0), Umanite(95.0)],
            guid: Guid([
                95, 97, 64, 89, 179, 173, 161, 78, 147, 8, 49, 239, 75, 96, 152, 2,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Gamma Contamination",
            cost: [
                Credits(8350),
                EnorPearl(130.0),
                Magnite(80.0),
                Bismor(100.0),
            ],
            guid: Guid([
                94, 221, 41, 135, 219, 4, 127, 73, 138, 221, 114, 252, 235, 108, 30, 227,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Mega Power Supply",
            cost: [Credits(8100), Umanite(140.0), Magnite(65.0), Croppa(95.0)],
            guid: Guid([
                163, 232, 87, 144, 171, 203, 199, 74, 188, 203, 60, 198, 54, 80, 154, 27,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Super Focus Lens",
            cost: [Credits(7550), Bismor(130.0), Croppa(80.0), Jadiz(105.0)],
            guid: Guid([
                121, 168, 112, 237, 0, 201, 129, 74, 148, 43, 228, 173, 73, 161, 11, 99,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Chain Hit",
            cost: [Credits(7600), Croppa(120.0), Bismor(65.0), Jadiz(100.0)],
            guid: Guid([
                164, 171, 124, 98, 122, 247, 254, 79, 162, 53, 227, 170, 195, 152, 237, 31,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Oversized Magazine",
            cost: [Credits(9000), Umanite(130.0), Croppa(70.0), Jadiz(110.0)],
            guid: Guid([
                241, 62, 32, 196, 231, 73, 118, 67, 142, 133, 194, 55, 174, 7, 24, 48,
            ]),
            ty: Balanced,
            dwarf: Driller,
        },
        Overclock {
            name: "Explosive Reload",
            cost: [
                Credits(8100),
                Umanite(125.0),
                Magnite(65.0),
                EnorPearl(95.0),
            ],
            guid: Guid([
                41, 73, 163, 190, 104, 234, 42, 65, 151, 213, 50, 175, 128, 84, 145, 1,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Automatic Fire",
            cost: [Credits(7400), EnorPearl(120.0), Croppa(65.0), Bismor(95.0)],
            guid: Guid([
                40, 168, 232, 23, 229, 10, 115, 70, 150, 62, 65, 139, 160, 141, 254, 24,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Homebrew Powder",
            cost: [Credits(7150), Bismor(135.0), Magnite(70.0), Croppa(100.0)],
            guid: Guid([
                213, 123, 84, 47, 188, 121, 53, 66, 182, 187, 86, 23, 193, 166, 114, 41,
            ]),
            ty: Clean,
            dwarf: Driller,
        },
        Overclock {
            name: "Tranquilizer Rounds",
            cost: [Credits(7150), Jadiz(135.0), Umanite(75.0), Croppa(95.0)],
            guid: Guid([
                88, 116, 136, 186, 119, 178, 141, 77, 161, 119, 64, 13, 153, 87, 55, 167,
            ]),
            ty: Unstable,
            dwarf: Driller,
        },
        Overclock {
            name: "Rewiring Mod",
            cost: [Credits(8200), Bismor(130.0), Croppa(65.0), Jadiz(110.0)],
            guid: Guid([
                90, 191, 28, 72, 123, 243, 189, 75, 175, 218, 242, 164, 66, 135, 32, 236,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Impact Deflection",
            cost: [Credits(7950), Jadiz(140.0), Croppa(60.0), Umanite(105.0)],
            guid: Guid([
                27, 215, 64, 89, 115, 158, 9, 75, 190, 10, 59, 121, 242, 233, 75, 70,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Thermal Liquid Coolant",
            cost: [
                Credits(8100),
                Umanite(130.0),
                Bismor(75.0),
                EnorPearl(105.0),
            ],
            guid: Guid([
                0, 234, 9, 218, 20, 218, 118, 70, 152, 45, 213, 161, 178, 70, 122, 8,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Thermal Exhaust Feedback",
            cost: [Credits(7500), Bismor(140.0), Umanite(75.0), Jadiz(100.0)],
            guid: Guid([
                191, 254, 12, 52, 54, 172, 88, 77, 137, 11, 110, 254, 145, 36, 170, 241,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Overtuned Particle Accelerator",
            cost: [Credits(7200), Magnite(120.0), Jadiz(65.0), Bismor(100.0)],
            guid: Guid([
                146, 37, 118, 151, 119, 53, 80, 65, 189, 235, 112, 202, 205, 148, 106, 39,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Aggressive Venting",
            cost: [Credits(8000), Bismor(130.0), EnorPearl(80.0), Croppa(95.0)],
            guid: Guid([
                216, 49, 176, 0, 242, 162, 200, 76, 135, 73, 165, 177, 130, 52, 133, 116,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Shield Battery Booster",
            cost: [Credits(8850), Jadiz(120.0), Bismor(65.0), Magnite(105.0)],
            guid: Guid([
                83, 39, 20, 184, 3, 134, 124, 79, 138, 166, 234, 120, 250, 78, 170, 234,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Six Shooter",
            cost: [Credits(7750), Bismor(120.0), Croppa(60.0), Magnite(100.0)],
            guid: Guid([
                169, 55, 220, 57, 56, 222, 232, 65, 140, 208, 100, 28, 206, 25, 180, 106,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Elephant Rounds",
            cost: [
                Credits(7300),
                EnorPearl(140.0),
                Umanite(65.0),
                Magnite(90.0),
            ],
            guid: Guid([
                213, 63, 148, 251, 192, 94, 4, 72, 181, 123, 201, 223, 104, 70, 38, 123,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Chain Hit",
            cost: [Credits(7300), Magnite(120.0), EnorPearl(80.0), Jadiz(110.0)],
            guid: Guid([
                88, 133, 163, 59, 21, 174, 132, 69, 145, 166, 107, 101, 162, 229, 73, 78,
            ]),
            ty: Clean,
            dwarf: Gunner,
        },
        Overclock {
            name: "Magic Bullets",
            cost: [Credits(8750), Magnite(130.0), Umanite(75.0), Croppa(105.0)],
            guid: Guid([
                219, 2, 180, 231, 125, 67, 191, 68, 164, 116, 153, 130, 192, 135, 155, 119,
            ]),
            ty: Unstable,
            dwarf: Gunner,
        },
        Overclock {
            name: "Homebrew Powder",
            cost: [
                Credits(7350),
                Croppa(135.0),
                Magnite(70.0),
                EnorPearl(105.0),
            ],
            guid: Guid([
                205, 105, 147, 249, 56, 226, 124, 73, 172, 105, 41, 59, 249, 66, 168, 240,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Volatile Bullets",
            cost: [Credits(7350), Croppa(130.0), Magnite(60.0), Jadiz(110.0)],
            guid: Guid([
                209, 48, 108, 188, 132, 33, 178, 72, 164, 185, 91, 51, 45, 243, 224, 86,
            ]),
            ty: Balanced,
            dwarf: Gunner,
        },
        Overclock {
            name: "Shaped Shells",
            cost: [Credits(7700), Jadiz(135.0), EnorPearl(70.0), Bismor(95.0)],
            guid: Guid([
                239, 12, 172, 42, 239, 87, 189, 65, 167, 33, 85, 180, 218, 57, 93, 71,
            ]),
            ty: Balanced,
            dwarf: Scout,
        },
        Overclock {
            name: "Jumbo Shells",
            cost: [Credits(8800), Jadiz(125.0), Bismor(65.0), EnorPearl(105.0)],
            guid: Guid([
                11, 26, 46, 198, 1, 104, 197, 78, 158, 71, 202, 118, 76, 59, 112, 189,
            ]),
            ty: Unstable,
            dwarf: Scout,
        },
        Overclock {
            name: "Stuffed Shells",
            cost: [
                Credits(7850),
                EnorPearl(135.0),
                Umanite(80.0),
                Bismor(100.0),
            ],
            guid: Guid([
                63, 104, 203, 124, 47, 171, 40, 67, 177, 68, 109, 19, 59, 217, 44, 29,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Double Barrel",
            cost: [
                Credits(7950),
                Umanite(125.0),
                EnorPearl(75.0),
                Croppa(100.0),
            ],
            guid: Guid([
                95, 9, 11, 179, 239, 201, 111, 66, 151, 241, 191, 195, 241, 82, 198, 99,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Compact Shells",
            cost: [Credits(8550), Umanite(120.0), Magnite(65.0), Jadiz(100.0)],
            guid: Guid([
                250, 64, 238, 74, 31, 140, 176, 79, 191, 175, 145, 100, 105, 228, 177, 103,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Special Powder",
            cost: [Credits(7050), Croppa(125.0), EnorPearl(65.0), Bismor(95.0)],
            guid: Guid([
                230, 218, 247, 214, 6, 80, 4, 67, 158, 249, 102, 214, 253, 189, 105, 166,
            ]),
            ty: Clean,
            dwarf: Scout,
        },
        Overclock {
            name: "Light-weight Rounds",
            cost: [Credits(8700), Umanite(135.0), Jadiz(65.0), Bismor(90.0)],
            guid: Guid([
                5, 177, 87, 160, 117, 224, 115, 75, 180, 113, 239, 141, 174, 105, 40, 101,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Well Oiled Machine",
            cost: [Credits(8400), Magnite(140.0), Croppa(65.0), Jadiz(95.0)],
            guid: Guid([
                206, 148, 144, 68, 91, 3, 107, 73, 184, 157, 178, 188, 141, 24, 253, 18,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "EM Refire Booster",
            cost: [Credits(8300), Jadiz(135.0), Bismor(60.0), EnorPearl(90.0)],
            guid: Guid([
                129, 184, 66, 49, 14, 1, 184, 71, 191, 153, 119, 77, 188, 61, 192, 221,
            ]),
            ty: Balanced,
            dwarf: Engineer,
        },
        Overclock {
            name: "Super-Slim Rounds",
            cost: [Credits(8550), Croppa(130.0), EnorPearl(75.0), Bismor(90.0)],
            guid: Guid([
                19, 207, 28, 74, 105, 146, 146, 74, 154, 255, 92, 234, 240, 91, 113, 6,
            ]),
            ty: Clean,
            dwarf: Engineer,
        },
        Overclock {
            name: "Turret EM Discharge",
            cost: [Credits(8450), Jadiz(125.0), Bismor(80.0), EnorPearl(105.0)],
            guid: Guid([
                12, 27, 170, 100, 176, 60, 185, 78, 187, 119, 183, 181, 94, 124, 108, 135,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
        Overclock {
            name: "Turret Arc",
            cost: [Credits(8350), Croppa(135.0), Umanite(60.0), Bismor(100.0)],
            guid: Guid([
                60, 66, 219, 234, 48, 33, 238, 68, 175, 109, 148, 74, 145, 49, 77, 117,
            ]),
            ty: Unstable,
            dwarf: Engineer,
        },
    ]
}
