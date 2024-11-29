use clap::Parser;
use rad_tools_bio_dose::bed;
use std::process::exit;
use tracing::{error, trace, Level};
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Computes biologically equivalent dose (BED) in Gy.",
    long_about = r#"
Computes biologically equivalent dose (BED) in Gy.

BED = D * (1 + d/[a/b])

where:
    - D: total dose in Gy.
    - d: dose per fraction in Gy
    - a/b: dose at which the linear and quadratic components of cell kill are equal in Gy
    "#
)]
struct Cli {
    /// Dose per fraction in Gy
    #[arg(short, value_name = "DOSE")]
    d: f64,
    /// Total number of fractions
    #[arg(short = 'n', value_name = "TOTAL_NUMBER_FRACTIONS")]
    n: u32,
    /// Dose (Gy) at which the linear and quadratic components of cell kill are equal.
    #[arg(short = 'a', value_name = "ALPHA_BETA_RATIO")]
    ab: f64,
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

    match bed(cli.d, cli.n, cli.ab) {
        Ok(f) => {
            println!("{:.6}", f);
        }
        Err(e) => {
            error!("Error: {}", e);
            exit(1);
        }
    }
}
