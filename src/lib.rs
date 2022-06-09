use std::collections::{HashMap, HashSet};

use addons::Addon;
use config::AddonEntry;

extern crate colored;
extern crate regex;
extern crate requestty;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_derive;
extern crate tempfile;
extern crate toml;
extern crate walkdir;
extern crate zip;

pub mod addons;
pub mod config;
pub mod errors;
pub mod htmlparser;

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

pub fn get_unmanaged_addons<'a, I>(desired: &Vec<AddonEntry>, installed: I) -> Vec<&'a Addon>
where
    I: Iterator<Item = &'a Addon>,
{
    let mut result = vec![];

    let mut desired_map = HashSet::new();
    for addon in desired.iter() {
        desired_map.insert(addon.name.clone());
    }

    for addon in installed {
        if !desired_map.contains(&addon.name) {
            result.push(addon);
        }
    }

    result
}

pub fn get_unused_dependencies(installed: &Vec<Addon>, desired: &Vec<AddonEntry>) -> Vec<String> {
    let mut dep_graph: HashMap<String, HashSet<String>> = HashMap::new();

    for addon in installed.iter() {
        if !dep_graph.contains_key(&addon.name) {
            dep_graph.insert(addon.name.clone(), HashSet::new());
        }

        for dependency in addon.depends_on.iter() {
            match dep_graph.get_mut(dependency) {
                Some(set) => {
                    set.insert(addon.name.to_owned());
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(addon.name.to_owned());
                    dep_graph.insert(dependency.to_owned(), set);
                }
            }
        }
    }

    let mut unused_addons = vec![];

    for (addon, dependency_for) in dep_graph.iter() {
        if dependency_for.len() == 0 {
            let addon_config = desired.iter().find(|x| x.name == addon.to_owned());
            let unused = addon_config.map(|x| x.dependency).unwrap_or(true);

            if unused {
                unused_addons.push(addon.to_owned())
            }
        }
    }

    unused_addons
}
