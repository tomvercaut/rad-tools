use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dicom_dictionary_std::uids::{CT_IMAGE_STORAGE, RT_STRUCTURE_SET_STORAGE};
use rad_tools_dcm_data::{ApprovalStatus, ContourGeometry, PersonName, Sop};
use std::default::Default;
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
fn read_rtstruct_image_test() {
    init_logger();
    let path = Path::new("tests/resources/RS1.2.752.243.1.1.20220722130644567.1980.53284.dcm");
    let rs = rad_tools_dcm_data::io::read_rtstruct(path).unwrap();
    assert_eq!(rs.specific_character_set, "ISO_IR 100");
    assert_eq!(rs.sop.class_uid, RT_STRUCTURE_SET_STORAGE);
    assert_eq!(
        rs.sop.instance_uid,
        "1.2.752.243.1.1.20220722130644567.1980.53284"
    );
    assert_eq!(rs.accession_number, None);
    assert_eq!(rs.manufacturer, Some("RaySearch Laboratories".to_string()));
    assert_eq!(rs.referring_physician_name, None);
    assert_eq!(rs.manufacturer_model_name, Some("RayStation".to_string()));
    assert_eq!(
        rs.patient_name,
        PersonName {
            family_name: "X_Rando".to_string(),
            given_name: "Head".to_string(),
            ..PersonName::default()
        }
    );
    assert_eq!(rs.patient_id, "X_Rando_Head");
    assert_eq!(
        rs.patient_birth_date,
        Some(NaiveDate::from_ymd_opt(2022, 7, 22).unwrap())
    );
    assert_eq!(rs.patient_sex, "O".to_string());
    assert!(rs.patient_identity_removed);
    assert_eq!(
        rs.deidentification_method,
        Some("RayStation 9.2.0.0".to_string())
    );
    assert_eq!(
        rs.software_versions,
        Some("9.2.0.483 (Dicom Export)".to_string())
    );
    assert_eq!(
        rs.study_instance_uid,
        "1.2.752.243.1.1.20220722130644226.3100.27382".to_string()
    );
    assert_eq!(
        rs.series_instance_uid,
        "1.2.752.243.1.1.20220722130644567.1990.76407".to_string()
    );
    assert_eq!(rs.study_id, None);
    assert_eq!(rs.series_number, 1);
    assert_eq!(rs.instance_number, 1);
    assert_eq!(
        rs.frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(rs.position_reference_indicator, None);
    assert_eq!(rs.structure_set_label, "RS: Unapproved");
    assert_eq!(
        rs.structure_set_dt,
        Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2021, 11, 19).unwrap(),
            NaiveTime::from_hms_opt(12, 33, 38).unwrap()
        ))
    );
    assert_eq!(rs.referenced_frame_of_reference_seq.len(), 1);
    let ref_frame_of_reference = rs.referenced_frame_of_reference_seq.first().unwrap();
    assert_eq!(
        ref_frame_of_reference.frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(ref_frame_of_reference.rt_referenced_study_sequence.len(), 1);
    let rt_referenced_study = ref_frame_of_reference
        .rt_referenced_study_sequence
        .first()
        .unwrap();
    assert_eq!(
        rt_referenced_study.referenced_sop.class_uid,
        "1.2.840.10008.3.1.2.3.1"
    );
    assert_eq!(
        rt_referenced_study.referenced_sop.instance_uid,
        "1.2.752.243.1.1.20220722130644226.3100.27382"
    );
    assert_eq!(rt_referenced_study.rt_referenced_series_sequence.len(), 1);
    let rt_referenced_serie = rt_referenced_study
        .rt_referenced_series_sequence
        .first()
        .unwrap();
    assert_eq!(
        rt_referenced_serie.series_instance_uid,
        "1.2.752.243.1.1.20220722130644226.3200.74681"
    );
    assert_eq!(rt_referenced_serie.contour_image_sequence.len(), 165);
    assert_eq!(
        rt_referenced_serie
            .contour_image_sequence
            .first()
            .unwrap()
            .class_uid,
        CT_IMAGE_STORAGE
    );
    assert_eq!(
        rt_referenced_serie
            .contour_image_sequence
            .first()
            .unwrap()
            .instance_uid,
        "1.2.752.243.1.1.20220722130644225.2900.16575"
    );

    assert_eq!(rs.structure_set_roi_sequence.len(), 2);
    let structure_set_roi = rs.structure_set_roi_sequence.first().unwrap();
    assert_eq!(structure_set_roi.roi_number, 8);
    assert_eq!(
        structure_set_roi.referenced_frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(structure_set_roi.roi_name, Some("External".to_string()));
    assert_eq!(
        structure_set_roi.roi_generation_algorithm,
        Some("SEMIAUTOMATIC".to_string())
    );
    assert_eq!(structure_set_roi.roi_generation_description, None);
    let structure_set_roi = rs.structure_set_roi_sequence.get(1).unwrap();
    assert_eq!(structure_set_roi.roi_number, 7);
    assert_eq!(
        structure_set_roi.referenced_frame_of_reference_uid,
        "1.2.752.243.1.1.20220722130644226.3300.71363"
    );
    assert_eq!(structure_set_roi.roi_name, Some("Isocenter".to_string()));
    assert_eq!(
        structure_set_roi.roi_generation_algorithm,
        Some("SEMIAUTOMATIC".to_string())
    );
    assert_eq!(structure_set_roi.roi_generation_description, None);

    assert_eq!(rs.roi_contour_sequence.len(), 2);
    let roi_contour = rs.roi_contour_sequence.first().unwrap();
    assert_eq!(roi_contour.roi_display_color, Some([0, 128, 0]));
    assert_eq!(roi_contour.contour_sequence.as_ref().unwrap().len(), 314);
    let contour = roi_contour
        .contour_sequence
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(
        contour
            .contour_image_sequence
            .as_ref()
            .unwrap()
            .first()
            .unwrap(),
        &Sop {
            class_uid: CT_IMAGE_STORAGE.to_string(),
            instance_uid: "1.2.752.243.1.1.20220722130644565.1970.50255".to_string(),
        }
    );
    assert_eq!(contour.contour_geometry_type, ContourGeometry::ClosedPlanar);
    assert_eq!(contour.number_of_contour_points, 337);
    assert_eq!(contour.contour_number, Some(0));
    assert_eq!(contour.contour_data.len(), 1011);

    let exp_contour_data = [
        194.7266, -252.3594, 74.0, 194.5145, -266.4219, 74.0, 194.3005, -268.7656, 74.0, 193.8147,
        -271.1094, 74.0, 193.6724, -278.1406, 74.0, 193.4896, -282.8281, 74.0, 192.2157, -285.1719,
        74.0, 192.1875, -289.8594, 74.0, 192.0531, -296.8906, 74.0, 190.882, -299.2344, 74.0,
        188.6719, -300.4575, 74.0, 183.9844, -300.4118, 74.0, 179.2969, -300.2849, 74.0, 176.9531,
        -299.6987, 74.0, 174.6094, -298.6815, 74.0, 172.2656, -298.5419, 74.0, 169.9219, -298.29,
        74.0, 167.5781, -297.8526, 74.0, 162.8906, -297.6698, 74.0, 160.5469, -295.7357, 74.0,
        155.8594, -295.6176, 74.0, 148.8281, -295.5385, 74.0, 139.4531, -295.5385, 74.0, 134.7656,
        -295.6107, 74.0, 132.4219, -295.9021, 74.0, 130.0781, -297.6103, 74.0, 125.3906, -297.808,
        74.0, 123.0469, -298.422, 74.0, 120.7031, -298.6046, 74.0, 119.9625, -299.2344, 74.0,
        118.3594, -300.1752, 74.0, 116.0156, -300.2766, 74.0, 113.2076, -301.5781, 74.0, 111.3281,
        -302.1711, 74.0, 108.9844, -302.7628, 74.0, 104.2969, -303.0818, 74.0, 103.3609, -303.9219,
        74.0, 101.9531, -304.9021, 74.0, 99.60938, -305.0938, 74.0, 97.26563, -306.5215, 74.0,
        94.92188, -306.8835, 74.0, 90.23438, -308.0193, 74.0, 89.52927, -308.6094, 74.0, 87.89063,
        -309.5345, 74.0, 85.54688, -309.9477, 74.0, 83.20313, -310.6437, 74.0, 80.85938, -310.9804,
        74.0, 78.51563, -311.9765, 74.0, 76.17188, -312.092, 74.0, 73.82813, -314.1003, 74.0,
        71.48438, -314.1086, 74.0, 69.14063, -314.417, 74.0, 66.79688, -314.927, 74.0, 64.45313,
        -315.9788, 74.0, 62.10938, -316.608, 74.0, 59.3013, -317.9844, 74.0, 57.42188, -318.5703,
        74.0, 55.07813, -318.6928, 74.0, 52.73438, -319.3785, 74.0, 50.39063, -319.465, 74.0,
        48.04688, -320.4746, 74.0, 45.70313, -321.2689, 74.0, 43.35938, -321.3488, 74.0, 41.01563,
        -321.5357, 74.0, 38.67188, -322.9547, 74.0, 31.64063, -323.2364, 74.0, 29.29688, -323.8316,
        74.0, 17.57813, -323.9746, 74.0, 1.171875, -324.0586, 74.0, -1.171875, -324.4845, 74.0,
        -1.868367, -325.0156, 74.0, -3.515625, -325.88, 74.0, -12.89063, -325.88, 74.0, -14.70947,
        -325.0156, 74.0, -15.23438, -324.628, 74.0, -17.57813, -324.0639, 74.0, -29.29688,
        -323.8438, 74.0, -31.64063, -323.2824, 74.0, -38.67188, -322.9828, 74.0, -41.01563,
        -321.5527, 74.0, -43.35938, -321.3549, 74.0, -45.70313, -321.2755, 74.0, -48.04688,
        -320.5064, 74.0, -50.39063, -319.5493, 74.0, -55.07813, -319.4083, 74.0, -57.42188,
        -318.9557, 74.0, -62.10938, -318.5908, 74.0, -62.76948, -317.9844, 74.0, -64.45313,
        -316.7585, 74.0, -66.79688, -316.6413, 74.0, -69.14063, -315.9376, 74.0, -71.48438,
        -315.0036, 74.0, -73.82813, -314.4567, 74.0, -76.17188, -314.231, 74.0, -78.51563,
        -314.0923, 74.0, -80.85938, -312.0494, 74.0, -83.20313, -311.9447, 74.0, -85.54688,
        -310.8799, 74.0, -87.89063, -310.4792, 74.0, -90.23438, -309.6516, 74.0, -92.57813,
        -309.5345, 74.0, -93.97928, -308.6094, 74.0, -94.92188, -307.8064, 74.0, -97.26563,
        -307.4203, 74.0, -99.60938, -306.8115, 74.0, -101.9531, -306.5361, 74.0, -104.2969,
        -306.1526, 74.0, -106.6406, -304.8782, 74.0, -107.9571, -303.9219, 74.0, -108.9844,
        -303.0127, 74.0, -111.3281, -302.871, 74.0, -113.6719, -302.2054, 74.0, -116.0156,
        -301.9814, 74.0, -116.4325, -301.5781, 74.0, -118.3594, -300.2489, 74.0, -120.0714,
        -299.2344, 74.0, -120.7031, -298.6689, 74.0, -123.0469, -298.422, 74.0, -125.3906,
        -297.7695, 74.0, -127.7344, -297.6053, 74.0, -130.0781, -295.6432, 74.0, -132.4219,
        -295.543, 74.0, -134.7656, -295.2397, 74.0, -137.1094, -294.2375, 74.0, -141.7969,
        -294.073, 74.0, -146.4844, -294.073, 74.0, -151.1719, -294.1744, 74.0, -153.5156,
        -294.2706, 74.0, -155.8594, -295.0112, 74.0, -158.2031, -295.5385, 74.0, -160.5469,
        -295.6282, 74.0, -162.8906, -295.8258, 74.0, -165.2344, -297.6982, 74.0, -167.5781,
        -297.8176, 74.0, -169.9219, -298.2737, 74.0, -172.2656, -298.5057, 74.0, -174.6094,
        -298.6444, 74.0, -176.9531, -299.625, 74.0, -179.2969, -300.2198, 74.0, -183.9844,
        -300.226, 74.0, -186.3281, -299.3481, 74.0, -188.6719, -298.3426, 74.0, -189.9064,
        -296.8906, 74.0, -190.0873, -294.5469, 74.0, -190.2416, -287.5156, 74.0, -191.7108,
        -285.1719, 74.0, -192.0531, -282.8281, 74.0, -192.2266, -268.7656, 74.0, -193.4896,
        -266.4219, 74.0, -193.6584, -261.7344, 74.0, -193.8415, -254.7031, 74.0, -194.4872,
        -252.3594, 74.0, -194.6581, -242.9844, 74.0, -194.6978, -238.2969, 74.0, -195.8999,
        -235.9531, 74.0, -196.7454, -233.6094, 74.0, -196.8001, -226.5781, 74.0, -196.7894,
        -212.5156, 74.0, -196.7346, -200.7969, 74.0, -196.6833, -193.7656, 74.0, -194.7266,
        -191.4219, 74.0, -194.6635, -186.7344, 74.0, -193.8357, -184.3906, 74.0, -193.3594,
        -183.9049, 74.0, -190.8584, -182.0469, 74.0, -188.6719, -181.1227, 74.0, -186.3281,
        -178.7968, 74.0, -183.9844, -178.1031, 74.0, -181.6406, -176.88, 74.0, -179.2969,
        -174.4297, 74.0, -176.9531, -173.883, 74.0, -174.6094, -173.2121, 74.0, -174.0493,
        -172.6719, 74.0, -172.2656, -171.5383, 74.0, -170.657, -170.3281, 74.0, -169.9219,
        -169.6278, 74.0, -167.5781, -168.9915, 74.0, -165.2344, -166.9651, 74.0, -162.6923,
        -165.6406, 74.0, -160.5469, -164.8616, 74.0, -158.2031, -164.3406, 74.0, -155.8594,
        -161.9002, 74.0, -153.5156, -160.9639, 74.0, -151.1719, -159.8893, 74.0, -148.8281,
        -157.8511, 74.0, -146.4844, -157.6474, 74.0, -144.1406, -156.1354, 74.0, -139.4531,
        -154.3569, 74.0, -137.1094, -153.1166, 74.0, -132.4219, -151.9036, 74.0, -132.0743,
        -151.5781, 74.0, -130.0781, -150.4511, 74.0, -128.1804, -149.2344, 74.0, -127.7344,
        -148.8386, 74.0, -125.3906, -148.0982, 74.0, -122.9889, -146.8906, 74.0, -116.0156,
        -143.645, 74.0, -113.6867, -144.5469, 74.0, -116.0156, -145.6619, 74.0, -117.1943,
        -146.8906, 74.0, -116.0156, -148.7506, 74.0, -113.6719, -148.8438, 74.0, -111.3281,
        -148.0687, 74.0, -109.9693, -146.8906, 74.0, -109.8824, -142.2031, 74.0, -108.9844,
        -141.3242, 74.0, -106.6406, -140.3604, 74.0, -106.0596, -139.8594, 74.0, -104.2969,
        -138.8927, 74.0, -101.9531, -138.7255, 74.0, -99.60938, -136.8507, 74.0, -97.26563,
        -136.6786, 74.0, -94.92188, -136.0863, 74.0, -92.57813, -135.9108, 74.0, -91.74944,
        -135.1719, 74.0, -90.23438, -134.1899, 74.0, -87.89063, -134.0699, 74.0, -85.54688,
        -133.4922, 74.0, -83.20313, -132.2779, 74.0, -80.85938, -132.1634, 74.0, -78.51563,
        -131.5777, 74.0, -76.17188, -131.4641, 74.0, -73.82813, -130.872, 74.0, -73.38995,
        -130.4844, 74.0, -71.48438, -129.4837, 74.0, -66.79688, -129.3449, 74.0, -64.45313,
        -128.895, 74.0, -62.10938, -128.1514, 74.0, -57.42188, -128.0607, 74.0, -55.07813,
        -128.0607, 74.0, -50.39063, -128.1726, 74.0, -48.04688, -128.9146, 74.0, -45.70313,
        -129.3557, 74.0, -38.67188, -129.537, 74.0, -37.35542, -130.4844, 74.0, -36.32813,
        -131.3833, 74.0, -33.98438, -131.4861, 74.0, -31.64063, -131.7829, 74.0, -29.29688,
        -132.2073, 74.0, -26.95313, -132.3001, 74.0, -24.60938, -132.6189, 74.0, -22.26563,
        -134.0163, 74.0, -17.57813, -134.1944, 74.0, -15.99522, -135.1719, 74.0, -15.23438,
        -135.8426, 74.0, -10.54688, -135.9793, 74.0, -5.859375, -135.9052, 74.0, -1.171875,
        -135.9604, 74.0, 5.859375, -135.921, 74.0, 10.54688, -135.7974, 74.0, 12.89063, -135.5443,
        74.0, 13.30482, -135.1719, 74.0, 15.23438, -134.1545, 74.0, 19.92188, -134.0, 74.0,
        22.26563, -133.6213, 74.0, 24.60938, -132.3228, 74.0, 26.95313, -132.2213, 74.0, 31.64063,
        -132.14, 74.0, 33.98438, -131.6087, 74.0, 38.67188, -131.5198, 74.0, 43.35938, -131.5129,
        74.0, 52.73438, -131.6024, 74.0, 55.07813, -131.7176, 74.0, 57.42188, -132.1536, 74.0,
        59.76563, -132.2073, 74.0, 64.45313, -132.4248, 74.0, 64.92011, -132.8281, 74.0, 66.79688,
        -134.0, 74.0, 69.14063, -134.0856, 74.0, 71.48438, -134.3298, 74.0, 72.90453, -135.1719,
        74.0, 73.82813, -135.875, 74.0, 76.17188, -136.0387, 74.0, 78.51563, -136.5619, 74.0,
        83.20313, -136.8893, 74.0, 83.88428, -137.5156, 74.0, 85.54688, -138.7415, 74.0, 87.89063,
        -138.9319, 74.0, 89.70269, -139.8594, 74.0, 90.23438, -140.2911, 74.0, 92.57813, -140.6862,
        74.0, 94.92188, -141.1916, 74.0, 97.26563, -141.8763, 74.0, 99.60938, -143.3916, 74.0,
        101.9531, -143.5175, 74.0, 106.6406, -145.4911, 74.0, 108.9844, -145.8714, 74.0, 110.6572,
        -146.8906, 74.0, 111.3281, -147.501, 74.0, 113.6719, -148.2167, 74.0, 116.0156, -148.8258,
        74.0, 118.3594, -149.8447, 74.0, 120.7031, -150.5527, 74.0, 123.0469, -151.5155, 74.0,
        125.3906, -152.6604, 74.0, 129.9141, -153.9219, 74.0, 132.4219, -155.2836, 74.0, 133.9701,
        -156.2656, 74.0, 134.7656, -156.9733, 74.0, 137.1094, -157.6489, 74.0, 139.0707, -158.6094,
        74.0, 139.4531, -158.9047, 74.0, 141.7969, -159.899, 74.0, 144.1406, -160.5625, 74.0,
        146.4844, -161.6654, 74.0, 148.8281, -162.388, 74.0, 151.1719, -164.5296, 74.0, 153.5156,
        -165.2867, 74.0, 155.8594, -166.4294, 74.0, 158.2031, -167.0704, 74.0, 160.5469, -169.0704,
        74.0, 162.8906, -169.8113, 74.0, 165.2344, -171.292, 74.0, 166.3502, -172.6719, 74.0,
        166.4238, -175.0156, 74.0, 167.5781, -176.1473, 74.0, 169.9219, -176.2315, 74.0, 172.2656,
        -175.7647, 74.0, 174.6094, -176.1701, 74.0, 176.9531, -176.8833, 74.0, 179.2969, -178.0776,
        74.0, 181.6406, -178.8228, 74.0, 183.9844, -180.9614, 74.0, 186.3281, -181.6821, 74.0,
        188.6719, -182.7898, 74.0, 191.0156, -185.1597, 74.0, 195.7031, -186.4639, 74.0, 197.8766,
        -189.0781, 74.0, 198.1606, -191.4219, 74.0, 198.1606, -210.1719, 74.0, 198.0741, -219.5469,
        74.0, 197.9144, -228.9219, 74.0, 197.8953, -233.6094, 74.0, 197.7055, -235.9531, 74.0,
        196.8805, -238.2969, 74.0, 196.8532, -242.9844, 74.0, 196.713, -250.0156, 74.0,
    ];
    assert_eq!(contour.contour_data.len(), exp_contour_data.len());

    for (actual, expected) in contour.contour_data.iter().zip(exp_contour_data.iter()) {
        assert!(approx_equal(*actual, *expected, 1e-6));
    }

    let roi_contour = rs.roi_contour_sequence.last().unwrap();
    assert_eq!(roi_contour.roi_display_color, Some([255, 255, 0]));
    let contour = roi_contour
        .contour_sequence
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(contour.contour_geometry_type, ContourGeometry::Point);
    assert_eq!(contour.number_of_contour_points, 1);
    assert_eq!(contour.contour_number, Some(0));
    assert_eq!(contour.contour_data.len(), 3);

    let exp_contour_data = [0.5859375, -247.202, 195.1813];
    for (actual, expected) in contour.contour_data.iter().zip(exp_contour_data.iter()) {
        assert!(approx_equal(*actual, *expected, 1e-6));
    }

    assert_eq!(rs.rt_roi_observations_sequence.len(), 2);
    let rt_roi_observation = rs.rt_roi_observations_sequence.first().unwrap();
    assert_eq!(rt_roi_observation.observation_number, 1);
    assert_eq!(rt_roi_observation.referenced_roi_number, 8);
    assert_eq!(
        rt_roi_observation.rt_roi_interpreted_type,
        Some("EXTERNAL".to_string())
    );
    assert_eq!(rt_roi_observation.roi_interpreter, None);
    let rt_roi_observation = rs.rt_roi_observations_sequence.last().unwrap();
    assert_eq!(rt_roi_observation.observation_number, 2);
    assert_eq!(rt_roi_observation.referenced_roi_number, 7);
    assert_eq!(
        rt_roi_observation.rt_roi_interpreted_type,
        Some("ISOCENTER".to_string())
    );
    assert_eq!(rt_roi_observation.roi_interpreter, None);
    assert_eq!(rs.approval_status, Some(ApprovalStatus::Unapproved));
}
