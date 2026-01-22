#[macro_export]
macro_rules! dicom_value_type {
    ($name:ident, $names:ident, $vr:ident, $value_type:ty) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
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

        #[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
        pub struct $names<const G: u16, const E: u16> {
            value: Vec<$value_type>,
        }

        impl<const G: u16, const E: u16> Value<Vec<$value_type>> for $names<G, E> {
            fn tag(&self) -> dicom_core::Tag {
                dicom_core::Tag(G, E)
            }

            fn vr(&self) -> dicom_core::VR {
                dicom_core::VR::$vr
            }

            fn vm(&self) -> VM {
                VM::Multiple
            }

            fn value(&self) -> &Vec<$value_type> {
                &self.value
            }

            fn value_mut(&mut self) -> &mut Vec<$value_type> {
                &mut self.value
            }
        }
    };
}
