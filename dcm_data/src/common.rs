use std::str::FromStr;

/// Represents a Service-Object Pair (SOP) in the DICOM standard.
///
/// A Service-Object Pair (SOP) is a combination of a DICOM Service Elements (DIMSE)
/// (such as Store, Get, Find, etc.) and a specific type of
/// DICOM Information Object Definition (such as an CT image, MR image, etc.).
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use dcm_data::Sop;
///
/// let sop = Sop {
///     class_uid: "1.2.840.10008.1.1".to_string(),
///     instance_uid: "1.2.752.243.1.1.20220722130644359.1060.62784".to_string(),
/// };
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sop {
    /// The unique identifier (UID) for the DICOM service class.
    /// This UID defines the type of service being requested or provided,
    /// such as Verification, Storage, etc.
    pub class_uid: String,
    /// The unique identifier (UID) for the DICOM instance.
    /// This UID uniquely identifies a particular instance of the service object pair.
    pub instance_uid: String,
}

/// Represents a person's name divided into several components such as family name, given name, middle name, prefix, and suffix.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

impl PersonName {
    pub fn is_empty(&self) -> bool {
        self.family_name.is_empty()
            && self.given_name.is_empty()
            && self.middle_name.is_empty()
            && self.prefix.is_empty()
            && self.suffix.is_empty()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RotationDirectionError {
    #[error("Invalid rotation direction: {0}")]
    InvalidRotationDirection(String),
}

/// Represents the direction of rotation of the source with respect to the principal axis of the equipment.
///
/// This enum is used to specify the direction in which the source, such as an imaging device,
/// is rotated relative to the principal axis of the equipment. The options include no rotation,
/// clockwise rotation, and counter-clockwise rotation.
///
/// # Variants
///
/// - `NONE`: No rotation is applied.
/// - `CW`: Clockwise rotation relative to the principal axis.
/// - `CCW`: Counter-clockwise rotation relative to the principal axis.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use dcm_data::RotationDirection;
///
/// let direction = RotationDirection::from_str("CW").unwrap();
/// assert_eq!(direction, RotationDirection::CW);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RotationDirection {
    #[default]
    NONE,
    CW,
    CCW,
}

impl FromStr for RotationDirection {
    type Err = RotationDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "CW" || s == "cw" {
            Ok(RotationDirection::CW)
        } else if s == "CCW" || s == "ccw" {
            Ok(RotationDirection::CCW)
        } else if s == "None" || s == "none" || s == "NONE" {
            Ok(RotationDirection::NONE)
        } else {
            Err(RotationDirectionError::InvalidRotationDirection(s.into()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PatientPositionError {
    #[error("Invalid patient position: {0}")]
    InvalidPatientPosition(String),
}

/// Represents patient position in DICOM format.
///
/// This enum specifies the possible positions of a patient during an imaging study.
/// The positions are described using acronyms where:
/// - `HFP`: Head First-Prone
/// - `HFS`: Head First-Supine
/// - `HFDR`: Head First-Decubitus Right
/// - `HFDL`: Head First-Decubitus Left
/// - `FFDR`: Feet First-Decubitus Right
/// - `FFDL`: Feet First-Decubitus Left
/// - `FFP`: Feet First-Prone
/// - `FFS`: Feet First-Supine
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PatientPosition {
    #[default]
    NONE, // Undefined default
    HFP,  // Patient is positioned head first-prone
    HFS,  // Patient is positioned head first-supine
    HFDR, // Patient is positioned head first-decubitus right
    HFDL, // Patient is positioned head first-decubitus left
    FFDR, // Patient is positioned feet first-decubitus right
    FFDL, // Patient is positioned feet first-decubitus left
    FFP,  // Patient is positioned feet first-prone
    FFS,  // Patient is positioned feet first-supine
}

impl FromStr for PatientPosition {
    type Err = PatientPositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HFP" => Ok(PatientPosition::HFP),
            "HFS" => Ok(PatientPosition::HFS),
            "HFDR" => Ok(PatientPosition::HFDR),
            "HFDL" => Ok(PatientPosition::HFDL),
            "FFDR" => Ok(PatientPosition::FFDR),
            "FFDL" => Ok(PatientPosition::FFDL),
            "FFP" => Ok(PatientPosition::FFP),
            "FFS" => Ok(PatientPosition::FFS),
            _ => Err(PatientPositionError::InvalidPatientPosition(s.to_string())),
        }
    }
}

/// Item in a Code Sequence
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeItem {
    /// Identifier of a Coded Entry in a Coding Scheme.
    pub code_value: Option<String>,
    /// Identifier of the coding scheme in which the Coded Entry is defined.
    pub coding_scheme_designator: Option<String>,
    /// Identifier of the version of the coding scheme
    pub coding_scheme_version: Option<String>,
    /// Meaning of the Coded Entry
    pub code_meaning: String,
}

#[derive(thiserror::Error, Debug)]
pub enum PhotometricInterpretationError {
    #[error("Invalid photometric interpretation: {0}")]
    InvalidPhotometricInterpretation(String),
}

/// Represents the photometric interpretation of pixel data in a DICOM image.
///
/// This enum specifies the possible photometric interpretations which describe how pixel values
/// are intended to be interpreted. The variants are:
///
/// - `MONOCHROME1`: Monochrome 1 (pixel values range from white to black)
/// - `MONOCHROME2`: Monochrome 2 (pixel values range from black to white)
/// - `PALETTE_COLOR`: Palette Color (pixel values are indexes into a color lookup table)
/// - `RGB`: RGB (Red, Green, Blue color model)
/// - `YBR_FULL`: YBR (Luminance, Blue Difference, Red Difference without chroma subsampling)
/// - `YBR_FULL_422`: YBR 422 (Luminance, Blue Difference, Red Difference with 4:2:2 chroma subsampling)
/// - `YBR_PARTIAL_422`: YBR Partial 422 (4:2:2 chroma subsampling with different value ranges)
/// - `YBR_PARTIAL_420`: YBR Partial 420 (4:2:0 chroma subsampling)
/// - `YBR_ICT`: YBR ICT (Luminance, Chrominance Inter-component Transforms)
/// - `YBR_RCT`: YBR RCT (Luminance, Chrominance Reversible Component Transform)
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub enum PhotometricInterpretation {
    MONOCHROME1, // Monochrome 1
    #[default]
    MONOCHROME2, // Monochrome 2
    PALETTE_COLOR, // Palette Color
    RGB,         // RGB
    YBR_FULL,    // YBR_FULL
    YBR_FULL_422, // YBR_FULL_422
    YBR_PARTIAL_422, // YBR_PARTIAL_422
    YBR_PARTIAL_420, // YBR_PARTIAL_420
    YBR_ICT,     // YBR_ICT
    YBR_RCT,     // YBR_RCT
}

impl FromStr for PhotometricInterpretation {
    type Err = PhotometricInterpretationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MONOCHROME1" => Ok(PhotometricInterpretation::MONOCHROME1),
            "MONOCHROME2" => Ok(PhotometricInterpretation::MONOCHROME2),
            "PALETTE_COLOR" => Ok(PhotometricInterpretation::PALETTE_COLOR),
            "RGB" => Ok(PhotometricInterpretation::RGB),
            "YBR_FULL" => Ok(PhotometricInterpretation::YBR_FULL),
            "YBR_FULL_422" => Ok(PhotometricInterpretation::YBR_FULL_422),
            "YBR_PARTIAL_422" => Ok(PhotometricInterpretation::YBR_PARTIAL_422),
            "YBR_PARTIAL_420" => Ok(PhotometricInterpretation::YBR_PARTIAL_420),
            "YBR_ICT" => Ok(PhotometricInterpretation::YBR_ICT),
            "YBR_RCT" => Ok(PhotometricInterpretation::YBR_RCT),
            _ => {
                Err(PhotometricInterpretationError::InvalidPhotometricInterpretation(s.to_string()))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PixelRepresentationError {
    #[error("Invalid pixel representation: {0}")]
    InvalidPixelRepresentation(String),
}

/// Data representation of pixel samples.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum PixelRepresentation {
    #[default]
    Unsigned, // Unsigned integer
    Signed, // Signed integer (2's complement)
}

impl FromStr for PixelRepresentation {
    type Err = PixelRepresentationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(PixelRepresentation::Unsigned),
            "1" => Ok(PixelRepresentation::Signed),
            _ => Err(PixelRepresentationError::InvalidPixelRepresentation(
                s.to_string(),
            )),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RescaleTypeError {
    #[error("Invalid rescale type: {0}")]
    InvalidRescaleType(String),
}

/// Output unit of the rescale slope (0028,1053) and rescale intercept (0028,1052).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub enum RescaleType {
    #[default]
    US, // Unspecified
    OD,     // Optical Density
    HU,     // Hounsfield Units
    MGML,   // Milligram per Milliliter
    Z_EFF,  // Effective Atomic Number
    ED,     // Electron Density
    EDW,    // Electron Density Weighted
    HU_MOD, // Modified Hounsfield Units
    PCT,    // Percent
}

impl FromStr for RescaleType {
    type Err = RescaleTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "US" => Ok(RescaleType::US),
            "OD" => Ok(RescaleType::OD),
            "HU" => Ok(RescaleType::HU),
            "MGML" => Ok(RescaleType::MGML),
            "Z_EFF" => Ok(RescaleType::Z_EFF),
            "ED" => Ok(RescaleType::ED),
            "EDW" => Ok(RescaleType::EDW),
            "HU_MOD" => Ok(RescaleType::HU_MOD),
            "PCT" => Ok(RescaleType::PCT),
            _ => Err(RescaleTypeError::InvalidRescaleType(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ApprovalStatus {
    APPROVED,
    #[default]
    UNAPPROVED,
    REJECTED,
}

impl FromStr for ApprovalStatus {
    type Err = ApprovalStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "APPROVED" => Ok(ApprovalStatus::APPROVED),
            "UNAPPROVED" => Ok(ApprovalStatus::UNAPPROVED),
            "REJECTED" => Ok(ApprovalStatus::REJECTED),
            _ => Err(ApprovalStatusError::InvalidApprovalStatus(s.into())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ModalityError {
    #[error("Invalid DICOM modality: {0}")]
    InvalidDicomModality(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Modality {
    AR,    //  Autorefraction
    AS,    //  Angioscopy
    ASMT,  //  Content Assessment Results
    AU,    //  Audio
    BDUS,  //  Bone Densitometry (ultrasound)
    BI,    //  Biomagnetic imaging
    BMD,   //  Bone Densitometry (X-Ray)
    CD,    //  Color flow Doppler
    CF,    //  Cinefluorography
    CP,    //  Colposcopy
    CR,    //  Computed Radiography
    CS,    //  Cystoscopy
    CT,    //  Computed Tomography
    DD,    //  Duplex Doppler
    DF,    //  Digital fluoroscopy
    DG,    //  Diaphanography
    DM,    //  Digital microscopy
    DOC,   //  Document
    DS,    //  Digital Subtraction Angiography
    DX,    //  Digital Radiography
    EC,    //  Echocardiography
    ECG,   //  Electrocardiography
    EPS,   //  Cardiac Electrophysiology
    ES,    //  Endoscopy
    FA,    //  Fluorescein angiography
    FID,   //  Fiducials
    FS,    //  Fundoscopy
    GM,    // General Microscopy
    HC,    //  Hard Copy
    HD,    //  Hemodynamic Waveform
    IO,    //  Intra-Oral Radiography
    IOL,   //  Intraocular Lens Data
    IVOCT, //  Intravascular Optical Coherence Tomography
    IVUS,  //  Intravascular Ultrasound
    KER,   //  Keratometry
    KO,    //  Key Object Selection
    LEN,   //  Lensometry
    LP,    //  Laparoscopy
    LS,    //  Laser surface scan
    MA,    //  Magnetic resonance angiography
    MG,    //  Mammography
    MR,    //  Magnetic Resonance
    MS,    //  Magnetic resonance spectroscopy
    NM,    //  Nuclear Medicine
    OAM,   //  Ophthalmic Axial Measurements
    OCT,   //  Optical Coherence Tomography (non-Ophthalmic)
    OP,    //  Ophthalmic Photography
    OPM,   //  Ophthalmic Mapping
    OPR,   //  Ophthalmic Refraction
    OPT,   //  Ophthalmic Tomography
    OPV,   //  Ophthalmic Visual Field
    OSS,   //  Optical Surface Scan
    #[default]
    OT, //  Other
    PLAN,  //  Plan
    PR,    //  Presentation State
    PT,    //  Positron emission tomography (PET)
    PX,    //  Panoramic X-Ray
    REG,   //  Registration
    RESP,  //  Respiratory Waveform
    RF,    //  Radio Fluoroscopy
    RG,    //  Radiographic imaging (conventional film/screen)
    RTDOSE, //  Radiotherapy Dose
    RTIMAGE, //  Radiotherapy Image
    RTPLAN, //  Radiotherapy Plan
    RTRECORD, //  RT Treatment Record
    RTSTRUCT, //  Radiotherapy Structure Set
    RWV,   //  Real World Value Map
    SEG,   //  Segmentation
    SM,    //  Slide Microscopy
    SMR,   //  Stereometric Relationship
    SR,    //  SR Document
    SRF,   //  Subjective Refraction
    ST,    //  Single-photon emission computed tomography (SPECT)
    STAIN, //  Automated Slide Stainer
    TG,    //  Thermography
    US,    //  Ultrasound
    VA,    //  Visual Acuity
    VF,    //  Videofluorography
    XA,    //  X-Ray Angiography
    XC,    //  External-camera Photography
}

impl FromStr for Modality {
    type Err = ModalityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AR" => Ok(Modality::AR),
            "AS" => Ok(Modality::AS),
            "ASMT" => Ok(Modality::ASMT),
            "AU" => Ok(Modality::AU),
            "BDUS" => Ok(Modality::BDUS),
            "BI" => Ok(Modality::BI),
            "BMD" => Ok(Modality::BMD),
            "CD" => Ok(Modality::CD),
            "CF" => Ok(Modality::CF),
            "CP" => Ok(Modality::CP),
            "CR" => Ok(Modality::CR),
            "CS" => Ok(Modality::CS),
            "CT" => Ok(Modality::CT),
            "DD" => Ok(Modality::DD),
            "DF" => Ok(Modality::DF),
            "DG" => Ok(Modality::DG),
            "DM" => Ok(Modality::DM),
            "DOC" => Ok(Modality::DOC),
            "DS" => Ok(Modality::DS),
            "DX" => Ok(Modality::DX),
            "EC" => Ok(Modality::EC),
            "ECG" => Ok(Modality::ECG),
            "EPS" => Ok(Modality::EPS),
            "ES" => Ok(Modality::ES),
            "FA" => Ok(Modality::FA),
            "FID" => Ok(Modality::FID),
            "FS" => Ok(Modality::FS),
            "GM" => Ok(Modality::GM),
            "HC" => Ok(Modality::HC),
            "HD" => Ok(Modality::HD),
            "IO" => Ok(Modality::IO),
            "IOL" => Ok(Modality::IOL),
            "IVOCT" => Ok(Modality::IVOCT),
            "IVUS" => Ok(Modality::IVUS),
            "KER" => Ok(Modality::KER),
            "KO" => Ok(Modality::KO),
            "LEN" => Ok(Modality::LEN),
            "LP" => Ok(Modality::LP),
            "LS" => Ok(Modality::LS),
            "MA" => Ok(Modality::MA),
            "MG" => Ok(Modality::MG),
            "MR" => Ok(Modality::MR),
            "MS" => Ok(Modality::MS),
            "NM" => Ok(Modality::NM),
            "OAM" => Ok(Modality::OAM),
            "OCT" => Ok(Modality::OCT),
            "OP" => Ok(Modality::OP),
            "OPM" => Ok(Modality::OPM),
            "OPR" => Ok(Modality::OPR),
            "OPT" => Ok(Modality::OPT),
            "OPV" => Ok(Modality::OPV),
            "OSS" => Ok(Modality::OSS),
            "OT" => Ok(Modality::OT),
            "PLAN" => Ok(Modality::PLAN),
            "PR" => Ok(Modality::PR),
            "PT" => Ok(Modality::PT),
            "PX" => Ok(Modality::PX),
            "REG" => Ok(Modality::REG),
            "RESP" => Ok(Modality::RESP),
            "RF" => Ok(Modality::RF),
            "RG" => Ok(Modality::RG),
            "RTDOSE" => Ok(Modality::RTDOSE),
            "RTIMAGE" => Ok(Modality::RTIMAGE),
            "RTPLAN" => Ok(Modality::RTPLAN),
            "RTRECORD" => Ok(Modality::RTRECORD),
            "RTSTRUCT" => Ok(Modality::RTSTRUCT),
            "RWV" => Ok(Modality::RWV),
            "SEG" => Ok(Modality::SEG),
            "SM" => Ok(Modality::SM),
            "SMR" => Ok(Modality::SMR),
            "SR" => Ok(Modality::SR),
            "SRF" => Ok(Modality::SRF),
            "ST" => Ok(Modality::ST),
            "STAIN" => Ok(Modality::STAIN),
            "TG" => Ok(Modality::TG),
            "US" => Ok(Modality::US),
            "VA" => Ok(Modality::VA),
            "VF" => Ok(Modality::VF),
            "XA" => Ok(Modality::XA),
            "XC" => Ok(Modality::XC),
            _ => Err(ModalityError::InvalidDicomModality(s.to_string())),
        }
    }
}

impl std::fmt::Display for Modality {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Modality::AR => write!(f, "AR"),
            Modality::AS => write!(f, "AS"),
            Modality::ASMT => write!(f, "ASMT"),
            Modality::AU => write!(f, "AU"),
            Modality::BDUS => write!(f, "BDUS"),
            Modality::BI => write!(f, "BI"),
            Modality::BMD => write!(f, "BMD"),
            Modality::CD => write!(f, "CD"),
            Modality::CF => write!(f, "CF"),
            Modality::CP => write!(f, "CP"),
            Modality::CR => write!(f, "CR"),
            Modality::CS => write!(f, "CS"),
            Modality::CT => write!(f, "CT"),
            Modality::DD => write!(f, "DD"),
            Modality::DF => write!(f, "DF"),
            Modality::DG => write!(f, "DG"),
            Modality::DM => write!(f, "DM"),
            Modality::DOC => write!(f, "DOC"),
            Modality::DS => write!(f, "DS"),
            Modality::DX => write!(f, "DX"),
            Modality::EC => write!(f, "EC"),
            Modality::ECG => write!(f, "ECG"),
            Modality::EPS => write!(f, "EPS"),
            Modality::ES => write!(f, "ES"),
            Modality::FA => write!(f, "FA"),
            Modality::FID => write!(f, "FID"),
            Modality::FS => write!(f, "FS"),
            Modality::GM => write!(f, "GM"),
            Modality::HC => write!(f, "HC"),
            Modality::HD => write!(f, "HD"),
            Modality::IO => write!(f, "IO"),
            Modality::IOL => write!(f, "IOL"),
            Modality::IVOCT => write!(f, "IVOCT"),
            Modality::IVUS => write!(f, "IVUS"),
            Modality::KER => write!(f, "KER"),
            Modality::KO => write!(f, "KO"),
            Modality::LEN => write!(f, "LEN"),
            Modality::LP => write!(f, "LP"),
            Modality::LS => write!(f, "LS"),
            Modality::MA => write!(f, "MA"),
            Modality::MG => write!(f, "MG"),
            Modality::MR => write!(f, "MR"),
            Modality::MS => write!(f, "MS"),
            Modality::NM => write!(f, "NM"),
            Modality::OAM => write!(f, "OAM"),
            Modality::OCT => write!(f, "OCT"),
            Modality::OP => write!(f, "OP"),
            Modality::OPM => write!(f, "OPM"),
            Modality::OPR => write!(f, "OPR"),
            Modality::OPT => write!(f, "OPT"),
            Modality::OPV => write!(f, "OPV"),
            Modality::OSS => write!(f, "OSS"),
            Modality::OT => write!(f, "OT"),
            Modality::PLAN => write!(f, "PLAN"),
            Modality::PR => write!(f, "PR"),
            Modality::PT => write!(f, "PT"),
            Modality::PX => write!(f, "PX"),
            Modality::REG => write!(f, "REG"),
            Modality::RESP => write!(f, "RESP"),
            Modality::RF => write!(f, "RF"),
            Modality::RG => write!(f, "RG"),
            Modality::RTDOSE => write!(f, "RTDOSE"),
            Modality::RTIMAGE => write!(f, "RTIMAGE"),
            Modality::RTPLAN => write!(f, "RTPLAN"),
            Modality::RTRECORD => write!(f, "RTRECORD"),
            Modality::RTSTRUCT => write!(f, "RTSTRUCT"),
            Modality::RWV => write!(f, "RWV"),
            Modality::SEG => write!(f, "SEG"),
            Modality::SM => write!(f, "SM"),
            Modality::SMR => write!(f, "SMR"),
            Modality::SR => write!(f, "SR"),
            Modality::SRF => write!(f, "SRF"),
            Modality::ST => write!(f, "ST"),
            Modality::STAIN => write!(f, "STAIN"),
            Modality::TG => write!(f, "TG"),
            Modality::US => write!(f, "US"),
            Modality::VA => write!(f, "VA"),
            Modality::VF => write!(f, "VF"),
            Modality::XA => write!(f, "XA"),
            Modality::XC => write!(f, "XC"),
        }
    }
}

impl Modality {
    pub fn description(&self) -> &'static str {
        match self {
            Modality::AR => "Autorefraction",
            Modality::AS => "Angioscopy",
            Modality::ASMT => "Content Assessment Results",
            Modality::AU => "Audio",
            Modality::BDUS => "Bone Densitometry (ultrasound)",
            Modality::BI => "Biomagnetic imaging",
            Modality::BMD => "Bone Densitometry (X-Ray)",
            Modality::CD => "Color flow Doppler",
            Modality::CF => "Cinefluorography",
            Modality::CP => "Colposcopy",
            Modality::CR => "Computed Radiography",
            Modality::CS => "Cystoscopy",
            Modality::CT => "Computed Tomography",
            Modality::DD => "Duplex Doppler",
            Modality::DF => "Digital fluoroscopy",
            Modality::DG => "Diaphanography",
            Modality::DM => "Digital microscopy",
            Modality::DOC => "Document",
            Modality::DS => "Digital Subtraction Angiography",
            Modality::DX => "Digital Radiography",
            Modality::EC => "Echocardiography",
            Modality::ECG => "Electrocardiography",
            Modality::EPS => "Cardiac Electrophysiology",
            Modality::ES => "Endoscopy",
            Modality::FA => "Fluorescein angiography",
            Modality::FID => "Fiducials",
            Modality::FS => "Fundoscopy",
            Modality::GM => "General Microscopy",
            Modality::HC => "Hard Copy",
            Modality::HD => "Hemodynamic Waveform",
            Modality::IO => "Intra-Oral Radiography",
            Modality::IOL => "Intraocular Lens Data",
            Modality::IVOCT => "Intravascular Optical Coherence Tomography",
            Modality::IVUS => "Intravascular Ultrasound",
            Modality::KER => "Keratometry",
            Modality::KO => "Key Object Selection",
            Modality::LEN => "Lensometry",
            Modality::LP => "Laparoscopy",
            Modality::LS => "Laser surface scan",
            Modality::MA => "Magnetic resonance angiography",
            Modality::MG => "Mammography",
            Modality::MR => "Magnetic Resonance",
            Modality::MS => "Magnetic resonance spectroscopy",
            Modality::NM => "Nuclear Medicine",
            Modality::OAM => "Ophthalmic Axial Measurements",
            Modality::OCT => "Optical Coherence Tomography (non-Ophthalmic)",
            Modality::OP => "Ophthalmic Photography",
            Modality::OPM => "Ophthalmic Mapping",
            Modality::OPR => "Ophthalmic Refraction",
            Modality::OPT => "Ophthalmic Tomography",
            Modality::OPV => "Ophthalmic Visual Field",
            Modality::OSS => "Optical Surface Scan",
            Modality::OT => "Other",
            Modality::PLAN => "Plan",
            Modality::PR => "Presentation State",
            Modality::PT => "Positron emission tomography (PET)",
            Modality::PX => "Panoramic X-Ray",
            Modality::REG => "Registration",
            Modality::RESP => "Respiratory Waveform",
            Modality::RF => "Radio Fluoroscopy",
            Modality::RG => "Radiographic imaging (conventional film/screen)",
            Modality::RTDOSE => "Radiotherapy Dose",
            Modality::RTIMAGE => "Radiotherapy Image",
            Modality::RTPLAN => "Radiotherapy Plan",
            Modality::RTRECORD => "RT Treatment Record",
            Modality::RTSTRUCT => "Radiotherapy Structure Set",
            Modality::RWV => "Real World Value Map",
            Modality::SEG => "Segmentation",
            Modality::SM => "Slide Microscopy",
            Modality::SMR => "Stereometric Relationship",
            Modality::SR => "SR Document",
            Modality::SRF => "Subjective Refraction",
            Modality::ST => "Single-photon emission computed tomography (SPECT)",
            Modality::STAIN => "Automated Slide Stainer",
            Modality::TG => "Thermography",
            Modality::US => "Ultrasound",
            Modality::VA => "Visual Acuity",
            Modality::VF => "Videofluorography",
            Modality::XA => "X-Ray Angiography",
            Modality::XC => "External-camera Photography",
        }
    }

    pub fn is_retired(&self) -> bool {
        matches!(
            self,
            Modality::AS
                | Modality::CD
                | Modality::CF
                | Modality::CP
                | Modality::CS
                | Modality::DD
                | Modality::DF
                | Modality::DM
                | Modality::DS
                | Modality::EC
                | Modality::FA
                | Modality::FS
                | Modality::LP
                | Modality::MA
                | Modality::MS
                | Modality::OPR
                | Modality::ST
                | Modality::VF
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApprovalStatusError {
    #[error("Invalid approval status: {0}")]
    InvalidApprovalStatus(String),
}

#[cfg(test)]
mod tests {
    use crate::{
        ApprovalStatus, Modality, PatientPosition, PatientPositionError, PersonName,
        PhotometricInterpretation, PhotometricInterpretationError, PixelRepresentation,
        RescaleType, RotationDirection,
    };
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

    #[test]
    fn test_rotation_direction_from_str_none() {
        let input = "NONE";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::NONE);

        let input = "none";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::NONE);

        let input = "None";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::NONE);
    }

    #[test]
    fn test_rotation_direction_from_str_cw() {
        let input = "CW";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CW);

        let input = "cw";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CW);
    }

    #[test]
    fn test_rotation_direction_from_str_ccw() {
        let input = "CCW";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CCW);

        let input = "ccw";
        let direction = RotationDirection::from_str(input).unwrap();
        assert_eq!(direction, RotationDirection::CCW);
    }

    #[test]
    fn test_rotation_direction_from_str_invalid() {
        let input = "INVALID";
        let direction = RotationDirection::from_str(input);
        assert!(direction.is_err());
    }

    #[test]
    fn test_patient_position_from_str_hfp() {
        let input = "HFP";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFP);
    }

    #[test]
    fn test_patient_position_from_str_hfs() {
        let input = "HFS";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFS);
    }

    #[test]
    fn test_patient_position_from_str_hfdr() {
        let input = "HFDR";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFDR);
    }

    #[test]
    fn test_patient_position_from_str_hfdl() {
        let input = "HFDL";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::HFDL);
    }

    #[test]
    fn test_patient_position_from_str_ffdr() {
        let input = "FFDR";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFDR);
    }

    #[test]
    fn test_patient_position_from_str_ffdl() {
        let input = "FFDL";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFDL);
    }

    #[test]
    fn test_patient_position_from_str_ffp() {
        let input = "FFP";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFP);
    }

    #[test]
    fn test_patient_position_from_str_ffs() {
        let input = "FFS";
        let position = PatientPosition::from_str(input).unwrap();
        assert_eq!(position, PatientPosition::FFS);
    }

    #[test]
    fn test_patient_position_from_str_invalid() {
        let input = "INVALID";
        let position = PatientPosition::from_str(input);
        assert!(position.is_err());
        if let Err(PatientPositionError::InvalidPatientPosition(pos)) = position {
            assert_eq!(pos, "INVALID".to_string());
        } else {
            panic!("Expected InvalidPatientPosition error");
        }
    }

    #[test]
    fn test_photometric_interpretation_from_str_monochrome1() {
        let input = "MONOCHROME1";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::MONOCHROME1);
    }

    #[test]
    fn test_photometric_interpretation_from_str_monochrome2() {
        let input = "MONOCHROME2";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::MONOCHROME2);
    }

    #[test]
    fn test_photometric_interpretation_from_str_palcolor() {
        let input = "PALETTE_COLOR";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::PALETTE_COLOR);
    }

    #[test]
    fn test_photometric_interpretation_from_str_rgb() {
        let input = "RGB";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::RGB);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_full() {
        let input = "YBR_FULL";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_FULL);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_full_422() {
        let input = "YBR_FULL_422";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_FULL_422);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_partial_422() {
        let input = "YBR_PARTIAL_422";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_PARTIAL_422);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_rct() {
        let input = "YBR_RCT";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_RCT);
    }

    #[test]
    fn test_photometric_interpretation_from_str_ybr_ict() {
        let input = "YBR_ICT";
        let interpretation = PhotometricInterpretation::from_str(input).unwrap();
        assert_eq!(interpretation, PhotometricInterpretation::YBR_ICT);
    }

    #[test]
    fn test_photometric_interpretation_from_str_invalid() {
        let input = "INVALID";
        let interpretation = PhotometricInterpretation::from_str(input);
        assert!(interpretation.is_err());
        if let Err(PhotometricInterpretationError::InvalidPhotometricInterpretation(pos)) =
            interpretation
        {
            assert_eq!(pos, "INVALID".to_string());
        } else {
            panic!("Expected InvalidPhotometricInterpretation error");
        }
    }

    #[test]
    fn test_pixel_representation_from_str() {
        assert_eq!(
            PixelRepresentation::from_str("0").unwrap(),
            PixelRepresentation::Unsigned
        );
        assert_eq!(
            PixelRepresentation::from_str("1").unwrap(),
            PixelRepresentation::Signed
        );
        assert!(PixelRepresentation::from_str("2").is_err());
    }

    #[test]
    fn test_rescale_type_from_str_us() {
        let input = "US";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::US);
    }

    #[test]
    fn test_rescale_type_from_str_od() {
        let input = "OD";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::OD);
    }

    #[test]
    fn test_rescale_type_from_str_hu() {
        let input = "HU";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::HU);
    }

    #[test]
    fn test_rescale_type_from_str_mgml() {
        let input = "MGML";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::MGML);
    }

    #[test]
    fn test_rescale_type_from_str_z_eff() {
        let input = "Z_EFF";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::Z_EFF);
    }

    #[test]
    fn test_rescale_type_from_str_ed() {
        let input = "ED";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::ED);
    }

    #[test]
    fn test_rescale_type_from_str_edw() {
        let input = "EDW";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::EDW);
    }

    #[test]
    fn test_rescale_type_from_str_hu_mod() {
        let input = "HU_MOD";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::HU_MOD);
    }

    #[test]
    fn test_rescale_type_from_str_pct() {
        let input = "PCT";
        let rescale_type = RescaleType::from_str(input).unwrap();
        assert_eq!(rescale_type, RescaleType::PCT);
    }

    #[test]
    fn test_rescale_type_from_str_invalid() {
        let input = "INVALID";
        let rescale_type_result = RescaleType::from_str(input);
        assert!(rescale_type_result.is_err());
    }

    #[test]
    fn test_approval_status_from_str_approved() {
        let input = "Approved";
        let status = ApprovalStatus::from_str(input).unwrap();
        assert_eq!(status, ApprovalStatus::APPROVED);
    }

    #[test]
    fn test_approval_status_from_str_pending() {
        let input = "UNAPPROVED";
        let status = ApprovalStatus::from_str(input).unwrap();
        assert_eq!(status, ApprovalStatus::UNAPPROVED);
    }

    #[test]
    fn test_approval_status_from_str_rejected() {
        let input = "Rejected";
        let status = ApprovalStatus::from_str(input).unwrap();
        assert_eq!(status, ApprovalStatus::REJECTED);
    }

    #[test]
    fn test_approval_status_from_str_invalid() {
        let input = "INVALID";
        let status_result = ApprovalStatus::from_str(input);
        assert!(status_result.is_err());
    }

    #[test]
    fn test_from_str_modality_valid() {
        use std::str::FromStr;

        let valid_modalities_str = &["CT", "MR", "US", "NM", "PT", "CR", "DX", "MG", "RG"];
        let valid_modalities = &[
            Modality::CT,
            Modality::MR,
            Modality::US,
            Modality::NM,
            Modality::PT,
            Modality::CR,
            Modality::DX,
            Modality::MG,
            Modality::RG,
        ];

        for (t, m) in valid_modalities_str.iter().zip(valid_modalities.iter()) {
            let result = Modality::from_str(t);
            assert!(result.is_ok());
            assert_eq!(*m, result.unwrap());
        }
    }

    #[test]
    fn test_from_str_modality_invalid() {
        use std::str::FromStr;

        let invalid_modalities = vec!["", "INVALID", "123", "CT1", "NMR", "ULTRASOUND"];

        for modality in invalid_modalities {
            let result = Modality::from_str(modality);
            assert!(result.is_err());
        }
    }
}
