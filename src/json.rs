//! JSON utilities.
use crate::error::Error;
use serde_json::Value;

/// Get the keys of a JSON object. Returns an [Error] if the value is not an object.
pub fn get_keys(value: &Value) -> Result<Vec<String>, Error> {
    if let Value::Object(map) = value {
        Ok(map.keys().cloned().collect())
    } else {
        Err(Error::custom("Value is not an object."))
    }
}

/// Get the values of a JSON object. Returns an [Error] if the value is not an object.
pub fn get_values(value: &Value) -> Result<Vec<Value>, Error> {
    if let Value::Object(map) = value {
        Ok(map.values().cloned().collect::<Vec<_>>())
    } else {
        Err(Error::custom("Value is not an object."))
    }
}

/// Get the length of a JSON array or a JSON object. Returns an [Error] if the value is not an array.
pub fn get_length(value: &Value) -> Result<usize, Error> {
    if let Value::Array(array) = value {
        Ok(array.len())
    } else if let Value::Object(map) = value {
        Ok(map.len())
    } else {
        Err(Error::custom("Value is not an array."))
    }
}
