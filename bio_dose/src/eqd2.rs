use crate::Error;
use tracing::{error, instrument};

/// Computes the equivalent dose in 2 Gy fractions (EQD2).
///
/// EQD2 is a way to compare different radiation therapy regimens by normalizing
/// them to an equivalent dose given in 2 Gy fractions. This is useful for understanding and
/// comparing the biological effect of different dosages and fractionation schemes.
///
/// # Arguments
///
/// * `d` - dose delivered per fraction (in Gy).
/// * `n` - total number of fractions.
/// * `ab` - Dose (Gy) at which the linear and quadratic components of cell kill are equal.
///
/// # Returns
///
/// * `Result<f64, Error>` - The equivalent dose in 2 Gy fractions, or an error if any of the input
///   parameters are invalid.
///
/// # Example
///
/// ```
/// use rad_tools_bio_dose::eqd2;
///
/// let value = eqd2(3.0, 20, 3.0).unwrap();
/// assert!((72.0 - value).abs() < 1e-6);
///
/// let value = eqd2(3.0, 20, 10.0).unwrap();
/// assert!((65.0 - value).abs() < 1e-6);
/// ```
///
///
/// # References
///
/// This calculation is based on the linear quadratic model commonly used in radiobiology.
/// More information can be found in [The linear-quadratic formula and progress in fractionated radiotherapy ](https://pubmed.ncbi.nlm.nih.gov/2670032/) and [21 years of Biologically Effective Dose](https://pmc.ncbi.nlm.nih.gov/articles/PMC3473681/)
///
#[instrument(level = "debug")]
pub fn eqd2(d: f64, n: u32, ab: f64) -> Result<f64, Error> {
    if d <= 0.0 {
        error!("Dose per fraction ({:#?}) must be greater than zero", d);
        return Err(Error::InvalidDosePerFraction);
    }
    if n == 0 {
        error!(
            "Total number of fractions ({:#?}) must be greater than zero",
            n
        );
        return Err(Error::InvalidTotalFractions);
    }
    if ab <= 0.0 {
        error!("Alpha beta ratio ({:#?}) must be greater than zero", ab);
        return Err(Error::InvalidAlphaBetaRatio);
    }

    Ok(n as f64 * d * (d + ab) / (2.0 + ab))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eqd2_valid_inputs() {
        assert!((eqd2(3.0, 20, 3.0).unwrap() - 72.0).abs() < 1e-6);
        assert!((eqd2(3.0, 20, 10.0).unwrap() - 65.0).abs() < 1e-6);
    }

    #[test]
    fn test_eqd2_invalid_dose_per_fraction() {
        match eqd2(0.0, 20, 3.0) {
            Err(Error::InvalidDosePerFraction) => (),
            _ => panic!("Expected InvalidDosePerFraction error"),
        }
        match eqd2(-1.0, 20, 3.0) {
            Err(Error::InvalidDosePerFraction) => (),
            _ => panic!("Expected InvalidDosePerFraction error"),
        }
    }

    #[test]
    fn test_eqd2_invalid_total_fractions() {
        match eqd2(3.0, 0, 3.0) {
            Err(Error::InvalidTotalFractions) => (),
            _ => panic!("Expected InvalidTotalFractions error"),
        }
    }

    #[test]
    fn test_eqd2_invalid_alpha_beta_ratio() {
        match eqd2(3.0, 20, 0.0) {
            Err(Error::InvalidAlphaBetaRatio) => (),
            _ => panic!("Expected InvalidAlphaBetaRatio error"),
        }
        match eqd2(3.0, 20, -1.0) {
            Err(Error::InvalidAlphaBetaRatio) => (),
            _ => panic!("Expected InvalidAlphaBetaRatio error"),
        }
    }
}
