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

use std::{ops::Range, collections::HashMap, fmt::Write};
use indoc::indoc;

/// An &str containing the entirety of the license
pub const LICENSE: &str = include_str!("../COPYING");

/// The copyright notice that should be printed when `choco --about' is called
pub const ABOUT: &str = indoc!("
    Choco  Copyright (C) 2020  Liam Bloom
    This program comes with ABSOLUTELY NO WARRANTY; for details type `choco license -w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
");

/// A map that maps section #s to line # ranges for GPL 3.0
pub fn section_pages() -> HashMap<u8, Range<usize>> {
    map!(
        0 => 73..111,
        1 => 112..153,
        2 => 154..178,
        3 => 179..194,
        4 => 195..207,
        5 => 208..244,
        6 => 245..342,
        7 => 343..406,
        8 => 407..434,
        9 => 435..445,
        10 => 446..470,
        11 => 471..539,
        12 => 540..551,
        13 => 552..562,
        14 => 563..588,
        15 => 589..599,
        16 => 600..611,
        17 => 612..620
    )
}

pub fn section_text(mut sections: Vec<u8>) -> String {
    sections.sort_unstable();
    let lines = LICENSE.lines();
    let section_pages = section_pages();
    //let sections = sections.iter().map(|s| section_pages.get(s));
    let s = String::new();
    todo!();
    for i in sections {
        //writeln!(s, "{}", section_pages.get(&i))
    }
    s
}