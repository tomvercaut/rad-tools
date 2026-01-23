#[macro_export]
macro_rules! dicom_value_type {
    ($name:ident, $vr:ident, $value_type:ty) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct $name<const G: u16, const E: u16> {
            value: $value_type,
        }

        impl<const G: u16, const E: u16> Value<$value_type> for $name<G, E> {
            fn tag(&self) -> dicom_core::Tag {
                dicom_core::Tag(G, E)
            }

            fn vr(&self) -> dicom_core::VR {
                dicom_core::VR::$vr
            }

            fn vm(&self) -> VM {
                VM::Single
            }

            fn value(&self) -> &$value_type {
                &self.value
            }

            fn value_mut(&mut self) -> &mut $value_type {
                &mut self.value
            }
        }
    };
}

#[macro_export]
macro_rules! one_to_many_dicom_value_by_delim {
    ($name:ident, $names:ident, $delim:literal) => {
        impl<const G: u16, const E: u16> From<$name<G, E>> for $names<G, E> {
            fn from(v: $name<G, E>) -> Self {
                Self {
                    value: v
                        .value
                        .split($delim)
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .into(),
                }
            }
        }
    };
}
