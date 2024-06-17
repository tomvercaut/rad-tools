#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use peeteewee::data::{DetectorType, TaskType};

    #[test]
    pub fn read_xcc_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("resources");
        d.push("octavius_1500.xcc");

        let result = peeteewee::io::xcc::read(d);
        if result.is_err() {
            let err = result.as_ref().err().unwrap();
            eprintln!("Error: {:?}", err);
        }
        assert!(result.is_ok());
        let xcc = result.unwrap();

        assert_eq!("1.1.0.0", xcc.version.as_str());
        assert_eq!("2023-12-05 17:59:39", xcc.last_modified.as_str());

        let content = &xcc.content;
        assert_eq!(
            TaskType::Measurement2dArray,
            content.administrative.task_name
        );
        assert_eq!("VeriSoft 8.0.1.0", &content.administrative.module);
        assert_eq!(
            "2023-12-05 17:57:22.219",
            &content.administrative.measurement_date
        );

        assert_eq!(
            "ptwRURotationDirection_SameAsLinac",
            content.rotation_unit.rotation_direction
        );
        assert_eq!(
            "ptwInclinometerMounting_Normal",
            content.rotation_unit.inclinometer_mounting
        );

        assert_eq!(
            "PTW_GANTRYUPRIGHT_0",
            content.accelerator_settings.gantry_upright_position
        );
        assert_eq!(
            "PTW_GANTRYROTATION_CW",
            content.accelerator_settings.gantry_rotation_direction
        );

        let inclinometer = &content.inclinometer;
        assert_eq!("", inclinometer.inclinometer_sn);
        assert_eq!(
            "2023-12-05 17:57:22.219",
            inclinometer.measurement_date_first_angle
        );

        for x in &inclinometer.angle_values {
            assert!(*x >= 0.0 && *x <= 360.0);
        }
        let n = inclinometer.angle_times.len();
        for i in 1..n {
            assert!(
                inclinometer.angle_times.get(i - 1).unwrap()
                    < inclinometer.angle_times.get(i).unwrap()
            );
        }

        let measuring_device = &content.measuring_device;
        assert_eq!(DetectorType::Octavius1500, measuring_device.detector);
        assert_eq!(
            "220012001_OCTAVIUS1500_112372.cal",
            &measuring_device.detector_calibration_file_name
        );
        assert_eq!(
            "PTW_ELECTROMETER_DETECTOR_INTERFACE",
            &measuring_device.electrometer
        );
        assert_eq!("5716", &measuring_device.electrometer_sn);
        assert_eq!("112372", &measuring_device.detector_sn);
        assert_eq!("PTW_DEVICE_ROTATIONUNIT", &measuring_device.scan_device);

        assert_eq!(0.0, content.measurement_parameters.scan_depth);

        let corr = &content.correction;
        assert_eq!(1013.25, corr.air_density_pressure);
        assert_eq!(20.0, corr.air_density_temperature);
        assert_eq!(1013.25, corr.air_density_reference_pressure);
        assert_eq!(20.0, corr.air_density_reference_temperature);
        assert_eq!(1.0, corr.energy);
        assert_eq!(1.006, corr.k_user);
        assert_eq!("PTW_CORR_AIRDENSITY PTW_CORR_FACTOR", &corr.flags);
        assert_eq!("0;0;0;0;0", &corr.system_sync_add);

        let da = &content.detector_array;
        let ndn = 1405usize;
        assert_eq!("DoseAccumulated", &da.device_store_mode);
        assert_eq!(ndn, da.detector_numbers.len());
        for i in 0..da.detector_numbers.len() {
            assert_eq!((i + 1) as u16, *da.detector_numbers.get(i).unwrap());
        }
        assert_eq!(-130.0, da.matrix_left_coordinate);
        assert_eq!(130.0, da.matrix_gun_coordinate);
        assert_eq!(5.0, da.matrix_resolution_lr);
        assert_eq!(5.0, da.matrix_resolution_gt);
        assert_eq!(27, da.matrix_number_of_meas_lr);
        assert_eq!(27, da.matrix_number_of_meas_gt);
        assert_eq!(5.0, da.chamber_dimension_lr);
        assert_eq!(5.0, da.chamber_dimension_gt);

        let meas_data = &content.measurement_data.measurements;
        assert!(!meas_data.is_empty());
        let mut meas = meas_data.first().unwrap();
        assert!((meas.time - 0.2).abs() < f64::EPSILON);
        assert!((meas.angle - 329.8).abs() < f64::EPSILON);
        assert_eq!(ndn, meas.data.len());
        meas = meas_data.get(1).unwrap();
        assert!((meas.time - 0.4).abs() < f64::EPSILON);
        assert!((meas.angle - 329.8).abs() < f64::EPSILON);
        assert_eq!(ndn, meas.data.len());
    }

    #[test]
    pub fn octavius1500_xcc_file_to_detector() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("resources");
        d.push("octavius_1500.xcc");
        println!("{:?}", d);

        let result = peeteewee::io::xcc::read(d);
        if result.is_err() {
            let err = result.as_ref().err().unwrap();
            eprintln!("Error: {:?}", err);
        }
        assert!(result.is_ok());
        let xcc = result.unwrap();

        let res_oct = peeteewee::data::Octavius1500::new(&xcc, false);
        assert!(res_oct.is_ok());
    }
}
