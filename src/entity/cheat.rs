use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Cheat {
    description: String,
    command: String,
}

impl Cheat {
    pub(crate) fn display(&self, width: usize) {
        let margin_size = 4;
        let margin = " ".repeat(margin_size);

        let perceived_width =
            width - (margin_size * 2) - self.description.len() - self.command.len();

        println!(
            "{}{}{}{}{}",
            margin,
            self.description.clone().bold(),
            " ".repeat(perceived_width),
            self.command.clone().bold().blue(),
            margin
        );
    }
}
