use scraper::{Html, Selector};
use std::io::Read;

use crate::errors::{Error, Result};

pub fn get_addon_name(doc: Html) -> Option<String> {
    let selector = Selector::parse("meta").unwrap();
    let mut a = doc.select(&selector);

    a.find(|node| node.value().attr("property").unwrap_or("") == "og:title")
        .map(|node| node.value().attr("content").unwrap().to_owned())
}

pub fn get_cdn_download_link(doc: Html) -> Option<String> {
    let selector = Selector::parse("a").unwrap();
    let mut a = doc.select(&selector);

    let link_node = a.find(|node| {
        node.value()
            .attr("href")
            .unwrap_or("")
            .starts_with("https://cdn.esoui.com")
    });

    link_node.map(|node| node.value().attr("href").unwrap().to_owned())
}

pub fn get_document(url: &str) -> Result<Html> {
    let mut response = reqwest::blocking::get(url)
        .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;

    let mut buf = String::new();

    response
        .read_to_string(&mut buf)
        .map_err(|err| Error::CannotDownloadAddon(url.to_owned(), Box::new(err)))?;

    let document = Html::parse_document(&buf);

    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Selector;

    #[test]
    fn test_get_document() {
        let url = "https://example.com/";

        let doc = get_document(url);
        assert!(doc.is_ok());

        let doc = doc.unwrap();

        let selector = Selector::parse("title").unwrap();
        let title = doc.select(&selector).next().unwrap();

        assert_eq!(title.inner_html(), "Example Domain");
    }
}
