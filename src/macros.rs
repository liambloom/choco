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

#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),*) => {{
        let mut m = std::collections::HashMap::new();
        $(
            m.insert($key, $value);
        )*
        m
    }};
}