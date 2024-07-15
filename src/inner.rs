use std::{
    fs::{create_dir_all, read_to_string},
    path::Path,
};

use serde_json::{from_str, Value};

use crate::error::Error;

#[doc(hidden)]
pub(crate) fn _create_parent_dir<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or(Path::new("./"));

    if path.to_str().is_some() && path.to_str() != Some("./") {
        create_dir_all(parent).map_err(Error::io)?;
    }

    Ok(())
}

#[doc(hidden)]
pub(crate) fn _read_json_inner<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
    let content = read_to_string(path).map_err(Error::io)?;
    from_str(&content).map_err(Error::json)
}
