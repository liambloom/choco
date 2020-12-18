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

use std::{fs, path::Path, env::current_dir};
use clap::ArgMatches;
use toml::Value;
use crate::error_handling::*;

const DEFAULTS: &str = include_str!("../defaults.toml");
const CONFIG_NAMES: [&str; 2] = ["choco.toml", ".chocorc"];

pub struct Config<'a> {
    matches: &'a ArgMatches<'a>,
    files: Vec<Value>,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches<'a>) -> Self {
        let mut files = Vec::new();
        for dir in UnwrapDefault::unwrap(current_dir()).ancestors() {
            for filename in CONFIG_NAMES.iter() {
                let file = dir.with_file_name(filename);
                if file.exists() {
                    files.push(
                        Unwrap::expect(
                            UnwrapDefault::unwrap(
                                fs::read_to_string(file)
                            ).parse(),
                            format!("The file {} contains invalid toml syntax", filename).as_str()
                        )
                    );
                }
            }
        }
        files.push(UnwrapDefault::unwrap(DEFAULTS.parse()));
        Self {
            matches,
            files,
        }
    }
    /*pub fn get(&self, key: &str) -> Option<Value> {
        match self.matches.value_of(key) {
            Some(v) => Some(Value::String(v.to_string())),
            None => {
                /*for file in self.files.iter() {
                    if (file.)
                }*/
            }
        }
    }*/
}