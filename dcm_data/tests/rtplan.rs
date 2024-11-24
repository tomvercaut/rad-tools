use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dicom_dictionary_std::uids::{RT_PLAN_STORAGE, RT_STRUCTURE_SET_STORAGE};
use rad_tools_dcm_data::{
    ApprovalStatus, BeamDoseType, BeamType, FluenceMode, PatientPosition, PersonName,
    PrimaryDosimeterUnit, RTBeamLimitingDeviceType, RadiationType, RotationDirection,
    TreatmentDeliveryType,
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
fn read_rtplan_test() {
    init_logger();
    let path = Path::new("tests/resources/RP1.2.752.243.1.1.20220722130644612.2000.30831.dcm");
    let plan = rad_tools_dcm_data::io::read_rtplan(path).unwrap();
    assert_eq!(plan.specific_character_set, "ISO_IR 100");
    assert_eq!(
        plan.instance_creation_dt,
        Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2021, 12, 24).unwrap(),
            NaiveTime::from_hms_opt(8, 34, 19).unwrap()
        ))
    );
    assert_eq!(plan.sop.class_uid, RT_PLAN_STORAGE);
    assert_eq!(
        plan.sop.instance_uid,
        "1.2.752.243.1.1.20220722130644612.2000.30831"
    );
    assert_eq!(plan.study_dt, None);
    assert_eq!(plan.accession_number, None);
    assert_eq!(
        plan.manufacturer,
        Some("RaySearch Laboratories".to_string())
    );
    assert_eq!(plan.referring_physician_name, None);
    assert_eq!(plan.manufacturer_model_name, Some("RayStation".to_string()));
    assert_eq!(
        plan.patient_name,
        PersonName {
            family_name: "X_Rando".to_string(),
            given_name: "Head".to_string(),
            middle_name: "".to_string(),
            prefix: "".to_string(),
            suffix: "".to_string(),
        }
    );
    assert_eq!(
        plan.patient_birth_date,
        Some(NaiveDate::from_ymd_opt(2022, 7, 22).unwrap())
    );
    assert_eq!(plan.patient_sex, "O".to_string());
    assert!(plan.patient_identity_removed);
    assert_eq!(
        plan.deidentification_method,
        Some("RayStation 9.2.0.0".to_string())
    );
    assert_eq!(
        plan.study_instance_uid,
        "1.2.752.243.1.1.20220722130644226.3100.27382".to_string()
    );
    assert_eq!(
        plan.series_instance_uid,
        "1.2.752.243.1.1.20220722130644612.2010.75722".to_string()
    );
    assert_eq!(plan.study_id, None);
    assert_eq!(plan.series_number, Some(1));
    assert_eq!(
        plan.frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(plan.position_reference_indicator, None);
    assert_eq!(plan.rt_plan_label, "Anonymized");
    assert_eq!(plan.treatment_protocols, Some("SMLC".to_string()));
    assert_eq!(plan.plan_intent, Some("CURATIVE".to_string()));
    assert_eq!(plan.rt_plan_geometry, "PATIENT");
    assert_eq!(plan.fraction_group_sequence.len(), 1);
    let fg = plan.fraction_group_sequence.first().unwrap();
    assert_eq!(fg.fraction_group_number, 1);
    assert_eq!(fg.number_of_fractions_planned, Some(1));
    assert_eq!(fg.number_of_beams, 1);
    assert_eq!(fg.number_of_brachy_application_setups, 0);
    assert_eq!(fg.referenced_beam_sequence.len(), 1);
    let rb = fg.referenced_beam_sequence.first().unwrap();
    assert!(approx_equal(rb.beam_dose.unwrap(), 0.01100148, 1e-6));
    assert!(approx_equal(rb.beam_meterset.unwrap(), 1.0, 1e-6));
    assert!(approx_equal(
        rb.beam_dose_point_depth.unwrap(),
        88.2324753,
        1e-6
    ));
    assert!(approx_equal(
        rb.beam_dose_point_equivalent_depth.unwrap(),
        84.2336502,
        1e-6
    ));
    assert!(approx_equal(
        rb.beam_dose_point_ssd.unwrap(),
        911.767517,
        1e-6
    ));
    assert_eq!(rb.beam_dose_type, Some(BeamDoseType::Physical));
    assert_eq!(rb.referenced_beam_number, 1);

    assert_eq!(plan.beam_sequence.len(), 1);
    let beam = plan.beam_sequence.first().unwrap();
    assert_eq!(
        beam.primary_fluence_mode_sequence.as_ref().unwrap().len(),
        1
    );
    let primary_fluence_mode = beam
        .primary_fluence_mode_sequence
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(primary_fluence_mode.fluence_mode, FluenceMode::Standard);
    assert_eq!(primary_fluence_mode.fluence_mode_id, None);
    assert_eq!(beam.treatment_machine_name, Some("CLINACIX_V9".to_string()));
    assert_eq!(beam.primary_dosimeter_unit, Some(PrimaryDosimeterUnit::Mu));
    assert_eq!(beam.source_axis_distance, Some(1000.0));
    assert_eq!(beam.beam_limiting_device_sequence.len(), 3);
    let bld = beam.beam_limiting_device_sequence.first().unwrap();
    assert_eq!(
        bld.rt_beam_limiting_device_type,
        RTBeamLimitingDeviceType::AsymX
    );
    assert_eq!(bld.number_of_leaf_jaw_pairs, 1);
    assert_eq!(bld.source_to_beam_limiting_device_distance, None);
    assert_eq!(bld.leaf_position_boundaries, None);
    let bld = &beam.beam_limiting_device_sequence[1];
    assert_eq!(
        bld.rt_beam_limiting_device_type,
        RTBeamLimitingDeviceType::AsymY
    );
    assert_eq!(bld.number_of_leaf_jaw_pairs, 1);
    assert_eq!(bld.source_to_beam_limiting_device_distance, None);
    assert_eq!(bld.leaf_position_boundaries, None);
    let bld = &beam.beam_limiting_device_sequence[2];
    assert_eq!(
        bld.rt_beam_limiting_device_type,
        RTBeamLimitingDeviceType::MlcX
    );
    assert_eq!(bld.number_of_leaf_jaw_pairs, 60);
    assert_eq!(bld.source_to_beam_limiting_device_distance, None);
    let e_lp = vec![
        -200.0, -190.0, -180.0, -170.0, -160.0, -150.0, -140.0, -130.0, -120.0, -110.0, -100.0,
        -95.0, -90.0, -85.0, -80.0, -75.0, -70.0, -65.0, -60.0, -55.0, -50.0, -45.0, -40.0, -35.0,
        -30.0, -25.0, -20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0,
        40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0, 80.0, 85.0, 90.0, 95.0, 100.0, 110.0,
        120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0,
    ];
    assert_eq!(bld.leaf_position_boundaries, Some(e_lp));
    assert_eq!(beam.beam_number, 1);
    assert_eq!(beam.beam_name, Some("Syn 1".to_string()));
    assert_eq!(beam.beam_type, Some(BeamType::Static));
    assert_eq!(beam.radiation_type, Some(RadiationType::Photon));
    assert_eq!(
        beam.treatment_delivery_type,
        Some(TreatmentDeliveryType::Treatment)
    );
    assert_eq!(beam.number_of_wedges, 0);
    assert_eq!(beam.number_of_compensators, 0);
    assert_eq!(beam.number_of_boli, 0);
    assert_eq!(beam.number_of_blocks, 0);
    assert_eq!(beam.final_cumulative_meterset_weight, 1.0);
    assert_eq!(beam.number_of_control_points, 2);
    let cp = beam.control_point_sequence.first().unwrap();
    assert_eq!(cp.control_point_index, 0);
    assert_eq!(cp.gantry_angle, Some(0.0));
    assert_eq!(cp.gantry_rotation_direction, Some(RotationDirection::NONE));
    assert_eq!(cp.beam_limiting_device_angle, Some(0.0));
    assert_eq!(
        cp.beam_limiting_device_rotation_direction,
        Some(RotationDirection::NONE)
    );
    assert_eq!(cp.patient_support_angle, Some(0.0));
    assert_eq!(
        cp.patient_support_rotation_direction,
        Some(RotationDirection::NONE)
    );
    assert_eq!(cp.table_top_eccentric_angle, Some(0.0));
    assert_eq!(
        cp.table_top_eccentric_rotation_direction,
        Some(RotationDirection::NONE)
    );
    assert_eq!(cp.table_top_vertical_position, None);
    assert_eq!(cp.table_top_longitudinal_position, None);
    assert_eq!(cp.table_top_lateral_position, None);
    assert!(cp.isocenter_position.is_some());
    let e_iso = [0.5859375, -247.202, 195.1813];
    let iso = cp.isocenter_position.as_ref().unwrap();
    assert_eq!(iso.len(), 3);
    for i in 0..3 {
        assert!(approx_equal(iso[i], e_iso[i], 1e-6));
    }
    assert_eq!(cp.source_to_surface_distance, Some(911.7675));
    assert_eq!(cp.cumulative_meterset_weight, Some(0.0));
    assert_eq!(cp.table_top_pitch_angle, Some(0.0));
    assert_eq!(
        cp.table_top_pitch_rotation_direction,
        Some(RotationDirection::NONE)
    );
    assert_eq!(cp.table_top_roll_angle, Some(0.0));
    assert_eq!(
        cp.table_top_roll_rotation_direction,
        Some(RotationDirection::NONE)
    );
    assert_eq!(cp.gantry_pitch_angle, Some(0.0));
    assert_eq!(
        cp.gantry_pitch_rotation_direction,
        Some(RotationDirection::NONE)
    );
    let cp = &beam.control_point_sequence[1];
    assert_eq!(cp.control_point_index, 1);
    assert_eq!(cp.source_to_surface_distance, Some(911.7675));
    assert_eq!(cp.cumulative_meterset_weight, Some(1.0));
    assert_eq!(beam.referenced_patient_setup_number, Some(1));

    assert_eq!(plan.patient_setup_sequence.len(), 1);
    let ps = plan.patient_setup_sequence.first().unwrap();
    assert_eq!(ps.patient_position, PatientPosition::HFS);
    assert_eq!(ps.patient_setup_number, 1);

    assert_eq!(plan.referenced_structure_set_sequence.len(), 1);
    let rss = plan.referenced_structure_set_sequence.first().unwrap();
    assert_eq!(rss.class_uid, RT_STRUCTURE_SET_STORAGE);
    assert_eq!(
        rss.instance_uid,
        "1.2.752.243.1.1.20220722130644567.1980.53284"
    );

    assert_eq!(plan.approval_status, Some(ApprovalStatus::Unapproved));
}
