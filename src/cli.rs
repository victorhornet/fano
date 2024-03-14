use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "wrt")]
pub struct Args {
    #[arg(help = "The file to edit. If not set, the text will be printed to the console.")]
    pub file: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}
