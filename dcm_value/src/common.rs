use std::fmt::Display;
use std::str::FromStr;


/// Represents a person's name divided into several components such as family name, given name, middle name, prefix, and suffix.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersonName {
    /// The family name (also known as last name or surname) of the person.
    pub family_name: String,
    /// The given name (also known as first name) of the person.
    pub given_name: String,
    /// The middle name of the person.
    pub middle_name: String,
    /// Any prefix associated with the person's name, such as "Dr." or "Mr."
    pub prefix: String,
    /// Any suffix associated with the person's name, such as "Jr." or "III."
    pub suffix: String,
}

impl FromStr for PersonName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('^').map(|x| x.trim()).collect();
        let n = v.len();
        let mut name = PersonName::default();
        if n > 0 {
            name.family_name = v[0].to_string();
        }
        if n > 1 {
            name.given_name = v[1].to_string();
        }
        if n > 2 {
            name.middle_name = v[2].to_string();
        }
        if n > 3 {
            name.prefix = v[3].to_string();
        }
        if n > 4 {
            name.suffix = v[4].to_string();
        }
        Ok(name)
    }
}

impl Display for PersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{}^{}^{}^{}", self.family_name, self.given_name, self.middle_name, self.prefix, self.suffix)
    }
}

impl PersonName {
    pub fn is_empty(&self) -> bool {
        self.family_name.is_empty()
            && self.given_name.is_empty()
            && self.middle_name.is_empty()
            && self.prefix.is_empty()
            && self.suffix.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::PersonName;
    use std::str::FromStr;

    #[test]
    fn test_from_str_complete() {
        let input = "Smith^John^Doe^Mr.^Jr.";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert_eq!(name.prefix, "Mr.");
        assert_eq!(name.suffix, "Jr.");
    }

    #[test]
    fn test_from_str_partial1() {
        let input = "Smith^John";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert!(name.middle_name.is_empty());
        assert!(name.prefix.is_empty());
        assert!(name.suffix.is_empty());
    }

    #[test]
    fn test_from_str_partial2() {
        let input = "Smith^John^Doe";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert!(name.prefix.is_empty());
        assert!(name.suffix.is_empty());
    }

    #[test]
    fn test_from_str_partial3() {
        let input = "Smith^John^Doe^^Jr";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert!(name.prefix.is_empty());
        assert_eq!(name.suffix, "Jr");
    }

    #[test]
    fn test_from_str_empty() {
        let input = "";
        let name = PersonName::from_str(input).unwrap();
        assert!(name.family_name.is_empty());
        assert!(name.given_name.is_empty());
        assert!(name.middle_name.is_empty());
        assert!(name.prefix.is_empty());
        assert!(name.suffix.is_empty());
    }

    #[test]
    fn test_from_str_with_whitespace() {
        let input = " Smith ^ John ^ Doe ^ Mr. ^ Jr. ";
        let name = PersonName::from_str(input).unwrap();
        assert_eq!(name.family_name, "Smith");
        assert_eq!(name.given_name, "John");
        assert_eq!(name.middle_name, "Doe");
        assert_eq!(name.prefix, "Mr.");
        assert_eq!(name.suffix, "Jr.");
    }
}
