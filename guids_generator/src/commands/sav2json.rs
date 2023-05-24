use std::{collections::HashMap, error, fs::File, path::PathBuf, process::exit};

use gvas::GvasFile;

use super::create_write_pretty;

pub fn run(file: PathBuf, out: PathBuf) {
    match inner(file) {
        Ok(gvas) => create_write_pretty(&out, gvas.properties),
        Err(e) => {
            eprintln!("Error when reading save file: {e}");
            exit(1);
        }
    }
}

pub fn inner(file: PathBuf) -> Result<GvasFile, Box<dyn error::Error>> {
    let mut file = File::open(file)?;

    Ok(GvasFile::read_with_hints(&mut file, &get_hints())?)
}

fn get_hints() -> HashMap<String, String> {
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
