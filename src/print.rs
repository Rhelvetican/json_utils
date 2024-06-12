//! Pretty-print JSON.

use anyhow::Result;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

/// Pretty-print a serializable value as JSON.

pub fn print_json<T: Serialize>(value: T) -> Result<()> {
    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser)?;
    println!("{}", String::from_utf8(buf)?);
    Ok(())
}

/// Pretty-print a serializable value as JSON with a custom indentation.

pub fn print_json_with_indent<T: Serialize>(value: T, indent: usize) -> Result<()> {
    let indent = " ".repeat(indent);
    let indent = indent.as_bytes();
    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(indent);
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser)?;
    println!("{}", String::from_utf8(buf)?);
    Ok(())
}
