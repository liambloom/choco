/*
    Copyright (C) 2020 Liam Bloom

    This file is part of Choco.

    Choco is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Choco is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Choco.  If not, see <https://www.gnu.org/licenses/>.
*/

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