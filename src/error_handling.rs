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

use std::fmt::Display;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        eprint!("\x1b[31merror\x1b[0m: ");
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! warn { // Unused
    ($($arg:tt)*) => {{
        eprint!("\x1b[33warning\x1b[0m: ");
        eprintln!($($arg)*);
    }};
}

pub trait Unwrap<T> {
    fn expect(self, msg: &str) -> T;
    fn unwrap_or(self ,msg: &str, default: T) -> T;
}

pub trait UnwrapDefault<T>: Unwrap<T> {
    fn unwrap(self) -> T;

}

impl<T> Unwrap<T> for Option<T> {
    fn expect(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => error!("{}", msg),
        }
    }
    fn unwrap_or(self, msg: &str, default: T) -> T {
        match self {
            Some(v) => v,
            None => {
                warn!("{}", msg);
                default
            }
        }
    }
}

impl<T, E> Unwrap<T> for Result<T, E> {
    fn expect(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => error!("{}", msg),
        }
    }
    fn unwrap_or(self, msg: &str, default: T) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                warn!("{}", msg);
                default
            }
        }
    }
}

impl<T, E: Display> UnwrapDefault<T> for Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => error!("{}", e),
        }
    }
}