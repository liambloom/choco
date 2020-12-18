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

use std::process;
use choco::*;
use clap::clap_app;

fn main() -> ! {
    loop  {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("foo"); 
        println!("{:?}", find_it(&input.trim()));
    }
    let pkg = choco::meta::Package::new();
    let matches = clap_app!(choco =>
        (version: pkg.version.as_str())
        (author: pkg.authors.as_str())
        (about: pkg.about.as_str())
        (@arg about: --about "Prints about information")
        (@subcommand license => 
            (version: "3.0-or-later")
            (about: "Prints information about the license")
            (@group info_type => 
                (@arg warranty: -w --warranty "Prints warranty information") // Should print sections 15 and 17 of GPL 3.0
                (@arg copying: -c --copying "Prints copying terms") // Should print sections 3-7 of GPL 3.0
                (@arg section: -s --section [SECTION_NUMBER] ... "Prints the requested section(s)")
                (@arg terms_and_conditions: -t --("terms-and-conditions") "Prints license terms and conditions")
                (@arg preamble: -p --preamble "Prints license preamble")
                (@arg how_to_apply: -a --("how-to-apply") "Prints how to apply the license")
                (@arg full: -f --full "Prints the full license text")
            )
        )
    ).get_matches();

    if matches.is_present("about") {
        println!("{}", meta::ABOUT);
        process::exit(0);
    }

    license::commands(&matches);
}