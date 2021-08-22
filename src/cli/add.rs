use eso_addons::{
    addons::Manager,
    config::{self, AddonEntry, Config},
    errors::ErrorChain,
    htmlparser,
};
use html5ever::{tendril::TendrilSink, tree_builder::TreeBuilderOpts, ParseOpts};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use regex::Regex;
use std::{error::Error, path::Path, rc::Rc};

#[derive(Clap)]
pub struct AddCommand {
    addon_url: Option<String>,
    #[clap(
        short,
        long,
        about = "Indicate, if the addon is only a dependency for another addon"
    )]
    dependency: bool,
}

impl AddCommand {
    pub fn run(
        &self,
        cfg: &mut Config,
        config_filepath: &Path,
        addon_manager: &Manager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut entry = self.get_entry()?;

        if cfg.addons.iter().find(|el| el.url == entry.url).is_some() {
            println!("Addon {} is already installed", &entry.name);
            return Ok(());
        }

        let installed = addon_manager
            .download_addon(&entry.url.clone().unwrap())
            .chain_err(&format!("while downloading {}", &entry.name))?;

        if entry.name != installed.name {
            entry.name = installed.name;
        }

        cfg.addons.push(entry.clone());

        config::save_config(config_filepath, &cfg)?;

        println!("🎊 Installed {}!", &entry.name);

        Ok(())
    }

    pub fn get_entry(&self) -> Result<AddonEntry, Box<dyn Error>> {
        let addon_url = match &self.addon_url {
            Some(url) => url.clone(),
            None => self.ask_for_addon_url()?,
        };

        let mut response = reqwest::blocking::get(&addon_url)?;

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

        let addon_name = get_addon_name(&dom).ok_or(simple_error!("failed to get addon name"))?;
        let download_url = get_download_url(&addon_url);

        Ok(AddonEntry {
            name: addon_name, // TODO: read correct addon name
            url: download_url,
            dependency: false,
        })
    }

    fn ask_for_addon_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        let question = requestty::Question::input("addon_url")
            .message("URL of the addon on esoui.com")
            .build();

        let answer = requestty::prompt_one(question)?;
        answer
            .as_string()
            .map(|x| x.to_owned())
            .ok_or(Box::new(simple_error!("URL not provided")))
    }
}

fn get_addon_name(dom: &RcDom) -> Option<String> {
    htmlparser::find_first_in_node(&dom.document, &|node: &Rc<Node>| match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            if &name.local == "meta" {
                for attr in attrs.borrow().iter() {
                    if &attr.name.local == "property" {
                        if attr.value.to_string().eq("og:title") {
                            return attrs
                                .borrow()
                                .iter()
                                .find(|x| &x.name.local == "content")
                                .map(|x| x.value.to_string());
                        }
                    }
                }
            }
            None
        }
        _ => None,
    })
}

fn get_download_url(addon_url: &str) -> Option<String> {
    let re = Regex::new(r"^https://www.esoui.com/downloads/info(\d+)-(.+)\.html$").unwrap();
    re.captures(addon_url).map(|captures| {
        format!(
            "https://www.esoui.com/downloads/download{}-{}.html",
            captures[1].to_owned(),
            captures[2].to_owned()
        )
    })
}
