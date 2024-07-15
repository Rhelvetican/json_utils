//! This module provides functions to read and write JSON files.
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, from_value, ser::PrettyFormatter, Serializer, Value};

use crate::error::Error;

#[doc(hidden)]
fn _read_json_inner<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
    let content = read_to_string(path).map_err(Error::io)?;
    from_str(&content).map_err(Error::json)
}

/// Reads a JSON file and deserializes it into an object.
/// This function will error if the file cannot be read or deserialized.
pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T, Error> {
    let val = _read_json_inner(path)?;
    from_value(val).map_err(Error::json)
}

#[doc(hidden)]
fn _create_parent_dir<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or(Path::new("./"));

    if path.to_str().is_some() && path.to_str() != Some("./") {
        create_dir_all(parent).map_err(Error::io)?;
    }

    Ok(())
}

/// Writes an object to a JSON file.
/// This function will error if the file cannot be created or written to.
pub fn write_json<T: Serialize, P: AsRef<Path>>(path: P, value: T) -> Result<(), Error> {
    write_json_with_indent(path, value, 4)?;
    Ok(())
}

/// Writes an object to a JSON file with a custom indentation.
pub fn write_json_with_indent<T: Serialize, P: AsRef<Path>>(path: P, value: T, indent: usize) -> Result<(), Error> {
    _create_parent_dir(&path)?;

    let indent = " ".repeat(indent);
    let indent = indent.as_bytes();

    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(indent);
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser).map_err(Error::json)?;
    write(path, buf).map_err(Error::io)?;

    Ok(())
}
