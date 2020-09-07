use toml::{map::Map, Value};

pub fn package_data() -> Option<Map<String, Value>> {
    if let Value::Table(map) = include_str!("../Cargo.toml").parse::<Value>().ok()? {
        if let Value::Table(map) = map.get("package")? {
            Some(map.to_owned())
        }
        else {
            None
        }
    }
    else {
        None
    }
}

#[macro_export]
macro_rules! toml_get {
    ($variant:ident $toml:ident.$property:ident) => {
        if let Value::$variant(v) = $toml.get(stringify!($property)).unwrap() { v } else { unreachable!() }
    };
}