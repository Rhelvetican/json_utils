//! JSON utilities.
use serde_json::Value;

use crate::error::Error;

/// Get the keys of a JSON object. Returns an `anyhow::Error` if the value is not an object.
pub fn get_keys(value: &Value) -> Result<Vec<String>, Error> {
    let mut keys = Vec::new();
    if let Value::Object(map) = value {
        for key in map.keys() {
            keys.push(key.to_string());
        }
    } else {
        return Err(Error::custom("Value is not an object"));
    }
    Ok(keys)
}

/// Get the values of a JSON object. Returns an `anyhow::Error` if the value is not an object.
pub fn get_values(value: &Value) -> Result<Vec<Value>, Error> {
    let mut values = Vec::new();
    if let Value::Object(map) = value {
        for value in map.values() {
            values.push(value.clone());
        }
    } else {
        return Err(Error::custom("Value is not an object"));
    }
    Ok(values)
}

/// Get the length of a JSON array. Returns an `anyhow::Error` if the value is not an array.
pub fn get_length(value: &Value) -> Result<usize, Error> {
    if let Value::Array(array) = value {
        Ok(array.len())
    } else {
        Err(Error::custom("Value is not an array"))
    }
}
