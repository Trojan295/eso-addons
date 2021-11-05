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

#[derive(Parser)]
pub struct AddCommand {
    addon_url: Option<String>,
    #[clap(
        short,
        long,
        about = "Indicate, if the addon is only a dependency for another addon"
    )]
    dependency: Option<bool>,
}

impl AddCommand {
    pub fn run(
        &mut self,
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

    pub fn get_entry(&mut self) -> Result<AddonEntry, Box<dyn Error>> {
        if self.addon_url.is_none() {
            self.ask_for_fields()
                .chain_err(&format!("failed to get parameters"))?;
        }

        let addon_url = self.addon_url.clone().ok_or("missing addon URL")?;
        let dependency = self.dependency.unwrap_or(false);

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
            name: addon_name,
            url: download_url,
            dependency: dependency,
        })
    }

    fn ask_for_fields(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut questions = vec![requestty::Question::input("addon_url")
            .message("URL of the addon on esoui.com")
            .build()];

        if self.dependency.is_none() {
            questions.push(
                requestty::Question::confirm("dependency")
                    .message("Is addon only a dependency?")
                    .default(false)
                    .build(),
            );
        }

        let answers = requestty::prompt(questions)?;

        if let Some(addon_url) = answers.get("addon_url") {
            self.addon_url = addon_url.as_string().map(|x| x.to_owned());
        };

        if let Some(dependency) = answers.get("dependency") {
            self.dependency = dependency.as_bool();
        };

        Ok(())
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
    let re = Regex::new(r"^https://.*esoui.com/downloads/info(\d+)-(.+)\.html$").unwrap();
    re.captures(addon_url).map(|captures| {
        format!(
            "https://www.esoui.com/downloads/download{}-{}.html",
            captures[1].to_owned(),
            captures[2].to_owned()
        )
    })
}
