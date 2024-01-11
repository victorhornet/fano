use color_eyre::eyre::Result;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to open file: {0}")]
    OpenFile(#[from] std::io::Error),
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(path)
        .map_err(Error::OpenFile)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn write_file(path: &PathBuf, text: &str) -> Result<()> {
    File::create(path)
        .map_err(Error::OpenFile)?
        .write_all(text.as_bytes())?;
    Ok(())
}
