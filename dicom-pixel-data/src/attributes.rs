use crate::ConvertError::{UnknownPixelRepresentation, UnknownPlanarConfiguration};

#[derive(thiserror::Error, Debug)]
pub enum ConvertError {
    #[error("Unknown value [{0}] to convert to a PlanarConfiguration")]
    UnknownPlanarConfiguration(u16),
    #[error("Unknown value [{0}] to convert to a PlanarConfiguration")]
    UnknownPixelRepresentation(u16),
}

/// Enum representing the photometric interpretation of an image.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhotometricInterpretation {
    Monochrome1,
    Monochrome2,
    PaletteColor,
    Rgb,
    YbrFull,
    YbrFull422,
    YbrPartial420,
    YbrIct,
    YbrRct,
    Other(String),
}

impl PhotometricInterpretation {
    pub fn is_monochrome(&self) -> bool {
        match self {
            PhotometricInterpretation::Monochrome1 => true,
            PhotometricInterpretation::Monochrome2 => true,
            _ => false,
        }
    }
}

impl From<&str> for PhotometricInterpretation {
    fn from(value: &str) -> Self {
        match value {
            "MONOCHROME1" => PhotometricInterpretation::Monochrome1,
            "MONOCHROME2" => PhotometricInterpretation::Monochrome2,
            "PALETTE COLOR" => PhotometricInterpretation::PaletteColor,
            "RGB" => PhotometricInterpretation::Rgb,
            "YBR_FULL" => PhotometricInterpretation::YbrFull,
            "YBR_FULL_422" => PhotometricInterpretation::YbrFull422,
            "YBR_PARTIAL_420" => PhotometricInterpretation::YbrPartial420,
            "YBR_ICT" => PhotometricInterpretation::YbrIct,
            "YBR_RCT" => PhotometricInterpretation::YbrRct,
            _ => PhotometricInterpretation::Other(value.to_string()),
        }
    }
}

impl From<String> for PhotometricInterpretation {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl AsRef<str> for PhotometricInterpretation {
    fn as_ref(&self) -> &str {
        match self {
            PhotometricInterpretation::Monochrome1 => "MONOCHROME1",
            PhotometricInterpretation::Monochrome2 => "MONOCHROME2",
            PhotometricInterpretation::PaletteColor => "PALETTE_COLOR",
            PhotometricInterpretation::Rgb => "RGB",
            PhotometricInterpretation::YbrFull => "YBR_FULL",
            PhotometricInterpretation::YbrFull422 => "YBR_FULL_422",
            PhotometricInterpretation::YbrPartial420 => "YBR_PARTIAL_420",
            PhotometricInterpretation::YbrIct => "YBR_ICT",
            PhotometricInterpretation::YbrRct => "YBR_RCT",
            PhotometricInterpretation::Other(s) => s,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanarConfiguration {
    ColorByPixel,
    ColorByPlane,
}

impl TryFrom<u16> for crate::PlanarConfiguration {
    type Error = ConvertError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ColorByPixel),
            1 => Ok(Self::ColorByPlane),
            _ => Err(UnknownPlanarConfiguration(value)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PixelRepresentation {
    UnsignedInteger,
    TwosComplement,
}

impl TryFrom<u16> for PixelRepresentation {
    type Error = ConvertError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UnsignedInteger),
            1 => Ok(Self::TwosComplement),
            _ => Err(UnknownPixelRepresentation(value)),
        }
    }
}

impl std::fmt::Display for PixelRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PixelRepresentation::UnsignedInteger => write!(f, "unsigned integer"),
            PixelRepresentation::TwosComplement => write!(f, "two's complement"),
        }
    }
}
