#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use log::error;

    use peeteewee::data::mcc::{CurveType, Orientation, Rotation};

    #[allow(dead_code)]
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn read_mcc_profile() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("resources");
        d.push("pdd_x06_fff_open_8x8_crin_wat.mcc");
        println!("{:?}", d);

        let r = peeteewee::io::mcc::read(d);
        if r.is_err() {
            let te = format!("{}", r.as_ref().err().unwrap());
            error!("{}", te);
        }
        assert!(r.is_ok());
        let mcc = r.unwrap();
        assert_eq!("CC-Export V1.9", mcc.format.as_str());
        assert_eq!("10-Dec-2023 13:12:13", mcc.file_creation_date.as_str());
        assert_eq!("10-Dec-2023 13:12:13", mcc.last_modified.as_str());
        assert_eq!(10, mcc.scans.len());

        let scan = &mcc.scans[0];

        assert_eq!("tba PDD Profiles", scan.task_name.as_str());
        assert_eq!("WaterTankScans", scan.program.as_str());
        assert_eq!("10-Dec-2023 12:48:35", scan.meas_date.as_str());
        assert_eq!("Ethos1", scan.linac.as_str());
        assert_eq!("X", scan.modality.as_str());
        assert_eq!(1000.00, scan.isocenter);
        assert_eq!("InplaneGT", scan.inplane_axis.as_str());
        assert_eq!("CrossplAB", scan.crossplane_axis.as_str());
        assert_eq!("Depth", scan.depth_axis.as_str());
        assert_eq!("GUN_TARGET", scan.inplane_axis_dir.as_str());
        assert_eq!("LEFT_RIGHT", scan.crossplane_axis_dir.as_str());
        assert_eq!("UP_DOWN", scan.depth_axis_dir.as_str());
        assert_eq!(6.00, scan.energy);
        assert_eq!(13.00, scan.nominal_dmax);
        assert_eq!(900.00, scan.ssd);
        assert!(!scan.block);
        assert_eq!(0.00, scan.wedge_angle);
        assert_eq!(80.00, scan.field_inplane);
        assert_eq!(80.00, scan.field_crossplane);
        assert_eq!("RECTANGULAR", scan.field_type);
        assert_eq!(0.00, scan.gantry);
        assert!(!scan.gantry_upright_position);
        assert_eq!(Rotation::CW, scan.gantry_rotation);
        assert_eq!(0.00, scan.coll_angle);
        assert_eq!(0.00, scan.coll_offset_inplane);
        assert_eq!(0.00, scan.coll_offset_crossplane);
        assert_eq!("BeamScan", scan.scan_device.as_str());
        assert_eq!("BARA_GUN_TARGET", scan.scan_device_setup.as_str());
        assert_eq!("BeamScan", scan.electrometer.as_str());
        assert_eq!("LOW", scan.range_field.as_str());
        assert_eq!("LOW", scan.range_reference.as_str());
        assert_eq!("THIMBLE_CHAMBER", scan.detector.as_str());
        assert_eq!("SEMIFLEX", scan.detector_subcode.as_str());
        assert_eq!(2.40, scan.detector_radius);
        assert_eq!("PTW 31021 Semiflex 3D 668", scan.detector_name.as_str());
        assert_eq!("144668", scan.detector_sn.as_str());
        assert_eq!(500000000.00, scan.detector_calibration);
        assert!(scan.detector_is_calibrated);
        assert_eq!("THIMBLE_CHAMBER", scan.detector_reference.as_str());
        assert_eq!("SEMIFLEX", scan.detector_reference_subcode.as_str());
        assert_eq!(2.40, scan.detector_reference_radius);
        assert_eq!(
            "PTW 31021 Semiflex 3D 669",
            scan.detector_reference_name.as_str()
        );
        assert_eq!("144669", scan.detector_reference_sn.as_str());
        assert!(!scan.detector_reference_is_calibrated);
        assert_eq!(500000000.00, scan.detector_reference_calibration);
        assert_eq!(400.0, scan.detector_hv);
        assert_eq!(400.0, scan.detector_reference_hv);
        assert_eq!(Orientation::Horizontal, scan.detector_orientation);
        assert_eq!("FFF", scan.filter.as_str());
        assert_eq!(2.00, scan.scan_speed_profile);
        assert_eq!("NONE", scan.scan_prof_speed_dep.as_str());
        assert_eq!(2.00, scan.scan_speed_pdd);
        assert_eq!("NONE", scan.scan_pdd_speed_dep.as_str());
        assert_eq!("T31021", scan.detector_type.as_str());
        assert_eq!(100.00, scan.ref_field_depth);
        assert_eq!("ISOCENTER", scan.ref_field_defined.as_str());
        assert_eq!(100.00, scan.ref_field_inplane);
        assert_eq!(100.00, scan.ref_field_crossplane);
        let e_ref_scan_positions = vec![
            -120.00, -119.00, -118.00, -117.00, -116.00, -115.00, -114.00, -113.00, -112.00,
            -111.00, -110.00, -109.00, -108.00, -107.00, -106.00, -105.00, -104.00, -103.00,
            -102.00, -101.00, -100.00, -99.00, -98.00, -97.00, -96.00, -95.00, -94.00, -93.00,
            -92.00, -91.00, -90.00, -89.00, -88.00, -87.00, -86.00, -85.00, -84.00, -83.00, -82.00,
            -81.00, -80.00, -79.00, -78.00, -77.00, -76.00, -75.00, -74.00, -73.00, -72.00, -71.00,
            -70.00, -69.00, -68.00, -67.00, -66.00, -65.00, -64.00, -63.00, -62.00, -61.00, -60.00,
            -59.00, -58.00, -57.00, -56.00, -55.00, -54.00, -53.00, -52.00, -51.00, -50.00, -49.00,
            -48.00, -47.00, -46.00, -45.00, -44.00, -43.00, -42.00, -41.00, -40.00, -39.00, -38.00,
            -37.00, -36.00, -35.00, -34.00, -33.00, -32.00, -31.00, -30.00, -29.00, -28.00, -27.00,
            -26.00, -25.00, -24.00, -23.00, -22.00, -21.00, -20.00, -19.00, -18.00, -17.00, -16.00,
            -15.00, -14.00, -13.00, -12.00, -11.00, -10.00, -9.00, -8.00, -7.00, -6.00, -5.00,
            -4.00, -3.00, -2.00, -1.00, 0.00, 1.00, 2.00, 3.00, 4.00, 5.00, 6.00, 7.00, 8.00, 9.00,
            10.00, 11.00, 12.00, 13.00, 14.00, 15.00, 16.00, 17.00, 18.00, 19.00, 20.00, 21.00,
            22.00, 23.00, 24.00, 25.00, 26.00, 27.00, 28.00, 29.00, 30.00, 31.00, 32.00, 33.00,
            34.00, 35.00, 36.00, 37.00, 38.00, 39.00, 40.00, 41.00, 42.00, 43.00, 44.00, 45.00,
            46.00, 47.00, 48.00, 49.00, 50.00, 51.00, 52.00, 53.00, 54.00, 55.00, 56.00, 57.00,
            58.00, 59.00, 60.00, 61.00, 62.00, 63.00, 64.00, 65.00, 66.00, 67.00, 68.00, 69.00,
            70.00, 71.00, 72.00, 73.00, 74.00, 75.00, 76.00, 77.00, 78.00, 79.00, 80.00, 81.00,
            82.00, 83.00, 84.00, 85.00, 86.00, 87.00, 88.00, 89.00, 90.00, 91.00, 92.00, 93.00,
            94.00, 95.00, 96.00, 97.00, 98.00, 99.00, 100.00, 101.00, 102.00, 103.00, 104.00,
            105.00, 106.00, 107.00, 108.00, 109.00, 110.00, 111.00, 112.00, 113.00, 114.00, 115.00,
            116.00, 117.00, 118.00, 119.00, 120.00,
        ];
        assert_eq!(e_ref_scan_positions.len(), scan.ref_scan_positions.len());
        for (a, b) in e_ref_scan_positions
            .iter()
            .zip(scan.ref_scan_positions.iter())
        {
            let delta = (a - b).abs();
            let check = delta < 2.0 * f64::EPSILON;
            assert!(check);
        }
        assert_eq!(1.00, scan.ref_overscan_factor);
        assert_eq!(CurveType::InplaneProfile, scan.scan_curvetype);
        assert_eq!(13.00, scan.scan_depth);
        assert_eq!(0.00, scan.scan_offaxis_inplane);
        assert_eq!(0.00, scan.scan_offaxis_crossplane);
        assert_eq!(0.00, scan.scan_angle);
        assert_eq!("NOT_DIAGONAL", scan.scan_diagonal.as_str());
        assert_eq!("POSITIVE", scan.scan_direction.as_str());
        assert_eq!("WATER", scan.meas_medium.as_str());
        assert_eq!("MEAS_CONTINUOUS_REFERENCE_SCAN", scan.meas_preset.as_str());
        assert_eq!(0.500, scan.meas_time);
        assert_eq!("A.U.", scan.meas_unit.as_str());
        assert_eq!(1005.64, scan.pressure);
        assert_eq!(22.23, scan.temperature);
        assert_eq!(20.00, scan.norm_temperature);
        assert_eq!(1.0000, scan.correction_factor);
        assert_eq!(3.00, scan.expected_max_dose_rate);
        assert_eq!(0.0, scan.epom_depth_shift);
        assert_eq!("TRUFIX", scan.epom_mode.as_str());
    }
}
