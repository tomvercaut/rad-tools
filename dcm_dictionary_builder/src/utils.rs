use std::io::Read;
use std::str::FromStr;
use std::sync::LazyLock;
use regex::Regex;
use crate::tag::{Item, TagDictionary};

const URL_DICOM_TAGS: &str =
    "https://raw.githubusercontent.com/DCMTK/dcmtk/master/dcmdata/data/dicom.dic";
pub(crate) static RE_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

fn download() -> crate::Result<String> {
    println!("Downloading DICOM tags");
    let mut res = ureq::get(URL_DICOM_TAGS).call()?;
    let len: usize = res
        .headers()
        .get("Content-Length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    let mut s = String::with_capacity(len);
    res.body_mut().as_reader().read_to_string(&mut s)?;
    Ok(s)
}

pub fn create_tag_dictionary() -> crate::Result<TagDictionary> {
    let s = download()?;
    let mut dict = TagDictionary::default();
    for line in s.split('\n') {
        if line.starts_with("(") {
            let normalized = RE_WHITESPACE.replace_all(line, " ");
            if let Ok(item) = Item::from_str(&normalized) {
                dict.insert(item);
            }
        }
    }

    Ok(dict)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_replace_tabs_spaces() {
        let line = "a  b\t c \td\t\te";
        let normalized = RE_WHITESPACE.replace_all(line, " ");
        assert_eq!(normalized, "a b c d e");
    }   use super::*;
}
