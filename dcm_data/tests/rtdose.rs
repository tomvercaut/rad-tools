use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dcm_data::{
    DoseSummationType, DoseType, DoseUnit, PersonName, PhotometricInterpretation,
    PixelRepresentation, TissueHeterogeneityCorrection,
};
use std::path::Path;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

fn init_logger() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

fn approx_equal(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

#[test]
#[allow(clippy::excessive_precision)]
fn read_rtdose_test() {
    init_logger();
    let path = Path::new("tests/resources/RD1.2.752.243.1.1.20220722130644614.2020.66722.dcm");
    let rtd = dcm_data::io::read_rtdose(path).unwrap();
    assert_eq!(rtd.specific_character_set, "ISO_IR 100");
    assert_eq!(
        rtd.instance_creation_dt,
        Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2021, 12, 24).unwrap(),
            NaiveTime::from_hms_opt(8, 34, 19).unwrap()
        ))
    );

    assert_eq!(rtd.study_dt, None);
    assert_eq!(rtd.content_dt, None);
    assert_eq!(rtd.accession_number, None);
    assert_eq!(rtd.manufacturer, Some("RaySearch Laboratories".to_string()));
    assert_eq!(rtd.referring_physician_name, None);
    assert_eq!(rtd.manufacturer_model_name, Some("RayStation".to_string()));
    assert_eq!(
        rtd.patient_name,
        PersonName {
            family_name: "X_Rando".to_string(),
            given_name: "Head".to_string(),
            ..PersonName::default()
        }
    );
    assert_eq!(rtd.patient_id, "X_Rando_Head");
    assert_eq!(
        rtd.patient_birth_date,
        Some(NaiveDate::from_ymd_opt(2022, 7, 22).unwrap())
    );
    assert_eq!(rtd.patient_sex, "O");
    assert!(rtd.patient_identity_removed);
    assert_eq!(
        rtd.deidentification_method,
        Some("RayStation 9.2.0.0".to_string())
    );
    assert!(approx_equal(rtd.slice_thickness.unwrap(), 2.0, 1e-6));
    assert_eq!(
        rtd.software_versions,
        Some("9.2.0.483 (Dicom Export)".to_string())
    );
    assert_eq!(
        rtd.study_instance_uid,
        "1.2.752.243.1.1.20220722130644226.3100.27382"
    );
    assert_eq!(
        rtd.series_instance_uid,
        "1.2.752.243.1.1.20220722130644614.2030.85616"
    );
    assert_eq!(rtd.study_id, None);
    assert_eq!(rtd.series_number, 1);
    assert_eq!(rtd.instance_number, 1);
    assert_eq!(rtd.image_position_patient, [-198.3936, -366.8501, 70.0]);
    assert_eq!(
        rtd.image_orientation_patient,
        [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
    );
    assert_eq!(
        rtd.frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(rtd.position_reference_indicator, None);
    assert_eq!(rtd.samples_per_pixel, 1);
    assert_eq!(
        rtd.photometric_interpretation,
        PhotometricInterpretation::MONOCHROME2
    );
    assert_eq!(rtd.number_of_frames, 169);
    assert_eq!(rtd.frame_increment_pointer, "(3004,000C)");
    assert_eq!(rtd.rows, 125);
    assert_eq!(rtd.columns, 202);
    assert_eq!(rtd.pixel_spacing, [2.0, 2.0]);
    assert_eq!(rtd.bits_allocated, 16);
    assert_eq!(rtd.bits_stored, 16);
    assert_eq!(rtd.high_bit, 15);
    assert_eq!(rtd.pixel_representation, PixelRepresentation::Unsigned);
    assert_eq!(rtd.dose_units, DoseUnit::GY);
    assert_eq!(rtd.dose_type, DoseType::PHYSICAL);
    assert_eq!(rtd.dose_comment, None);
    assert_eq!(rtd.dose_summation_type, DoseSummationType::PLAN);
    assert!(approx_equal(rtd.dose_grid_scaling, 2.655266E-07, 1E-15));
    assert_eq!(
        rtd.tissue_heterogeneity_correction,
        Some(TissueHeterogeneityCorrection::IMAGE)
    );

    let number_of_bytes = rtd.rows as usize
        * rtd.columns as usize
        * rtd.number_of_frames as usize
        * (rtd.bits_allocated as f64 / 8f64) as usize;
    assert_eq!(rtd.pixel_data_bytes.len(), number_of_bytes);

    let idxs = [
        [42, 100, 67, 0usize],
        [42, 89, 41, 0usize],
        [143, 51, 32, 0usize],
    ];
    let evals = [44908.0, 37416.0, 34684.0];
    assert_eq!(idxs.len(), evals.len());
    for (i, ev) in idxs.iter().zip(evals.iter()) {
        let val = rtd.pixel_data[[i[0], i[1], i[2], i[3]]];
        approx_equal(val, *ev, 1e-6);
    }
}
