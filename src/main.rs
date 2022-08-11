mod config;

use std::fs;
use std::collections::HashMap;

use crossterm::style::Stylize;
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command
}


#[derive(Subcommand)]
enum Command {
    Show {
        #[clap(value_parser)]
        cheatsheet: String
    },
    List
}

#[derive(Debug, Serialize, Deserialize)]
struct Cheatsheet {
    name: String,
    sections: HashMap<String, Section>
}

impl Cheatsheet {
    fn filter_tag(&self, tag: &str) -> HashMap<String, Section> {
        let mut filtered_map = HashMap::new();
        for section in self.sections.keys() {
            let filtered_cheats = self.sections[section].filter_tag(tag);
            if filtered_cheats.len() > 0 {
                filtered_map.insert(section.clone(), Section { cheats: filtered_cheats.into() });
            }
        }
        filtered_map
    }

    fn display_name(&self) {
        let perceived_width = 78 - self.name.len();

        println!("{}{}{}", "\u{250C}".cyan(), "\u{2500}".repeat(78).cyan(), "\u{2510}".cyan());
        println!("{}{}{}{}{}", "\u{2502}".cyan(), " ".repeat((perceived_width as f32 / 2.0).floor() as usize), self.name.clone().bold().magenta(), " ".repeat((perceived_width as f32 / 2.0).ceil() as usize), "\u{2502}".cyan());
        println!("{}{}{}\n", "\u{2514}".cyan(), "\u{2500}".repeat(78).cyan(), "\u{2518}".cyan());
    }

    fn display(&self) {
        self.display_name();
        for section in self.sections.keys() {
            println!("{}{}", " ".repeat(2), section.clone().bold().magenta());
            println!("{}", "-".repeat(80).cyan());
            self.sections[section].display();
            println!();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Section {
    cheats: Vec<Cheat>
}

impl Section {
    fn filter_tag(&self, tag: &str) -> Vec<Cheat> {
        self.cheats.iter().filter(|cheat| cheat.has_tag(tag)).map(|cheat| cheat.clone()).collect()
    }

    fn display(&self) {
        for cheat in self.cheats.iter() {
            cheat.display(30);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cheat {
    description: String,
    command: String,
    tags: Option<Vec<String>>
}

impl Cheat {
    fn has_tag(&self, tag: &str) -> bool {
        if let Some(ref tags) = self.tags {
            tags.contains(&tag.to_string())
        } else {
            false
        }
    }

    fn display(&self, width: usize) {
        let perceived_width = 72 - (self.description.len() + self.command.len());
        println!("{}{}{}{}{}",
            " ".repeat(4),
            self.description.clone().bold(),
            " ".repeat(perceived_width),
            self.command.clone().bold().blue(),
            " ".repeat(4));
    }
}

#[derive(Debug)]
enum Error {
    ConfigError(config::ConfigError),
    IOError(std::io::Error),
    SerdeError(serde_yaml::Error)

}

fn load_cheatsheets() -> Result<HashMap<String, Cheatsheet>, Error> {
    let config = config::get_config().map_err(|e| Error::ConfigError(e))?;

    let mut cheatsheet_path = config.cheatsheets_path.clone();
    cheatsheet_path.push("test.yaml");


    let cheatsheet_sources = [
        cheatsheet_path
    ];

    let mut cheatsheets = HashMap::new();

    for cheatsheet in cheatsheet_sources {
        let contents = fs::read_to_string(cheatsheet).map_err(|e| Error::IOError(e))?;
        cheatsheets.insert("test".to_string(), serde_yaml::from_str(&contents).map_err(|e| Error::SerdeError(e))?);
    }
    Ok(cheatsheets)
}

fn main() -> Result<(), Error> {
    let cheatsheets = load_cheatsheets()?;

    let cli = Cli::parse();

    match &cli.command {
        Command::List => {
            println!("{:?}", cheatsheets.keys());
        },
        Command::Show { cheatsheet } => {
            cheatsheets[cheatsheet].display()
        }
    }


    Ok(())
}
