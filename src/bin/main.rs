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
    //env,
    //process,
    fmt::Write,
    //include_str,
};
use choco::*;
use clap::clap_app;
use toml::Value;
use indoc::printdoc;

/*
TODO:

- Add `show w` and `show c` commands (or equivalent, see end of license)
*/

fn main() {
    let pkg = meta::package_data().unwrap();
    let version: &str = toml_get!(String pkg.version).as_str();
    let about: &str = toml_get!(String pkg.description).as_str();
    let authors = toml_get!(Array pkg.authors);
    let authors = match authors.len() {
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
    };
    let authors = authors.as_str();
    
    let matches = clap_app!(choco =>
        (version: version)
        (author: authors)
        (about: about)
        (@arg about: --about "Prints about information")
        (@subcommand license => 
            (about: "Prints information about the license")
            //(@arg choco_license: --choco "Prints choco license information")
            (@group info_type => 
                (@arg warranty: -w --warranty "Prints warranty information") // Should print sections 15 and 17 of GPL 3.0
                (@arg copying: -c --copying "Prints copying terms") // Should print sections 3-7 of GPL 3.0
                (@arg terms_and_conditions: -t --("terms-and-conditions") "Prints license terms and conditions")
                (@arg preamble: -p --preamble "Prints license preamble")
                (@arg section: -s --section [SECTION_NUMBER] ... "Prints the requested section")
                (@arg how_to_apply: -a --("how-to-apply") "Prints how to apply the license")
                (@arg full: -f --full "Prints the full license text")
            )
        )
    ).get_matches();

    if matches.is_present("about") {
        printdoc!("
            Choco  Copyright (C) 2020  Liam Bloom

            This program is a simple java package manager. I created this primarily for myself, to learn
            various java commands and learn how to create a command line app in rust, but it is available
            for anyone else who wants it.

            This program comes with ABSOLUTELY NO WARRANTY; for details type `choco license -w'.
            This is free software, and you are welcome to redistribute it
            under certain conditions; type `choco license -c' for details.
        ");
    }

    license::commands(&matches);
}