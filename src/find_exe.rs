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

use std::env;
use std::path::{Path, PathBuf};

/// This is based off of code created by Shepmaster on stackoverflow
fn find_it<P>(exe_name: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } 
            else {
                None
            }
        }).next()
    })
}

#[cfg(not(target_os = "windows"))]
fn add_file_ext<P>(path: P) -> P
    where P: AsRef<Path>,
{
    path
}

#[cfg(target_os = "windows")]
fn add_file_ext<P>(path: P) -> P
    where P: AsRef<Path>,
{
    if path.as_ref().extension().is_some() {
        path
    }
    else {
        env::var_os("PATHEXT").and_then(|exts| {
            env::split_paths(&exts).filter_map(|ext| {
                let with_ext = full_path.with_extension(ext);
                if with_ext.is_file() {
                    println!("found: {:?}", with_ext);
                    Some(with_ext)
                }
                else {
                    None
                }
            }).next()
        }).next()
    }
}