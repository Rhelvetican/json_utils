//! JSON utilities.

use serde_json::Value;

/// Get the keys of a JSON object.

pub fn get_keys(value: &Value) -> Vec<String> {
    let mut keys = Vec::new();
    if let Value::Object(map) = value {
        for key in map.keys() {
            keys.push(key.to_string());
        }
    }
    keys
}

/// Get the values of a JSON object.

pub fn get_values(value: &Value) -> Vec<Value> {
    let mut values = Vec::new();
    if let Value::Object(map) = value {
        for value in map.values() {
            values.push(value.clone());
        }
    }
    values
}

/// Get the length of a JSON array.

pub fn get_length(value: &Value) -> usize {
    if let Value::Array(array) = value {
        array.len()
    } else {
        0
    }
}
