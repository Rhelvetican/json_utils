use anyhow::Result;
use jsonutils::{file::write_json, prelude::print_json, print::print_json_with_indent};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Object<'a> {
    id: &'a str,
    name: &'a str,
}

impl<'a> Object<'a> {
    fn new(id: &'a str, name: &'a str) -> Self {
        Self { id, name }
    }
}

fn main() -> Result<()> {
    let obj = Object::new("1", "Alice");
    let obj1 = Object::new("2", "Bob");
    let obj2 = Object::new("3", "Charlie");
    let obj3 = Object::new("4", "David");
    let obj4 = Object::new("5", "Eve");
    print_json(&obj)?;
    print_json_with_indent(&obj, 2)?;
    write_json("./json/object.json", &obj)?;
    let vec = vec![obj, obj1, obj2, obj3, obj4];
    write_json("./json/objects.json", vec)?;

    Ok(())
}
