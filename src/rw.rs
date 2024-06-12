//! This module provides functions to read and write JSON files.

use std::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::Result;
use serde::Serialize;
use serde_json::{from_str, ser::PrettyFormatter, Serializer, Value};

/// Reads a JSON file and returns a `serde_json::Value`.
/// This function will error if the file does not exist or if the file is not valid JSON.

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    Ok(from_str(&read_to_string(path)?)?)
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
    let fmtr = PrettyFormatter::with_indent(b"    ");
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
