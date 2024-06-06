use anyhow::Result;
use clap::{Args, Parser};
use rad_tools_bio_dose::{bed, BedModel, BedParams, LQModelTimeFactor};
use tracing::{debug, Level};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Compute the biologically effective dose for a tissue with a well defined alpha/beta ratio taking into account an LQ model with a time factor.",
    long_about = ""
)]
struct Cli {
    /// Dose per fraction (Gy)
    fraction_dose: f64,
    /// Number of fractions
    fractions: usize,
    /// Dose (Gy) (a/b) at which the lineair and quadratic compoment of cell kill are equal.
    ab: f64,
    #[command(flatten)]
    time_factor: Option<TimeFactor>,
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct TimeFactor {
    /// Overall treatment time in days
    #[arg(long)]
    pub t: usize,
    /// Repopulation doesn't start until day `tk`.
    /// k is the kick-off for the delayed repopulation during irradiation.
    #[arg(long)]
    pub tk: usize,
    /// Letal damage inflicted with a single ionizing event producing a double strand DNA break.
    /// Unit is Gy.
    #[arg(long)]
    pub a: f64,
    /// Constant cell doubling time up to the end of the radiation treatment.
    #[arg(long)]
    pub tp: f64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let level = if cli.trace {
        Level::TRACE
    } else if cli.debug {
        Level::DEBUG
    } else {
        Level::WARN
    };
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    let mut p = BedParams {
        d: cli.fraction_dose,
        n: cli.fractions,
        ab: cli.ab,
        model: BedModel::None,
    };
    if cli.time_factor.is_some() {
        let tf = cli.time_factor.as_ref().unwrap();
        let btf = LQModelTimeFactor {
            t: tf.t,
            tk: tf.tk,
            a: tf.a,
            tp: tf.tp,
        };
        p.model = BedModel::LQTimeFactor(btf);
    }
    let eq = bed(&p)?;
    debug!("Number of fractions: {}", p.n);
    debug!("Dose per fraction (Gy): {}", p.d);
    debug!("Total dose (Gy): {}", p.d * p.n as f64);
    debug!("BED dose (Gy): {}", eq);
    println!("{}", eq);
    Ok(())
}
