use std::{f64, str::FromStr};

use log::trace;

use crate::couch_profile_log::{CouchProfileLog, CouchProfileLogError};

/// Width of the central moving average filter.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmaWidth {
    /// Width of the CMA
    Absolute(usize),
    /// Width of the CMA filter is in function of the percentage.
    ///
    /// - 0.01 => 1 percent
    /// - 0.01 => 2 percent
    /// - 1.00 => 100 percent
    Percent(f64),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CmaWidthError {
    #[error("Negative numeric value is not allowed.")]
    Negative,
    #[error("Unable to parse data into an integer")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Unable to parse data into an floating point")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("Percentage are expected to be with the range [0.0 .. 1.0]")]
    PercentageRange,
}

impl FromStr for CmaWidth {
    type Err = CmaWidthError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        trace!("s: {:#?}", s);
        let t = s.trim();
        trace!("t: {:#?}", t);
        if t.starts_with('-') {
            Err(CmaWidthError::Negative)
        } else if t.ends_with('%') || t.ends_with('p') {
            let u = &t[0..t.len() - 1];
            trace!("u: {:#?}", u);
            let f = u.parse::<f64>()?;
            if !(0.0..=1.0).contains(&f) {
                Err(CmaWidthError::PercentageRange)
            } else {
                Ok(Self::Percent(f))
            }
        } else {
            let f = t.parse::<usize>()?;
            Ok(Self::Absolute(f))
        }
    }
}

/// Applies a central moving average (CMA) filter over the data.
///
/// More information on the filter can be found on [Wikipedia](https://en.wikipedia.org/wiki/Moving_average)
///
/// Boundary conditions for n data points with a CMA filter width (2 * k + 1):
/// - 0..=k: filter takes into account the indices that are not negative
/// - -k=..\<index\>..=k: all values are taken into account
/// - k-1..n: filter takes into account all indices that are not exceed the length of the data
///
pub fn central_moving_average(
    cpl: &CouchProfileLog,
    avg_nw_len: CmaWidth,
    avg_w_len: CmaWidth,
) -> Result<CouchProfileLog, CouchProfileLogError> {
    let mut avg = CouchProfileLog {
        date_time: cpl.date_time,
        primary_x: cpl.primary_x,
        primary_y: cpl.primary_y,
        primary_z: cpl.primary_z,
        ..Default::default()
    };
    avg.ac_nw = central_moving_average_vec(&cpl.ac_nw, avg_nw_len)?;
    avg.ac_w = central_moving_average_vec(&cpl.ac_w, avg_w_len)?;
    Ok(avg)
}

pub fn central_moving_average_vec(
    v: &[(f64, f64)],
    width: CmaWidth,
) -> Result<Vec<(f64, f64)>, CouchProfileLogError> {
    let n = v.len();
    trace!("CmaWidth: {:#?}", &width);
    let width = match width {
        CmaWidth::Absolute(w) => w,
        CmaWidth::Percent(p) => (p * n as f64) as usize,
    };
    let r = (width as f64 / 2.0).floor() as usize;
    let m = r * 2 + 1;
    trace!("n={:#?}", n);
    trace!("r={:#?}", r);
    trace!("m={:#?}", m);
    if m > n {
        return Err(CouchProfileLogError::InsufficientDataAverage);
    }
    let mut w = vec![];

    let add_pairs = |avg: &mut (f64, f64), a: usize, b: usize, v: &[(f64, f64)]| {
        avg.0 += v[a].0;
        avg.1 += v[a].1;

        avg.0 += v[b].0;
        avg.1 += v[b].1;
    };

    for i in 0..r {
        //trace!("i={:?}", i);
        let mut avg = v[i];
        let mut count = 1;
        for j in 1..r + 1 {
            let o = i.checked_sub(j);
            if o.is_none() {
                continue;
            }
            add_pairs(&mut avg, o.unwrap(), i + j, v);
            count += 1;
        }
        avg.0 /= count as f64;
        avg.1 /= count as f64;
        w.push(avg);
    }

    for i in r..(n - r) {
        //trace!("i={:?}", i);
        let mut avg = v[i];
        for j in 1..r + 1 {
            add_pairs(&mut avg, i - j, i + j, v);
        }
        avg.0 /= m as f64;
        avg.1 /= m as f64;
        w.push(avg);
    }

    for i in (n - r)..n {
        //trace!("i={:?}", i);
        let mut avg = v[i];
        let mut count = 1;
        for j in 1..r + 1 {
            let o = i.checked_add(i + j);
            if o.is_none() {
                continue;
            }
            let b = o.unwrap();
            if b >= n {
                continue;
            }
            add_pairs(&mut avg, i - j, b, v);
            count += 1;
        }
        avg.0 /= count as f64;
        avg.1 /= count as f64;
        w.push(avg);
    }

    Ok(w)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use log::debug;

    use crate::algo::{CmaWidth, CmaWidthError};

    #[allow(dead_code)]
    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Trace)
            .try_init();
    }

    #[test]
    fn cma_width_from_str() {
        init();
        let values = &[CmaWidth::from_str("2"), CmaWidth::from_str("0.2p")];

        let expected = &[CmaWidth::Absolute(2), CmaWidth::Percent(0.2)];
        assert_eq!(expected.len(), values.len());
        for (r, e) in values.iter().zip(expected.iter()) {
            assert!(r.is_ok());
            let w = r.as_ref().unwrap();
            assert_eq!(e, w);
        }
    }

    #[test]
    fn cma_width_from_str_expect_errors() {
        init();
        let values = &[CmaWidth::from_str("2p"), CmaWidth::from_str("-0.2p")];

        let expected = &[CmaWidthError::PercentageRange, CmaWidthError::Negative];
        assert_eq!(expected.len(), values.len());
        for (r, e) in values.iter().zip(expected.iter()) {
            assert!(r.is_err());
            let w = r.as_ref().unwrap_err();
            assert_eq!(e, w);
        }
    }

    #[test]
    fn central_moving_average_vec() {
        init();
        let input = &[
            (0.0, 0.0),
            (1.0, 1.0),
            (2.0, 2.0),
            (3.0, 3.0),
            (4.0, 4.0),
            (5.0, 5.0),
            (6.0, 6.0),
            (7.0, 7.0),
            (8.0, 8.0),
            (9.0, 9.0),
            (10.0, 10.0),
        ];
        let expected = &[
            (0.5, 0.5),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (1.0, 1.0),
            (9.5, 9.5),
        ];
        let output = super::central_moving_average_vec(input, CmaWidth::Absolute(3)).unwrap();
        let n = expected.len();
        debug!("{:#?}", output);
        assert_eq!(n, output.len());
        assert_eq!(input.len(), output.len());
    }
}
