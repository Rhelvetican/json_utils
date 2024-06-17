//! This module provides functions to read and write JSON files.

use std::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, ser::PrettyFormatter, Serializer};

use crate::INDENT;

/// Reads a JSON file and deserializes it into an object.

pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T> {
    let content = read_to_string(path)?;
    Ok(from_str(&content)?)
}

/// Writes an object to a JSON file.
/// This function will error if the file cannot be created or written to.

pub fn write_json<T: Serialize, P: AsRef<Path>>(path: P, value: T) -> Result<()> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or(Path::new("./"));
    if !parent.exists() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(INDENT);
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser)?;
    file.write_all(&buf)?;

    Ok(())
}

/// Writes an object to a JSON file with a custom indentation.

pub fn write_json_with_indent<T: Serialize, P: AsRef<Path>>(
    path: P,
    value: T,
    indent: usize,
) -> Result<()> {
    let indent = " ".repeat(indent);
    let indent = indent.as_bytes();
    let path = path.as_ref();
    let parent = path.parent().unwrap_or(Path::new("./"));
    if !parent.exists() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(indent);
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser)?;
    file.write_all(&buf)?;

    Ok(())
}
