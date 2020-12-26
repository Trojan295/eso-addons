use std::collections::HashSet;

use addons::Addon;
use config::AddonEntry;

extern crate serde;
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate simple_error;
extern crate html5ever;
extern crate markup5ever_rcdom;
extern crate regex;
extern crate reqwest;
extern crate tempfile;
extern crate zip;

pub mod errors;

pub mod addons;
pub mod config;

pub fn get_missing_dependencies(installed: &Vec<Addon>) -> impl Iterator<Item = String> {
    let mut missing = HashSet::new();

    let mut addon_map = HashSet::new();
    for addon in installed.iter() {
        addon_map.insert(addon.name.clone());
    }

    for addon in installed.iter() {
        for dependency in addon.depends_on.iter() {
            if !addon_map.contains(dependency) {
                missing.insert(dependency.to_owned());
            }
        }
    }

    missing.into_iter()
}

pub fn get_unmanaged_addons<'a>(
    desired: &Vec<AddonEntry>,
    installed: &'a Vec<Addon>,
) -> Vec<&'a Addon> {
    let mut result = vec![];

    let mut desired_map = HashSet::new();
    for addon in desired.iter() {
        desired_map.insert(addon.name.clone());
    }

    for addon in installed.iter() {
        if !desired_map.contains(&addon.name) {
            result.push(addon);
        }
    }

    result
}
