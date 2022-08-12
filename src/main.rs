mod cli;
mod entity;
mod initialize;
mod settings;

use std::{collections::HashMap, process};

use entity::Cheatsheet;
use settings::Settings;

use clap::Parser;

#[derive(Debug)]
enum Error {
    Settings(settings::SettingsError),
    Init(initialize::InitError),
}

fn main() -> Result<(), Error> {
    let settings = Settings::load().map_err(Error::Settings)?;
    let cheatsheets = initialize::load_cheatsheets(&settings).map_err(Error::Init)?;

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Command::List => {
            list_cheatsheets(cheatsheets, settings);
        }
        cli::Command::Show {
            cheatsheet,
            section,
        } => {
            display_cheatsheet(cheatsheet, section, cheatsheets, settings);
        }
        cli::Command::Edit { cheatsheet } => {
            edit_cheatsheet(cheatsheet, settings);
        }
    }

    Ok(())
}

fn display_cheatsheet(
    cheatsheet: &str,
    section: &Option<String>,
    cheatsheets: HashMap<String, Cheatsheet>,
    settings: Settings,
) {
    cheatsheets[cheatsheet].display(section, settings.width);
}

fn list_cheatsheets(cheatsheets: HashMap<String, Cheatsheet>, settings: Settings) {
    for cheatsheet in cheatsheets.keys() {
        cheatsheets[cheatsheet].display_name(settings.width);
    }
}

fn edit_cheatsheet(cheatsheet: &str, settings: Settings) {
    let file_path = {
        let mut file_path = settings.cheatsheets_path;
        file_path.push(format!("{cheatsheet}.yaml"));
        file_path
    };

    process::Command::new(settings.editor)
        .arg(&file_path)
        .status()
        .unwrap();
}
