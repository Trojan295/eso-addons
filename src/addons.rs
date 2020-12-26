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

pub struct Addon {
    pub name: String,
    pub depends_on: Vec<String>,
}

pub struct Manager {
    addon_dir: PathBuf,
}

fn extract_dependency(dep: &str) -> String {
    let re = Regex::new(r"^(.+?)(([<=>]+)(.*))?$").unwrap();
    match re.captures(dep) {
        None => dep.to_owned(),
        Some(captures) => captures[1].to_owned(),
    }
}

impl Manager {
    pub fn new(addon_dir: &str) -> Manager {
        let path = PathBuf::from(addon_dir);

        Manager { addon_dir: path }
    }

    pub fn get_addons(&self) -> Result<Vec<Addon>, Box<dyn Error>> {
        let read_dir = fs::read_dir(&self.addon_dir)
            .chain_err(&format!("while listing addon dir {:?}", self.addon_dir))?;

        let mut addons = vec![];

        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let addon = self.read_addon(&path)?;
            addons.push(addon);
        }

        Ok(addons)
    }

    fn read_addon(&self, path: &Path) -> Result<Addon, Box<dyn Error>> {
        let addon_name = path
            .file_name()
            .ok_or(simple_error!("cannot get filename"))?;

        let addon_name = addon_name
            .to_str()
            .ok_or(simple_error!("failed to get addon name"))?;

        let txt_filename = PathBuf::from(format!("{}.txt", addon_name));

        let mut txt_file = path.to_owned();
        txt_file.push(txt_filename);
        let file = File::open(txt_file)?;

        let re = Regex::new(r"## (.*): (.*)").unwrap();

        let mut addon = Addon {
            name: addon_name.to_owned(),
            depends_on: vec![],
        };

        let lines = io::BufReader::new(file).lines();
        for line in lines {
            match line {
                Ok(line) => {
                    if line.contains("DependsOn") {
                        let depends_on = match re.captures(&line) {
                            Some(ref captures) => captures[2]
                                .split(" ")
                                .map(|s| s.to_owned())
                                .into_iter()
                                .map(|s| extract_dependency(&s).to_owned())
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
        let addon_name = addon_name.name();
        addon_path.push(addon_name);

        let addon = self
            .read_addon(&addon_path)
            .chain_err("while reading addon")?;

        Ok(addon)
    }
}

fn get_cdn_download_link(dom: &RcDom) -> Option<String> {
    let node = find_first_in_node(&dom.document, &|node: &Rc<Node>| match &node.data {
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

fn find_first_in_node<T>(node: &Rc<Node>, f: &dyn Fn(&Rc<Node>) -> Option<T>) -> Option<T> {
    match f(node) {
        Some(x) => Some(x),
        None => {
            for child in node.children.borrow().iter() {
                match find_first_in_node(child, f) {
                    Some(x) => return Some(x),
                    None => {}
                }
            }

            None
        }
    }
}
