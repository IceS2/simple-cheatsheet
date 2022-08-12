use crate::entity::{Cheat, Cheatsheet, Section};

use crate::settings::Settings;

use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub(crate) enum InitError {
    Parse(serde_yaml::Error),
    IO(std::io::Error),
}

pub(crate) fn load_cheatsheets(
    settings: &Settings,
) -> Result<HashMap<String, Cheatsheet>, InitError> {
    let dir_entries = fs::read_dir(&settings.cheatsheets_path).map_err(InitError::IO)?;

    let cheatsheet_sources = dir_entries.into_iter().filter(|entry| {
        if let Ok(entry) = entry {
            entry.path().extension().unwrap() == "yaml"
        } else {
            false
        }
    });

    let mut cheatsheets = HashMap::new();

    for cheatsheet in cheatsheet_sources {
        let contents =
            fs::read_to_string(cheatsheet.as_ref().unwrap().path()).map_err(InitError::IO)?;
        let name = cheatsheet
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let sections: HashMap<String, Section> = {
            let sections: HashMap<String, Vec<Cheat>> =
                serde_yaml::from_str(&contents).map_err(InitError::Parse)?;
            sections
                .into_iter()
                .map(|(key, value)| (key.clone(), Section::new(key, value)))
                .collect::<HashMap<String, Section>>()
        };
        cheatsheets.insert(name.clone(), Cheatsheet::new(name, sections));
    }
    Ok(cheatsheets)
}
