use crate::errors::{Error, Result};
use crate::htmlparser;

use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use tempfile::tempfile;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Addon {
    pub name: String,
    pub depends_on: Vec<String>,
}

pub struct AddonList {
    pub addons: Vec<Addon>,
    pub errors: Vec<Error>,
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

    pub fn get_addons(&self) -> Result<AddonList> {
        let mut addon_list = AddonList {
            addons: vec![],
            errors: vec![],
        };

        if let Err(err) = fs::metadata(&self.addon_dir) {
            return Err(Error::CannotOpenAddonDirectory(
                self.addon_dir.clone(),
                Box::new(err),
            ));
        }

        for entry in WalkDir::new(&self.addon_dir) {
            let entry_dir = entry.map_err(|err| {
                Error::CannotOpenAddonDirectory(self.addon_dir.clone(), Box::new(err))
            })?;
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
                Err(err) => addon_list.errors.push(err),
            }
        }

        Ok(addon_list)
    }

    pub fn get_addon(&self, name: &str) -> Result<Option<Addon>> {
        let addon_list = self.get_addons()?;
        let found = addon_list.addons.into_iter().find(|x| x.name == name);
        Ok(found)
    }

    fn read_addon(&self, path: &Path) -> Result<Addon> {
        let addon_name = path.file_name().unwrap().to_str().unwrap();

        let file = self
            .open_addon_metadata_file(path, addon_name)
            .map_err(|err| Error::CannotReadAddon(addon_name.to_owned(), Box::new(err)))?;
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

    pub fn delete_addon(&self, addon: &Addon) -> Result<()> {
        let mut addon_path = self.addon_dir.to_owned();
        addon_path.push(&addon.name);

        fs::remove_dir_all(addon_path)
            .map_err(|err| Error::CannotRemoveAddon(addon.name.to_owned(), Box::new(err)))?;
        Ok(())
    }

    pub fn download_addon(&self, url: &str) -> Result<Addon> {
        let download_link = htmlparser::get_document(url).map(htmlparser::get_cdn_download_link)?;
        let download_link = download_link.ok_or(Error::CannotDownloadAddon(
            url.to_owned(),
            "CDN link missing".into(),
        ))?;

        let mut response = reqwest::blocking::get(&download_link)
            .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;

        let mut tmpfile = tempfile()?;
        response
            .copy_to(&mut tmpfile)
            .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;
        let mut archive = zip::ZipArchive::new(tmpfile)
            .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;
            let outpath = match file.enclosed_name() {
                Some(path) => {
                    let mut p = self.addon_dir.clone();
                    p.push(path);
                    p
                }

                None => continue,
            };

            if (file.name()).ends_with('/') {
                fs::create_dir_all(&outpath)
                    .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).map_err(|err| {
                            Error::CannotDownloadAddon(url.to_owned(), Box::new(err))
                        })?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)
                    .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;
            }
        }

        let mut addon_path = self.addon_dir.to_owned();
        let addon_name = archive
            .by_index(0)
            .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;
        let addon_name = get_root_dir(&addon_name.mangled_name());
        addon_path.push(addon_name);

        let addon = self.read_addon(&addon_path)?;

        Ok(addon)
    }

    fn open_addon_metadata_file(&self, path: &Path, addon_name: &str) -> Result<File> {
        let mut filepath = path.to_owned();
        let mut filepath_lowercase = path.to_owned();

        let filename = PathBuf::from(format!("{}.txt", addon_name));
        let filename_lowercase = PathBuf::from(format!("{}.txt", addon_name.to_lowercase()));

        filepath.push(filename);
        filepath_lowercase.push(filename_lowercase);

        if filepath.exists() {
            File::open(&filepath).map_err(|err| Error::Other(Box::new(err)))
        } else if filepath_lowercase.exists() {
            File::open(&filepath_lowercase).map_err(|err| Error::Other(Box::new(err)))
        } else {
            Err(Error::Other("missing addon metadata file".into()))
        }
    }
}

pub fn get_download_url(addon_url: &str) -> Option<String> {
    let fns: Vec<fn(&str) -> Option<String>> = vec![
        |url: &str| {
            let re = Regex::new(r"^https://.*esoui\.com/downloads/info(\d+)-(.+)$").unwrap();
            re.captures(url).map(|captures| captures[1].to_owned())
        },
        |url: &str| {
            let re =
                Regex::new(r"^https://.+esoui\.com/downloads/fileinfo\.php\?id=(\d+)$").unwrap();
            re.captures(url).map(|captures| captures[1].to_owned())
        },
    ];

    for f in fns {
        let url = f(addon_url);
        if let Some(id) = url {
            return Some(format!("https://www.esoui.com/downloads/download{}", id));
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
            (
                "https://www.esoui.com/downloads/fileinfo.php?id=2817",
                Some("https://www.esoui.com/downloads/download2817".to_string()),
            ),
        ];

        for test in tests {
            let url = get_download_url(test.0);
            assert!(url == test.1, "Got value: {:?}", url);
        }
    }
}
