use crate::PersonName;
use std::str::FromStr;

crate::dicom_value_type!(PN, PN, PersonName);
crate::dicom_value_type!(PNs, PN, Vec<PersonName>);

impl<const G: u16, const E: u16> FromStr for PN<G, E> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = PersonName::from_str(s)?;
        Ok(PN { value })
    }
}
