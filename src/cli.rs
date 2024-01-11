use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "wrt")]
pub struct Args {
    #[arg(help = "The file to edit.")]
    pub file: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
