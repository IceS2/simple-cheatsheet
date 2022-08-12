use std::collections::HashMap;

use super::Section;

use crossterm::style::Stylize;

#[derive(Debug)]
pub struct Cheatsheet {
    name: String,
    sections: HashMap<String, Section>,
}

impl Cheatsheet {
    pub(crate) fn new(name: String, sections: HashMap<String, Section>) -> Self {
        Cheatsheet { name, sections }
    }

    pub(crate) fn display_name(&self, width: usize) {
        let perceived_width = width - 2 - self.name.len();

        println!(
            "{}{}{}",
            "\u{250C}".cyan(),
            "\u{2500}".repeat(width - 2).cyan(),
            "\u{2510}".cyan()
        );
        println!(
            "{}{}{}{}{}",
            "\u{2502}".cyan(),
            " ".repeat((perceived_width as f32 / 2.0).floor() as usize),
            self.name.clone().bold().magenta(),
            " ".repeat((perceived_width as f32 / 2.0).ceil() as usize),
            "\u{2502}".cyan()
        );
        println!(
            "{}{}{}\n",
            "\u{2514}".cyan(),
            "\u{2500}".repeat(width - 2).cyan(),
            "\u{2518}".cyan()
        );
    }

    pub fn display(&self, section_name: &Option<String>, width: usize) {
        self.display_name(width);
        if let Some(section) = section_name {
            self.sections[section].display(width);
        } else {
            for section in self.sections.keys() {
                self.sections[section].display(width);
                println!();
            }
        }
    }
}
