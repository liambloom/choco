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

/// The macros of this crate that don't really fit in anywhere else
pub mod macros;
/// Gets metadata about the package from Cargo.toml
pub mod meta;
/// Finds files
pub mod files;
/// For retrieving license information
pub mod license;
/// Initializes a java package
pub mod init;
/// Module for config related stuff
pub mod config;
/// Module for error handling
pub mod error_handling;
/// A module to find an executable in the PATH
pub mod find_exe;