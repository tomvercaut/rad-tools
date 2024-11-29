use crate::Error;
use tracing::{error, instrument};

/// Calculates the Biologically Effective Dose (BED) based on given parameters.
///
/// This function performs a calculation for the BED, which is used in radiotherapy
/// to compare different fractionation regimens. The BED helps to estimate the
/// biological effect of a given dose of radiotherapy.
///
/// # Parameters
///
/// - `d`: dose delivered per fraction (Gy).
/// - `n`: total number of fractions
/// * `ab` - Dose (Gy) at which the linear and quadratic components of cell kill are equal.
///
/// # Returns
///
/// Returns a `Result` containing the calculated BED value if all parameters are valid.
/// Returns an `Error` if any parameter is invalid (e.g., negative or zero values).
///
/// # Errors
///
/// - `Error::InvalidDosePerFraction`: Returned if `d` is not positive.
/// - `Error::InvalidTotalFractions`: Returned if `n` is zero.
/// - `Error::InvalidAlphaBetaRatio`: Returned if `ab` is not positive.
///
/// # Example
///
/// ```rust
/// use rad_tools_bio_dose::bed;
///
/// fn main() {
///     let dose_per_fraction = 2.0;
///     let total_fractions = 30;
///     let alpha_beta_ratio = 10.0;
///
///     match bed(dose_per_fraction, total_fractions, alpha_beta_ratio) {
///         Ok(bed) => println!("Calculated BED: {}", bed),
///         Err(e) => eprintln!("Error calculating BED: {:?}", e),
///     }
/// }
/// ```
///
/// # References
///
/// For more information about the Biologically Effective Dose, see the article:
/// "The Linear-Quadratic Formula and Fractionation in Radiotherapy" by Fowler (1989).
///
#[instrument(level = "debug")]
pub fn bed(d: f64, n: u32, ab: f64) -> Result<f64, Error> {
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

    Ok(n as f64 * d * (1.0 + d / ab))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bed_valid_input() {
        let dose_per_fraction = 3.0;
        let total_fractions = 20;
        let abs = [1.0, 2.0, 3.0, 6.0, 10.0];
        let expected = [240.0, 150.0, 120.0, 90.0, 78.0];
        let n = abs.len();
        assert_eq!(n, expected.len());
        for i in 0..n {
            let value = bed(dose_per_fraction, total_fractions, abs[i]).unwrap();
            assert!((value - expected[i]).abs() < 1e-6);
        }
    }

    #[test]
    fn test_bed_invalid_dose_per_fraction() {
        let dose_per_fraction = 0.0;
        let total_fractions = 30;
        let alpha_beta_ratio = 10.0;

        let result = bed(dose_per_fraction, total_fractions, alpha_beta_ratio);
        assert!(matches!(result, Err(Error::InvalidDosePerFraction)));
    }

    #[test]
    fn test_bed_invalid_total_fractions() {
        let dose_per_fraction = 2.0;
        let total_fractions = 0;
        let alpha_beta_ratio = 10.0;

        let result = bed(dose_per_fraction, total_fractions, alpha_beta_ratio);
        assert!(matches!(result, Err(Error::InvalidTotalFractions)));
    }

    #[test]
    fn test_bed_invalid_alpha_beta_ratio() {
        let dose_per_fraction = 2.0;
        let total_fractions = 30;
        let alpha_beta_ratio = 0.0;

        let result = bed(dose_per_fraction, total_fractions, alpha_beta_ratio);
        assert!(matches!(result, Err(Error::InvalidAlphaBetaRatio)));
    }
}
