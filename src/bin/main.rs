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

use std::{
    env,
    process,
    //include_str,
};
use choco::{
    meta::*,
    *,
};
use clap::clap_app;
use toml::Value;

/*
TODO:

- Add `show w` and `show c` commands (or equivalent, see end of license)
*/

fn main() {
    let pkg = package_data().unwrap();
    let version = toml_get!(String pkg.version);
    
    let matches = clap_app!(choco =>
        (version: version().unwrap().as_str())
    ).get_matches();
}