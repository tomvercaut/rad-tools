#[derive(Clone, Debug)]
pub struct Item {
    pub tag: dicom_core::Tag,
    pub vr: dicom_core::VR,
    pub name: String,
    pub vm: String,
    pub version: String,
}

impl std::str::FromStr for Item {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::with_capacity(5);
        for t in s.split(" ") {
            v.push(t);
        }
        if v.len() != 5 {
            return Err(crate::Error::DictionaryTagLineFormatInvalid);
        }
        // Parse the tag
        let tag = dicom_core::Tag::from_str(v[0])?;
        // Parse the VR
        let vr = dicom_core::VR::from_str(v[1]).map_err(|_| crate::Error::ParseVrFailed)?;
        Ok(Self {
            tag,
            vr,
            name: v[2].to_string(),
            vm: v[3].to_string(),
            version: v[4].to_string(),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct TagDictionary {
    items: std::collections::HashMap<dicom_core::Tag, Item>,
    named_items: std::collections::HashMap<String, dicom_core::Tag>,
}

impl TagDictionary {
    pub fn insert(&mut self, item: Item) {
        let exists = self.items.contains_key(&item.tag);
        if !exists {
            let tag = item.tag;
            let name = item.name.clone();
            self.items.insert(item.tag, item);
            self.named_items.insert(name, tag);
        }
    }

    pub fn by_tag(&self, tag: dicom_core::Tag) -> Option<&Item> {
        self.items.get(&tag)
    }

    pub fn by_name(&self, name: &str) -> Option<&Item> {
        let tag = self.named_items.get(name)?;
        self.by_tag(*tag)
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.named_items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_valid_item_from_str() {
        let line = "(3008,0050) SQ TreatmentSummaryCalculatedDoseReferenceSequence 1 DICOM";
        let item = Item::from_str(line).unwrap();
        assert_eq!(item.tag, dicom_core::Tag(0x3008, 0x0050));
        assert_eq!(item.vr, dicom_core::VR::SQ);
        assert_eq!(item.name, "TreatmentSummaryCalculatedDoseReferenceSequence");
        assert_eq!(item.vm, "1");
        assert_eq!(item.version, "DICOM");
    }

    #[test]
    fn test_invalid_tag_format() {
        let line = "(5000-50FF,0112)	US	RETIRED_CoordinateStartValue	1-n	DICOM/retired";
        assert!(Item::from_str(line).is_err());
    }

    #[test]
    fn test_invalid_item_format() {
        let line = "(3008,0050) SQ TreatmentSummaryCalculatedDoseReferenceSequence";
        assert!(matches!(
            Item::from_str(line),
            Err(crate::Error::DictionaryTagLineFormatInvalid)
        ));
    }

    #[test]
    fn test_invalid_vr() {
        let line = "(3008,0050) XX TreatmentSummaryCalculatedDoseReferenceSequence 1 DICOM";
        assert!(matches!(
            Item::from_str(line),
            Err(crate::Error::ParseVrFailed)
        ));
    }

    #[test]
    fn test_tag_dictionary_insert_and_get_by_tag() {
        let mut dict = TagDictionary::default();
        let item = Item::from_str("(0008,0060) CS Modality 1 DICOM").unwrap();
        dict.insert(item.clone());
        assert_eq!(
            dict.by_tag(dicom_core::Tag(0x0008, 0x0060)).unwrap().name,
            "Modality"
        );
    }

    #[test]
    fn test_tag_dictionary_insert_and_get_by_name() {
        let mut dict = TagDictionary::default();
        let item = Item::from_str("(0008,0060) CS Modality 1 DICOM").unwrap();
        dict.insert(item.clone());
        assert_eq!(
            dict.by_name("Modality").unwrap().tag,
            dicom_core::Tag(0x0008, 0x0060)
        );
    }

    #[test]
    fn test_tag_dictionary_duplicate_insert() {
        let mut dict = TagDictionary::default();
        let item1 = Item::from_str("(0008,0060) CS Modality 1 DICOM").unwrap();
        let item2 = Item::from_str("(0008,0060) CS NewModality 1 DICOM").unwrap();
        dict.insert(item1.clone());
        dict.insert(item2);
        assert_eq!(
            dict.by_tag(dicom_core::Tag(0x0008, 0x0060)).unwrap().name,
            "Modality"
        );
    }

    #[test]
    fn test_tag_dictionary_nonexistent_items() {
        let dict = TagDictionary::default();
        assert!(dict.by_tag(dicom_core::Tag(0x0008, 0x0060)).is_none());
        assert!(dict.by_name("Modality").is_none());
    }
}
