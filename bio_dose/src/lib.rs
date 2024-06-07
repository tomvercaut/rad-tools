use core::f64;
use tracing::trace;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BioDoseError {
    #[error("a/b ratio [{0}] can't be a negative number")]
    NegativeAB(f64),
    #[error("Dose per fraction [{0}] can't be a negative number")]
    NegativeDosePerFraction(f64),
}

/// Parameters required to compute the equivalent dose
/// in 2 Gy fractions.
#[derive(Debug, Clone, Copy)]
pub struct Eqd2Params {
    /// Dose per fraction (Gy)
    pub d: f64,
    /// Number of fractions
    pub n: usize,
    /// Dose (Gy) (a/b) at which the lineair and quadratic compoment of cell kill are equal.
    pub ab: f64,
}

/// Parameters required to compute the biologically equivalent
/// dose for a tissue with a (well) defined a/b ratio.
#[derive(Debug, Clone, Copy)]
pub struct BedParams {
    /// Dose per fraction (Gy)
    pub d: f64,
    /// Number of fractions
    pub n: usize,
    /// Dose (Gy) (a/b) at which the lineair and quadratic compoment of cell kill are equal.
    pub ab: f64,
    /// Model
    pub model: BedModel,
}

/// Model with additional parameters to computer the BED.
#[derive(Debug, Clone, Copy, Default)]
pub enum BedModel {
    /// No additional parameters or corrections applied.
    #[default]
    None,
    LQTimeFactor(LQModelTimeFactor),
    LQL(LQLModel),
}

/// Linear quadratic model taking into account
/// a time factor for cell repopulation.
#[derive(Debug, Clone, Copy, Default)]
pub struct LQModelTimeFactor {
    /// Overall treatment time in days
    pub t: usize,
    /// Repopulation doesn't start until day `tk`.
    /// k is the kick-off for the delayed repopulation during irradiation.
    pub tk: usize,
    /// Letal damage inflicted with a single ionizing event producing a double strand DNA break.
    /// Unit is Gy.
    pub a: f64,
    /// Constant cell doubling time up to the end of the radiation treatment.
    pub tp: f64,
}

/// Linear quadratic linear model which can be applied in high dose fractions.
///
/// More information can be found in:
/// [1] The modified linear-quadratic model of Guerrero and Li can be derived from a mechanistic basis and exhibits linear-quadratic-linear behaviour.
///     Marco Carlone1, David Wilkins1 and Peter Raaphorst1
///     Physics in Medicine & Biology, Volume 50, Number 10 Citation Marco Carlone et al 2005 Phys. Med. Biol. 50 L9 DOI 10.1088/0031-9155/50/10/L01
///
/// [2] Some implications of linear-quadratic-linear radiation dose-response with regard to hypofractionation.
///     Melvin Astrahan
///     https://doi.org/10.1118/1.2969065
///
#[derive(Debug, Clone, Copy, Default)]
pub struct LQLModel {
    /// Letal damage inflicted with a single ionizing event producing a double strand DNA break.
    /// Unit is Gy.
    pub a: f64,
    /// Linear coefficient for the (final) linear part of the survival curve.
    pub g: f64,
    /// Transition dose at which LQ-L behaviour starts
    pub dt: f64,
}

/// Computes the EQD2.
///
/// ```math
/// EQD_{2} = D * \frac{d+\frac{a}{b}}{2+\frac{a}{b}}
/// ```
pub fn eqd2(p: &Eqd2Params) -> Result<f64, BioDoseError> {
    if p.ab < 0.0 {
        return Err(BioDoseError::NegativeAB(p.ab));
    }
    if p.d < 0.0 {
        return Err(BioDoseError::NegativeDosePerFraction(p.d));
    }

    let td = p.d * p.n as f64;
    let eq = td * ((p.d + p.ab) / (2.0 + p.ab));
    Ok(eq)
}

/// Compute the biological equivalent dose.
pub fn bed(p: &BedParams) -> Result<f64, BioDoseError> {
    trace!("BED parameters: {:#?}", p);
    if p.ab < 0.0 {
        return Err(BioDoseError::NegativeAB(p.ab));
    }
    if p.d < 0.0 {
        return Err(BioDoseError::NegativeDosePerFraction(p.d));
    }
    let mut bed = p.n as f64 * p.d * (1.0 + p.d / p.ab);
    match &p.model {
        BedModel::LQTimeFactor(btf) => {
            trace!("BED before time factor: {}", bed);
            let ln2 = (2.0f64).ln();
            let ttk = btf.t as f64 - btf.tk as f64;
            let z = ln2 * ttk / (btf.a * btf.tp);
            trace!("BED time factor: {}", z);
            bed -= z;
            trace!("BED with time factor: {}", bed);
        }
        BedModel::LQL(lq_l) => {
            trace!("BED before LQ-L model: {}", bed);
            if p.d < lq_l.dt {
                trace!("d[{}] < dt[{}] -> BED not corrected", p.d, lq_l.dt);
            } else {
                let corr = p.n as f64 * (lq_l.g / lq_l.a) * (p.d - lq_l.dt);
                trace!("LQ-L correction: {}", corr);
                bed += corr;
            }
        }
        BedModel::None => {}
    }

    Ok(bed)
}

#[cfg(test)]
mod tests {
    use crate::{bed, eqd2, BedModel, BedParams, BioDoseError, Eqd2Params, LQModelTimeFactor};

    #[test]
    fn eqd2_expected_ok() {
        let params = &[
            Eqd2Params {
                d: 5.0,
                n: 6,
                ab: 3.0,
            },
            Eqd2Params {
                d: 2.0,
                n: 10,
                ab: 3.0,
            },
        ];
        let expected = &[48.0, 20.0];
        let nexpected = expected.len();
        for i in 0..nexpected {
            assert_eq!(nexpected, params.len());
            let r = eqd2(&params[i]);
            assert!(r.is_ok());
            let eq = r.unwrap();
            assert_eq!(expected[i], eq);
        }
    }

    #[test]
    fn eqd2_neg_nf() {
        let r = eqd2(&Eqd2Params {
            d: -5.0,
            n: 6,
            ab: 3.0,
        });
        assert!(r.is_err());
        let e = r.err().unwrap();
        assert_eq!(BioDoseError::NegativeDosePerFraction(-5.0), e);
    }

    #[test]
    fn eqd2_neg_ab() {
        let r = eqd2(&Eqd2Params {
            d: 5.0,
            n: 6,
            ab: -3.0,
        });
        assert!(r.is_err());
        let e = r.err().unwrap();
        assert_eq!(BioDoseError::NegativeAB(-3.0), e);
    }

    #[test]
    fn bed_expected_ok() {
        let params = &[
            BedParams {
                d: 3.0,
                n: 20,
                ab: 3.0,
                model: BedModel::None,
            },
            BedParams {
                d: 3.0,
                n: 20,
                ab: 2.0,
                model: BedModel::None,
            },
        ];
        let expected = &[120.0, 150.0];
        let nexpected = expected.len();
        for i in 0..nexpected {
            assert_eq!(nexpected, params.len());
            let r = bed(&params[i]);
            assert!(r.is_ok());
            let eq = r.unwrap();
            assert_eq!(expected[i], eq);
        }
    }

    #[test]
    fn bed_neg_nf() {
        let r = bed(&BedParams {
            d: -5.0,
            n: 6,
            ab: 3.0,
            model: BedModel::None,
        });
        assert!(r.is_err());
        let e = r.err().unwrap();
        assert_eq!(BioDoseError::NegativeDosePerFraction(-5.0), e);
    }

    #[test]
    fn bed_neg_ab() {
        let r = bed(&BedParams {
            d: 5.0,
            n: 6,
            ab: -3.0,
            model: BedModel::None,
        });
        assert!(r.is_err());
        let e = r.err().unwrap();
        assert_eq!(BioDoseError::NegativeAB(-3.0), e);
    }

    #[test]
    fn bed_time_factor() {
        let params = &[
            BedParams {
                d: 1.2,
                n: 68,
                ab: 10.0,
                model: BedModel::LQTimeFactor(LQModelTimeFactor {
                    t: 45,
                    tk: 21,
                    a: 0.35,
                    tp: 3.0,
                }),
            },
            BedParams {
                d: 1.2,
                n: 68,
                ab: 10.0,
                model: BedModel::LQTimeFactor(LQModelTimeFactor {
                    t: 40,
                    tk: 21,
                    a: 0.35,
                    tp: 3.0,
                }),
            },
            BedParams {
                d: 1.2,
                n: 68,
                ab: 10.0,
                model: BedModel::LQTimeFactor(LQModelTimeFactor {
                    t: 45,
                    tk: 11,
                    a: 0.35,
                    tp: 3.0,
                }),
            },
            BedParams {
                d: 1.2,
                n: 68,
                ab: 10.0,
                model: BedModel::LQTimeFactor(LQModelTimeFactor {
                    t: 45,
                    tk: 21,
                    a: 0.5,
                    tp: 3.0,
                }),
            },
            BedParams {
                d: 1.2,
                n: 68,
                ab: 10.0,
                model: BedModel::LQTimeFactor(LQModelTimeFactor {
                    t: 45,
                    tk: 21,
                    a: 0.35,
                    tp: 5.0,
                }),
            },
        ];
        let expected = &[
            75.54863587,
            78.84933673,
            68.94723415,
            80.30164511,
            81.88598152,
        ];
        let nexpected = expected.len();
        for i in 0..nexpected {
            assert_eq!(nexpected, params.len());
            let r = bed(&params[i]);
            assert!(r.is_ok());
            let eq = r.unwrap();
            assert!((expected[i] - eq).abs() < 1e-5);
        }
    }
}
