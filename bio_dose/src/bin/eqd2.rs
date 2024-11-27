use clap::Parser;
use rad_tools_bio_dose::eqd2;
use std::process::exit;
use tracing::{error, trace, Level};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Computes equivalent dose in 2 Gy fractions.",
    long_about = r#"
Computes equivalent dose in 2 Gy fractions:

EQD2 = D * ([d + a/b] / [2 + a/b])

where:
    - D: total dose in Gy.
    - d: dose per fraction in Gy
    - a/b: dose at which the linear and quadratic components of cell kill are equal in Gy
    "#
)]
struct Cli {
    /// Dose per fraction in Gy
    #[arg(short, long, value_name = "DOSE")]
    dose_per_fraction: f64,
    /// Total number of fractions
    #[arg(short = 'n', long, value_name = "TOTAL_NUMBER_FRACTIONS")]
    n_fractions: u32,
    /// Dose (Gy) at which the linear and quadratic components of cell kill are equal.
    #[arg(short = 'a', long, value_name = "ALPHA_BETA_RATIO")]
    alpha_beta_ratio: f64,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

fn main() {
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
        .with_line_number(true)
        .init();

    trace!("Commandline arguments: {:#?}", &cli);

    match eqd2(cli.dose_per_fraction, cli.n_fractions, cli.alpha_beta_ratio) {
        Ok(f) => {
            println!("EQD2: {:.4} Gy", f);
        }
        Err(e) => {
            error!("Error: {}", e);
            exit(1);
        }
    }
}
