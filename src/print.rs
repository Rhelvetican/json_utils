//! Pretty-printing JSONs with ease.

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

use crate::error::Error;

/// Pretty-print a serializable value as JSON with a default indentation of 4 spaces.
/// Will returns an [Error] if the value cannot be serialized or if the output is not valid UTF-8.
///
/// ```rust
/// use jsonutils::print::print_json;
/// use serde_json::json;
///
/// let json = json!({
///    "name": "Alice",
///    "age": 30,
///    "is_student": false
/// });
///
/// print_json(json).unwrap();
/// ```

pub fn print_json<T: Serialize>(value: T) -> Result<(), Error> {
    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser).map_err(Error::json)?;
    println!("{}", String::from_utf8(buf).map_err(Error::utf8)?);
    Ok(())
}

/// Pretty-print a serializable value as JSON with a custom indentation.
/// Will returns an [Error] if the value cannot be serialized or if the output is not valid UTF-8.
/// 
/// ```rust
/// use jsonutils::print::print_json_with_indent;
/// use serde_json::json;
///
/// let json = json!({
///    "name": "Alice",
///    "age": 30,
///    "is_student": false
/// });
///
/// print_json_with_indent(json, 2).unwrap();
/// ```

pub fn print_json_with_indent<T: Serialize>(value: T, indent: usize) -> Result<(), Error> {
    let indent = " ".repeat(indent);
    let indent = indent.as_bytes();
    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(indent);
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser).map_err(Error::json)?;
    println!("{}", String::from_utf8(buf).map_err(Error::utf8)?);
    Ok(())
}
