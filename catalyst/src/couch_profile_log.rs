use std::{
    f64,
    fs::File,
    io::{BufRead, BufReader},
    num::{ParseFloatError, ParseIntError},
    path::Path,
};

use log::trace;

const PROFILE_SCAN_STARTED: &str = "Profile scan started ";

#[derive(thiserror::Error, Debug)]
pub enum CouchProfileLogError {
    #[error("Failed to read couch profile log file.")]
    IO(#[from] std::io::Error),
    #[error("Unable to parse string to integer")]
    ParseInt(#[from] ParseIntError),
    #[error("Unable to parse date time from log file.")]
    DateTimeParse(#[from] chrono::format::ParseError),
    #[error("Unable to parse string to float")]
    ParseFloat(#[from] ParseFloatError),
    #[error("Insufficient measurement data points to compute an average.")]
    InsufficientDataAverage,
}

/// Log containing the amplitude and couch position of a Catalyst calibration
/// with and without weight on the table.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CouchProfileLog {
    /// Date and time of the calibration
    pub date_time: chrono::NaiveDateTime,
    pub primary_x: f64,
    pub primary_y: f64,
    pub primary_z: f64,

    // Amplitude & couch position
    // No weights on the table
    pub(crate) ac_nw: Vec<(f64, f64)>,
    // Amplitude & couch position
    // Weights on the table
    pub(crate) ac_w: Vec<(f64, f64)>,
}

pub type Result<T> = std::result::Result<T, CouchProfileLogError>;

enum State {
    Header,
    NoWeight,
    Weight,
}

impl CouchProfileLog {
    /// Add an amplitude and a couch position into the no weight dataset.
    pub fn add_nw(&mut self, amplitude: f64, couch_pos: f64) {
        self.ac_nw.push((amplitude, couch_pos));
    }

    /// Add an amplitude and a couch position into the weight dataset.
    pub fn add_w(&mut self, amplitude: f64, couch_pos: f64) {
        self.ac_w.push((amplitude, couch_pos));
    }

    /// Get amplitude and couch position without weight on the table.
    pub fn ac_nw(&self) -> &Vec<(f64, f64)> {
        &self.ac_nw
    }

    /// Get amplitude and couch position with weight on the table.
    pub fn ac_w(&self) -> &Vec<(f64, f64)> {
        &self.ac_w
    }

    /// Read a log file from a [`Path`].
    pub fn read_file<P>(p: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = p.as_ref();
        let file = File::open(path)?;
        let rdr = BufReader::new(file);
        Self::read(rdr)
    }

    /// Read a log file from a [`BufRead`].
    pub fn read<R>(rdr: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut cpl = CouchProfileLog::default();
        trace!("Reading using BufRead.");

        let mut state = State::Header;
        let mut skip = 3;
        let lines = rdr.lines().map_while(std::result::Result::ok);
        for line in lines {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            match state {
                State::Header => {
                    if line.starts_with(PROFILE_SCAN_STARTED) {
                        trace!("Parsing profile scan date & time.");
                        let t = helper::profile_scan_timestamp(&line)?;
                        cpl.date_time = t;
                    } else if line.starts_with("Primary") {
                        if line.contains("X:") {
                            trace!("Parsing primary X.");
                            cpl.primary_x = helper::get_f64(&line)?;
                        }
                        if line.contains("Y:") {
                            trace!("Parsing primary Y.");
                            cpl.primary_y = helper::get_f64(&line)?;
                        }
                        if line.contains("Z:") {
                            trace!("Parsing primary Z.");
                            cpl.primary_z = helper::get_f64(&line)?;
                            skip = 3;
                            state = State::NoWeight;
                            trace!("Parsing amplitude and couch positions with no weight on the table.");
                        }
                    }
                }
                State::NoWeight => {
                    if line.starts_with("****") {
                        state = State::Weight;
                        trace!("Parsing amplitude and couch positions with weight on the table.");
                        continue;
                    }
                    let values = helper::csv_line_to_f64s(&line)?;
                    cpl.ac_nw.push(values);
                }
                State::Weight => {
                    let values = helper::csv_line_to_f64s(&line)?;
                    cpl.ac_w.push(values);
                }
            }
        }
        Ok(cpl)
    }
}

#[cfg(test)]
pub(crate) const TEST_COUCH_CALIB: &str = r#"
Log file created 7/10/2024 5:53:25 PM by user 'User1'.

-------------------------------------------
Profile scan started 7/10/2024 5:53:25 PM
Primary              X: 5
Primary              Y: -25
Primary              Z: 4.346352
-------------------------------------------
Amplitude; CouchPos
-------------------------------------------
0.20; 1118.99
0.17; 1118.97
0.12; 1118.96
0.13; 1118.94
0.13; 1118.93
0.05; 1118.93
0.13; 1118.93
0.16; 1118.94
0.14; 1118.96
0.19; 1118.97
0.17; 1118.97
0.17; 1118.97
0.16; 1118.97
**** END OF PROFILE WITH NO WEIGHT ***
0.91; 1523.60
-3.26; 1523.26
-3.29; 1523.28
-3.23; 1523.32
-3.25; 1523.34
-3.18; 1523.36
-3.10; 1523.36
-3.20; 1523.37
-3.16; 1523.36
-3.14; 1523.34
-3.22; 1523.32
-3.25; 1523.32
-3.20; 1523.31
-3.23; 1523.28
-3.23; 1523.28
-3.22; 1523.26
"#;

mod helper {
    use std::num::ParseFloatError;

    use chrono::{NaiveDate, NaiveDateTime};
    use log::debug;

    pub(crate) fn profile_scan_timestamp(
        s: &str,
    ) -> Result<NaiveDateTime, super::CouchProfileLogError> {
        let t = &s.trim()[super::PROFILE_SCAN_STARTED.len()..];
        debug!("t: {:#?}", t);
        let date_end = t.find(' ').unwrap();
        let time_end = t.rfind(' ').unwrap();
        let sd = &t[0..date_end];
        let st = &t[date_end + 1..time_end];

        let delim1 = sd.find('/').unwrap();
        let delim2 = sd.rfind('/').unwrap();

        let smonth = &sd[..delim1];
        let sday = &sd[delim1 + 1..delim2];
        let syear = &sd[delim2 + 1..];

        let delim1 = st.find(':').unwrap();
        let delim2 = st.rfind(':').unwrap();

        let shour = &st[..delim1];
        let smin = &st[delim1 + 1..delim2];
        let ssec = &st[delim2 + 1..];

        debug!("year: {:#?}", syear);
        debug!("month: {:#?}", smonth);
        debug!("day: {:#?}", sday);

        debug!("year: {:#?}", syear);
        debug!("month: {:#?}", smonth);
        debug!("day: {:#?}", smonth);

        let ampm = &t[time_end + 1..];

        let month = smonth.parse::<u32>()?;
        let day = sday.parse::<u32>()?;
        let year = syear.parse::<i32>()?;

        let mut hour = shour.parse::<u32>()?;
        let min = smin.parse::<u32>()?;
        let sec = ssec.parse::<u32>()?;

        if ampm == "PM" {
            hour += 12;
        }

        Ok(NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap())
    }

    pub(crate) fn get_f64(s: &str) -> Result<f64, ParseFloatError> {
        let t = s.trim();
        let p = s.rfind(": ").unwrap() + 2;
        t[p..].parse::<f64>()
    }

    pub(crate) fn csv_line_to_f64s(s: &str) -> Result<(f64, f64), ParseFloatError> {
        let mut split = s.split("; ");
        let s0 = split.next().unwrap();
        let s1 = split.next().unwrap();
        let f0 = s0.parse::<f64>()?;
        let f1 = s1.parse::<f64>()?;
        Ok((f0, f1))
    }

    #[cfg(test)]
    mod test {
        use chrono::NaiveDate;
        use log::debug;

        use crate::couch_profile_log::CouchProfileLog;

        #[allow(dead_code)]
        fn init() {
            let _ = env_logger::builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Trace)
                .try_init();
        }

        #[test]
        fn profile_scan_timestamp() {
            init();
            let s = "Profile scan started 7/10/2024 5:53:25 PM";
            let dt = super::profile_scan_timestamp(s);
            let expected = NaiveDate::from_ymd_opt(2024, 7, 10)
                .unwrap()
                .and_hms_opt(17, 53, 25)
                .unwrap();

            debug!("dt: {:#?}", &dt);
            debug!("expected: {:#?}", &expected);
            assert_eq!(expected, dt.unwrap());
        }

        #[test]
        fn get_f64() {
            let vs = [
                "Primary              X: 0",
                "Primary              Y: -25",
                "Primary              Z: 4.346352",
            ];
            let expected = [0.0, -25.0, 4.346352];
            let n = expected.len();
            assert_eq!(vs.len(), n);
            for i in 0..n {
                assert_eq!(super::get_f64(vs[i]).unwrap(), expected[i]);
            }
        }

        #[test]
        fn csv_line_to_f64s() {
            let vs = ["0.17; 1118.97", "0.12; 1118.96"];
            let expected = [(0.17, 1118.97), (0.12, 1118.96)];
            let n = expected.len();
            assert_eq!(vs.len(), n);
            for i in 0..n {
                assert_eq!(super::csv_line_to_f64s(vs[i]).unwrap(), expected[i]);
            }
        }

        #[test]
        fn read() {
            init();
            let rdr = std::io::Cursor::new(super::super::TEST_COUCH_CALIB);
            let r = CouchProfileLog::read(rdr);
            assert!(r.is_ok());
            let cpl = r.unwrap();
            debug!("{:#?}", &cpl);
            assert_eq!(
                &NaiveDate::from_ymd_opt(2024, 7, 10)
                    .unwrap()
                    .and_hms_opt(17, 53, 25)
                    .unwrap(),
                &cpl.date_time
            );
            assert_eq!(5.0, cpl.primary_x);
            assert_eq!(-25.0, cpl.primary_y);
            assert_eq!(4.346352, cpl.primary_z);

            let exp_nw: Vec<(f64, f64)> = vec![
                (0.20, 1118.99),
                (0.17, 1118.97),
                (0.12, 1118.96),
                (0.13, 1118.94),
                (0.13, 1118.93),
                (0.05, 1118.93),
                (0.13, 1118.93),
                (0.16, 1118.94),
                (0.14, 1118.96),
                (0.19, 1118.97),
                (0.17, 1118.97),
                (0.17, 1118.97),
                (0.16, 1118.97),
            ];
            #[allow(clippy::approx_constant)]
            let exp_w: Vec<(f64, f64)> = vec![
                (0.91, 1523.60),
                (-3.26, 1523.26),
                (-3.29, 1523.28),
                (-3.23, 1523.32),
                (-3.25, 1523.34),
                (-3.18, 1523.36),
                (-3.10, 1523.36),
                (-3.20, 1523.37),
                (-3.16, 1523.36),
                (-3.14, 1523.34),
                (-3.22, 1523.32),
                (-3.25, 1523.32),
                (-3.20, 1523.31),
                (-3.23, 1523.28),
                (-3.23, 1523.28),
                (-3.22, 1523.26),
            ];
            assert_eq!(&exp_nw, cpl.ac_nw());
            assert_eq!(&exp_w, cpl.ac_w());
        }
    }
}
