#[macro_export]
macro_rules! dicom_value_type {
    ($name:ident, $vr:ident, $value_type:ty) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct $name<const G: u16, const E: u16> {
            value: $value_type,
        }

        impl<const G: u16, const E: u16> $crate::value::Value<$value_type> for $name<G, E> {
            fn tag(&self) -> dicom_core::Tag {
                dicom_core::Tag(G, E)
            }

            fn vr(&self) -> dicom_core::VR {
                dicom_core::VR::$vr
            }

            fn vm(&self) -> crate::value::VM {
                crate::value::VM::Single
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

#[macro_export]
macro_rules! from_dicom_object_for_string {
    ($name: ident, $vr: ident) => {
        impl<const G: u16, const E: u16> $crate::value::FromDicomObject for $name<G, E> {
            fn from_object(
                obj: &dicom_object::InMemDicomObject,
            ) -> Result<Self, $crate::io::DcmIOError> {
                match obj.element(dicom_core::Tag(G, E)) {
                    Ok(elem) => {
                        if elem.vr() == dicom_core::VR::$vr {
                            let value = elem.to_str()?.to_string();
                            Ok(Self { value })
                        } else {
                            Err($crate::io::DcmIOError::InvalidVRMatch(
                                dicom_core::VR::$vr,
                                elem.vr(),
                            ))
                        }
                    }
                    Err(e) => Err($crate::io::DcmIOError::from(e)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! from_dicom_object_for_strings {
    ($name: ident, $vr: ident, $delim:literal) => {
        impl<const G: u16, const E: u16> $crate::value::FromDicomObject for $name<G, E> {
            fn from_object(
                obj: &dicom_object::InMemDicomObject,
            ) -> Result<Self, $crate::io::DcmIOError> {
                match obj.element(dicom_core::Tag(G, E)) {
                    Ok(elem) => {
                        if elem.vr() == dicom_core::VR::$vr {
                            let value = elem.to_str()?.to_string();
                            let value = value
                                .split($delim)
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>();
                            Ok(Self { value })
                        } else {
                            Err($crate::io::DcmIOError::InvalidVRMatch(
                                dicom_core::VR::$vr,
                                elem.vr(),
                            ))
                        }
                    }
                    Err(e) => Err($crate::io::DcmIOError::from(e)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! dicom_value_from_same_type {
    ($name:ident, $value_type:ty) => {
        impl<const G: u16, const E: u16> std::convert::From<$value_type> for $name<G, E> {
            fn from(v: $value_type) -> Self {
                Self { value: v }
            }
        }
    }
}

#[macro_export]
macro_rules! dicom_value_from_str {
    ($name:ident) => {
        impl<const G: u16, const E: u16> std::str::FromStr for $name<G, E> {
            type Err = &'static str;
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                Ok(Self { value: v.to_string() })
            }
        }
    }
}
