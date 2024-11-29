mod bed;
pub use bed::bed;
mod eqd2;
pub use eqd2::eqd2;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Dose per fraction must be greater than zero")]
    InvalidDosePerFraction,
    #[error("Total number of fractions must be greater than zero")]
    InvalidTotalFractions,
    #[error("Alpha beta ratio must be greater than zero")]
    InvalidAlphaBetaRatio,
}
