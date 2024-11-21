use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dcm_data::{
    Modality, PatientPosition, PersonName, PhotometricInterpretation, PixelRepresentation,
    RescaleType, RotationDirection,
};
use dicom_dictionary_std::uids::CT_IMAGE_STORAGE;
use dicom_pixeldata::ndarray::s;
use std::path::Path;
use tracing::debug;
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
fn read_ct_image_test() {
    init_logger();
    let path = Path::new("tests/resources/CT1.2.752.243.1.1.20220722130644359.1060.62784.dcm");
    let ct = dcm_data::io::read_ct_image(path).unwrap();
    assert_eq!(
        ct.image_type,
        vec!["ORIGINAL", "PRIMARY", "AXIAL", "CT_SOM5 SPI"]
    );
    assert_eq!(ct.sop.class_uid, CT_IMAGE_STORAGE);
    assert_eq!(
        ct.sop.instance_uid,
        "1.2.752.243.1.1.20220722130644359.1060.62784"
    );
    assert_eq!(ct.modality, Modality::CT);
    assert!(ct.manufacturer.is_some());
    assert_eq!(ct.manufacturer.as_ref().unwrap(), "Siemens Healthineers");
    assert!(ct.referring_physician_name.is_none());
    assert!(ct.station_name.is_some());
    assert_eq!(ct.station_name.as_ref().unwrap(), "CT130037");
    assert!(ct.manufacturer_model_name.is_some());
    assert_eq!(
        ct.manufacturer_model_name.as_ref().unwrap(),
        "SOMATOM go.Open Pro"
    );
    assert_eq!(
        ct.irradiation_event_uid,
        "1.2.752.243.1.1.20220722130644226.3000.75723"
    );
    assert_eq!(
        ct.patient_name,
        PersonName {
            family_name: "X_Rando".into(),
            given_name: "Head".into(),
            ..PersonName::default()
        }
    );
    assert_eq!(
        ct.patient_birth_date,
        NaiveDate::from_ymd_opt(2022, 7, 22).unwrap()
    );
    assert_eq!(ct.patient_sex, "O");
    assert!(ct.patient_identity_removed);
    assert!(ct.deidentification_method.is_some());
    assert_eq!(
        ct.deidentification_method.as_ref().unwrap(),
        "RayStation 9.2.0.0"
    );
    assert!(ct.body_part_examined.is_some());
    assert_eq!(ct.body_part_examined.as_ref().unwrap(), "NECK");
    assert!(ct.slice_thickness.is_some());
    assert_eq!(*ct.slice_thickness.as_ref().unwrap(), 2.0);
    assert_eq!(ct.kvp, 90.0);
    assert_eq!(ct.data_collection_diameter, 600.5);
    assert_eq!(ct.device_serial_number, "130037");
    assert_eq!(ct.software_versions, "syngo CT VA40A");
    assert_eq!(ct.reconstruction_diameter, 600.0);
    assert_eq!(ct.distance_source_to_detector, 1113.0);
    assert_eq!(ct.distance_source_to_patient, 610.0);
    assert_eq!(ct.gantry_detector_tilt, 0.0);
    assert_eq!(ct.table_height, 246.5);
    assert_eq!(ct.rotation_direction, RotationDirection::CW);
    assert_eq!(ct.exposure_time, 1250);
    assert_eq!(ct.xray_tube_current, 225);
    assert_eq!(ct.exposure, 282);
    assert_eq!(ct.filter_type, "W1");
    assert_eq!(ct.genereator_power, 25);
    assert_eq!(ct.focal_spots, [1.6, 1.6]);
    assert!(ct.last_calibration_dt.is_some());
    assert_eq!(
        *ct.last_calibration_dt.as_ref().unwrap(),
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2021, 11, 19).unwrap(),
            NaiveTime::from_hms_opt(8, 8, 37).unwrap()
        )
    );
    assert_eq!(
        ct.convolution_kernel,
        ["Sm36f".to_string(), "3".to_string()]
    );
    assert_eq!(ct.patient_position, PatientPosition::HFS);
    assert_eq!(ct.revolution_time, 1.0);
    assert_eq!(ct.single_collimation_width, 0.6);
    assert!(approx_equal(
        ct.total_collimation_width,
        38.399999999999999,
        0.000000000000001
    ));
    assert!(approx_equal(
        ct.table_speed,
        30.699999999999999,
        0.000000000000001
    ));
    assert!(approx_equal(
        ct.table_feed_per_rotation,
        30.719999999999999,
        0.000000000000001
    ));
    assert_eq!(ct.spiral_pitch_factor, 0.8);
    assert_eq!(ct.data_collection_center_patient, [0.0, -246.5, 256.0]);
    assert_eq!(
        ct.reconstruction_target_center_patient,
        [0.0, -246.5, 256.0]
    );
    assert!(approx_equal(
        ct.ctdi_vol,
        8.4986236175965004,
        0.000000000000001
    ));
    assert_eq!(ct.ctdi_phantom_type_code_sequence.len(), 1);
    assert_eq!(
        ct.ctdi_phantom_type_code_sequence[0].code_value,
        Some("113691".to_string())
    );
    assert_eq!(
        ct.ctdi_phantom_type_code_sequence[0].coding_scheme_designator,
        Some("DCM".to_string())
    );
    assert_eq!(
        ct.ctdi_phantom_type_code_sequence[0].coding_scheme_version,
        None
    );
    assert_eq!(
        ct.ctdi_phantom_type_code_sequence[0].code_meaning,
        "IEC Body Dosimetry Phantom"
    );
    assert_eq!(
        ct.study_instance_uid,
        "1.2.752.243.1.1.20220722130644226.3100.27382"
    );
    assert_eq!(
        ct.series_instance_uid,
        "1.2.752.243.1.1.20220722130644226.3200.74681"
    );
    assert_eq!(ct.study_id, None);
    assert_eq!(ct.series_number, 2);
    assert_eq!(ct.acquisition_number, 201);
    assert_eq!(ct.instance_number, 74);
    assert_eq!(
        ct.image_position_patient,
        [-299.4140625, -545.9140625, 256.0]
    );
    assert_eq!(ct.image_orientation_patient, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    assert_eq!(
        ct.frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(ct.position_reference_indicator, None);
    assert_eq!(ct.slice_location, Some(256.0));
    assert_eq!(ct.samples_per_pixel, 1);
    assert_eq!(
        ct.photometric_interpretation,
        PhotometricInterpretation::MONOCHROME2
    );
    assert_eq!(ct.rows, 512);
    assert_eq!(ct.columns, 512);
    assert_eq!(ct.pixel_spacing, [1.171875, 1.171875]);
    assert_eq!(ct.bits_allocated, 16);
    assert_eq!(ct.bits_stored, 16);
    assert_eq!(ct.high_bit, 15);
    assert_eq!(ct.pixel_representation, PixelRepresentation::Unsigned);
    assert_eq!(ct.smallest_image_pixel_value, Some(0));
    assert_eq!(ct.largest_image_pixel_value, Some(9450));
    assert_eq!(ct.burned_in_annotation, Some("NO".to_string()));
    assert_eq!(ct.window_center, 50.0);
    assert_eq!(ct.window_width, 255.0);
    assert_eq!(ct.rescale_intercept, -8192.0);
    assert_eq!(ct.rescale_slope, 1.0);
    assert_eq!(ct.rescale_type, RescaleType::HU);
    assert_eq!(ct.window_center_width_explanation, None);
    assert_eq!(ct.lossy_image_compression, Some("00".to_string()));
    let number_of_bytes =
        ct.rows as usize * ct.columns as usize * (ct.bits_allocated as f64 / 8f64) as usize;
    debug!("number of bytes: {number_of_bytes}");
    let sub_pixels = ct
        .pixel_data
        .slice(s![0, 198, 239..249, 0])
        .as_slice()
        .unwrap()
        .to_vec();
    // Verified values using ImageJ
    let expected_sub_pixels = vec![
        -2.0, -38.0, -107.0, 197.0, 548.0, 314.0, -122.0, -165.0, -92.0, -54.0,
    ];
    assert_eq!(sub_pixels, expected_sub_pixels);
    debug!("pixels row [198]: {:#?}", sub_pixels);
    assert_eq!(ct.pixel_data_bytes.len(), number_of_bytes);
}
