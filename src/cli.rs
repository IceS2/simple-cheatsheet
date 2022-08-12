use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Show {
        #[clap(value_parser)]
        cheatsheet: String,

        #[clap(short, long)]
        section: Option<String>,
    },
    List,
    Edit {
        #[clap(value_parser)]
        cheatsheet: String,
    },
}
