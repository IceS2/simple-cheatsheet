use super::Cheat;

use crossterm::style::Stylize;

#[derive(Debug)]
pub(crate) struct Section {
    pub(crate) name: String,
    pub(crate) cheats: Vec<Cheat>,
}

impl Section {
    pub(crate) fn new(name: String, cheats: Vec<Cheat>) -> Self {
        Section { name, cheats }
    }

    pub(crate) fn display(&self, width: usize) {
        let margin = " ".repeat(2);

        let divider = "-".repeat(width).cyan();

        println!("{}{}", margin, self.name.clone().bold().magenta());
        println!("{}", divider);
        for cheat in self.cheats.iter() {
            cheat.display(width);
        }
    }
}
