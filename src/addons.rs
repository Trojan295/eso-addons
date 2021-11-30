use crate::htmlparser;

use super::errors::ErrorChain;
use html5ever::tendril::TendrilSink;
use html5ever::{self, tree_builder::TreeBuilderOpts, ParseOpts};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::{error::Error, rc::Rc};
use tempfile::tempfile;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Addon {
    pub name: String,
    pub depends_on: Vec<String>,
}

pub struct AddonList {
    pub addons: Vec<Addon>,
    pub errors: Vec<Box<dyn Error>>,
}

pub struct Manager {
    addon_dir: PathBuf,
}

fn extract_dependency(dep: &str) -> Option<String> {
    let re = Regex::new(r"^(.+?)(([<=>]+)(.*))?$").unwrap();
    re.captures(dep).map(|captures| captures[1].to_owned())
}

impl Manager {
    pub fn new(addon_dir: &Path) -> Manager {
        let path = PathBuf::from(addon_dir);

        Manager { addon_dir: path }
    }

    pub fn get_addons(&self) -> Result<AddonList, Box<dyn Error>> {
        let mut addon_list = AddonList {
            addons: vec![],
            errors: vec![],
        };

        for entry in WalkDir::new(&self.addon_dir) {
            let entry_dir = entry?;
            let file_path = entry_dir.path();

            let file_name = entry_dir.file_name();
            let parent_dir_name = file_path.parent().map(|f| f.file_name()).flatten();

            match parent_dir_name {
                None => continue,
                Some(parent_dir_name) => {
                    let mut name = parent_dir_name.to_os_string();
                    name.push(".txt");
                    if name != file_name {
                        continue;
                    }
                }
            }

            let addon_dir = file_path.parent().unwrap();

            match self.read_addon(addon_dir) {
                Ok(addon) => addon_list.addons.push(addon),
                Err(err) => addon_list
                    .errors
                    .push(format!("while reading addon {:?}: {}", file_path, err).into()),
            }
        }

        Ok(addon_list)
    }

    pub fn get_addon(&self, name: &str) -> Result<Option<Addon>, Box<dyn Error>> {
        let addon_list = self.get_addons().chain_err("while getting addons")?;
        let found = addon_list.addons.into_iter().find(|x| x.name == name);
        Ok(found)
    }

    fn read_addon(&self, path: &Path) -> Result<Addon, Box<dyn Error>> {
        let addon_name = path
            .file_name()
            .ok_or(simple_error!("cannot get filename"))?;

        let addon_name = addon_name
            .to_str()
            .ok_or(simple_error!("failed to get addon name"))?;

        let file = self.open_addon_metadata_file(path, addon_name)?;
        let re = Regex::new(r"## (.*): (.*)").unwrap();

        let mut addon = Addon {
            name: addon_name.to_owned(),
            depends_on: vec![],
        };

        let lines = io::BufReader::new(file).lines();
        for line in lines {
            match line {
                Ok(line) => {
                    if line.starts_with("## DependsOn:") {
                        let depends_on = match re.captures(&line) {
                            Some(ref captures) => captures[2]
                                .split(" ")
                                .map(|s| s.to_owned())
                                .into_iter()
                                .filter_map(|s| extract_dependency(&s).to_owned())
                                .collect(),
                            None => vec![],
                        };

                        addon.depends_on = depends_on;
                    }
                }
                _ => {}
            }
        }

        Ok(addon)
    }

    pub fn delete_addon(&self, addon: &Addon) -> Result<(), Box<dyn Error>> {
        let mut addon_path = self.addon_dir.to_owned();
        addon_path.push(&addon.name);

        fs::remove_dir_all(addon_path).chain_err("while removing addon directory")?;

        Ok(())
    }

    pub fn download_addon(&self, url: &str) -> Result<Addon, Box<dyn Error>> {
        let mut response = reqwest::blocking::get(url)?;

        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let dom = html5ever::parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut response)?;

        let download_link = get_cdn_download_link(&dom);

        let download_link =
            download_link.ok_or(simple_error!("failed to get CDN download link"))?;

        let mut response = reqwest::blocking::get(&download_link)?;

        let mut tmpfile = tempfile()?;
        response.copy_to(&mut tmpfile)?;
        let mut archive = zip::ZipArchive::new(tmpfile)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => {
                    let mut p = self.addon_dir.clone();
                    p.push(path);
                    p
                }

                None => continue,
            };

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }

        let mut addon_path = self.addon_dir.to_owned();
        let addon_name = archive.by_index(0)?;
        let addon_name = get_root_dir(&addon_name.mangled_name());
        addon_path.push(addon_name);

        let addon = self
            .read_addon(&addon_path)
            .chain_err("while reading addon")?;

        Ok(addon)
    }

    fn open_addon_metadata_file(
        &self,
        path: &Path,
        addon_name: &str,
    ) -> Result<File, Box<dyn Error>> {
        let mut filepath = path.to_owned();
        let mut filepath_lowercase = path.to_owned();

        let filename = PathBuf::from(format!("{}.txt", addon_name));
        let filename_lowercase = PathBuf::from(format!("{}.txt", addon_name.to_lowercase()));

        filepath.push(filename);
        filepath_lowercase.push(filename_lowercase);

        if filepath.exists() {
            File::open(&filepath).chain_err(&format!("failed to open {:?}", &filepath))
        } else if filepath_lowercase.exists() {
            File::open(&filepath_lowercase)
                .chain_err(&format!("failed to open {:?}", &filepath_lowercase))
        } else {
            Err("metadata file is missing".into())
        }
    }
}

fn get_cdn_download_link(dom: &RcDom) -> Option<String> {
    let node = htmlparser::find_first_in_node(&dom.document, &|node: &Rc<Node>| match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            if &name.local == "a" {
                for attr in attrs.borrow().iter() {
                    if &attr.name.local == "href" {
                        if attr.value.starts_with("https://cdn.esoui.com") {
                            return Some(attr.value.clone());
                        }
                    }
                }
            }
            None
        }
        _ => None,
    });

    match node {
        Some(node) => Some(node.to_string()),
        None => None,
    }
}

pub fn get_download_url(addon_url: &str) -> Option<String> {
    let fns = vec![
        |url: &str| {
            let re = Regex::new(r"^https://.*esoui\.com/downloads/info(\d+)-(.+)$").unwrap();
            re.captures(url).map(|captures| {
                format!(
                    "https://www.esoui.com/downloads/download{}",
                    captures[1].to_owned(),
                )
            })
        },
        |url: &str| Some("ok".to_owned()),
    ];

    for f in fns {
        let url = f(addon_url);
        if url.is_some() {
            return url;
        }
    }

    None
}

fn get_root_dir(path: &Path) -> PathBuf {
    match path.parent() {
        None => path.to_owned(),
        Some(parent) => match parent.to_str().unwrap() {
            "" => path.to_owned(),
            &_ => get_root_dir(parent),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_link() {
        let tests: Vec<(&str, Option<String>)> = vec![
            (
                "https://www.esoui.com/downloads/info1360-CombatMetrics",
                Some("https://www.esoui.com/downloads/download1360".to_string()),
            ),
            (
                "https://www.esoui.com/downloads/info1360-CombatMetrics.html",
                Some("https://www.esoui.com/downloads/download1360".to_string()),
            ),
            //(
            //    "https://www.esoui.com/downloads/fileinfo.php?id=2817",
            //    Some("https://www.esoui.com/downloads/download2817".to_string()),
            //),
        ];

        for test in tests {
            let url = get_download_url(test.0);
            assert!(url == test.1, "Got value: {:?}", url);
        }
    }
}
