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

use std::fmt::Write;
use toml::{map::Map, Value};
use indoc::indoc;
use crate::error;

#[macro_export]
macro_rules! toml_get {
    ($variant:ident $toml:ident.$property:ident) => {{
        let property = (&$toml).get(stringify!($property)).expect(format!("{}.{} does not exist", stringify!($toml), stringify!($ident)).as_str());
        toml_get!($variant property)
    }};
    ($variant:ident $value:expr) => {
        if let Value::$variant(v) = $value {
            v.to_owned() // TODO
        } 
        else {
            panic!("{} is not a {}", stringify!($value), stringify!($type));
        }
    };
}

pub const ABOUT: &str = indoc!("
    Choco  Copyright (C) 2020  Liam Bloom

    This program is a simple java package manager. I created this primarily for myself, to learn
    various java commands and learn how to create a command line app in rust, but it is available
    for anyone else who wants it.

    This program comes with ABSOLUTELY NO WARRANTY; for details type `choco license -w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `choco license -c' for details.
");

pub struct Package {
    pub toml: Map<String, Value>,
    pub version: String,
    pub about: String,
    pub authors: String,
}

impl Package {
    pub fn new() -> Self {
        let pkg = Self::data();
        let authors = toml_get!(Array pkg.authors);
        Self {
            version: toml_get!(String pkg.version),
            about: toml_get!(String pkg.description),
            authors: match authors.len() {
                1 => toml_get!(String &authors[0]).to_owned(),
                2 => format!("{} and {}", toml_get!(String &authors[0]), toml_get!(String &authors[1])),
                l => {
                    let mut s = String::new();
                    for i in authors.iter().enumerate() {
                        if i.0 != l - 1 {
                            write!(&mut s, ", ").unwrap();
                        }
                        if i.0 == l - 2 {
                            write!(&mut s, "and ").unwrap();
                        }
                        write!(&mut s, "{}", toml_get!(String i.1)).unwrap();
                    }
                    s
                }
            },
            toml: pkg,
        }
    }
    pub fn data() -> Map<String, Value> {
        if let Ok(map) = include_str!("../Cargo.toml").parse::<Value>() {
            if let Value::Table(map) = map {
                if let Some(map) = map.get("package") {
                    if let Value::Table(map) = map {
                        return map.to_owned();
                    }
                }
            }
        }
        error!("Failed to get package data");
    }
}