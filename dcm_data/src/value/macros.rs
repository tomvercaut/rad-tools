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

        impl<const G: u16, const E: u16> std::ops::Deref for $name<G, E> {
            type Target = $value_type;
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl<const G: u16, const E: u16> std::ops::DerefMut for $name<G, E> {
            fn deref_mut(&mut self) -> &mut Self::Target {
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
        impl<const G: u16, const E: u16> $crate::value::ReadDicomValue<dicom_object::InMemDicomObject> for $name<G, E> {
            fn read_value(
                backend: &dicom_object::InMemDicomObject,
            ) -> Result<Self, $crate::io::DcmIOError> {
                match backend.element(dicom_core::Tag(G, E)) {
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
macro_rules! to_dicom_object_for_string {
    ($name: ident, $vr: ident) => {
        impl<const G: u16, const E: u16> $crate::WriteDicomValue<dicom_object::InMemDicomObject> for $name<G, E> {
            fn write_value(&self,
                obj: &mut dicom_object::InMemDicomObject,
            ) -> Result<(), $crate::io::DcmIOError> {
                let _ = obj.put(dicom_core::DataElement::new(
                    self.tag(),
                    self.vr(),
                    self.value().as_str(),
                ));
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! to_dicom_object_for_strings {
    ($name: ident, $vr: ident) => {
        impl<const G: u16, const E: u16> $crate::WriteDicomValue<dicom_object::InMemDicomObject> for $name<G, E> {
            fn write_value(&self,
                obj: &mut dicom_object::InMemDicomObject,
            ) -> Result<(), $crate::io::DcmIOError> {
                let sv = dicom_core::smallvec::SmallVec::from(self.value().clone());
                let p = dicom_core::value::PrimitiveValue::Strs(sv);
                let _ = obj.put(dicom_core::DataElement::new(
                    self.tag(),
                    self.vr(),
                    p,
                ));
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! from_dicom_object_for_strings {
    ($name: ident, $vr: ident, $delim:literal) => {
        impl<const G: u16, const E: u16> $crate::value::ReadDicomValue<dicom_object::InMemDicomObject> for $name<G, E> {
            fn read_value(
                backend: &dicom_object::InMemDicomObject,
            ) -> Result<Self, $crate::io::DcmIOError> {
                match backend.element(dicom_core::Tag(G, E)) {
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
macro_rules! from_dicom_object_for_number {
    ($name: ident, $vr: ident, $fncall: ident) => {
        impl<const G: u16, const E: u16> $crate::value::ReadDicomValue<dicom_object::InMemDicomObject> for $name<G, E> {
            fn read_value(
                backend: &dicom_object::InMemDicomObject,
            ) -> Result<Self, $crate::io::DcmIOError> {
                match backend.element(dicom_core::Tag(G, E)) {
                    Ok(elem) => {
                        if elem.vr() == dicom_core::VR::$vr {
                            let value = elem.$fncall()?;
                            Ok(Self{value})
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
    }
}

#[macro_export]
macro_rules! from_dicom_object_for_numbers {
    ($name: ident, $vr: ident, $fncall: ident) => {
        impl<const G: u16, const E: u16> $crate::value::ReadDicomValue<dicom_object::InMemDicomObject> for $name<G, E> {
            fn read_value(
                backend: &dicom_object::InMemDicomObject,
            ) -> Result<Self, $crate::io::DcmIOError> {
                match backend.element(dicom_core::Tag(G, E)) {
                    Ok(elem) => {
                        if elem.vr() == dicom_core::VR::$vr {
                            let value = Vec::from(elem.$fncall()?);
                            Ok(Self{value})
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
    }
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
