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