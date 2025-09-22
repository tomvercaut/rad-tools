use crate::tag::{Item, TagDictionary};
use regex::Regex;
use std::io::{Read, Write};
use std::str::FromStr;

const URL_DICOM_TAGS: &str =
    "https://raw.githubusercontent.com/DCMTK/dcmtk/master/dcmdata/data/dicom.dic";
pub(crate) static RE_WHITESPACE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\s+").unwrap());

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
    println!("Parsing DICOM tags");
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

pub fn create_rs_file<P>(p: P, dict: &TagDictionary) -> crate::Result<()>
where
    P: AsRef<std::path::Path>,
{
    println!("Creating a tag dictionary file");
    let tag_rs = include_str!("tag.rs");

    let mut split = tag_rs.split("#[cfg(test)]");
    let part1 = split.next().unwrap();
    let part2 = split.next().unwrap();

    let mut s = String::new();
    s += part1;

    s += "pub static TAG_DICTIONARY: std::sync::LazyLock<TagDictionary> = std::sync::LazyLock::new(|| {\n";
    s += "  let mut dict = TagDictionary::default();\n";

    for (tag, item) in dict.items.iter() {
        s.push_str(&format!(
            r#"
  dict.insert(
      Item {{
        tag: dicom_core::Tag(0x{:04X}, 0x{:04X}),
        vr: dicom_core::VR::{},
        name: "{}".to_string(),
        vm: "{}".to_string(),
        version: "{}".to_string()
      }}
  );"#,
            tag.0, tag.1, item.vr, item.name, item.vm, item.version
        ));
    }

    s += "\n  dict";
    s += "\n});\n";
    s += part2;

    let mut f = std::fs::File::create(p.as_ref())?;
    f.write_all(s.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_replace_tabs_spaces() {
        let line = "a  b\t c \td\t\te";
        let normalized = RE_WHITESPACE.replace_all(line, " ");
        assert_eq!(normalized, "a b c d e");
    }
    use super::*;
}
