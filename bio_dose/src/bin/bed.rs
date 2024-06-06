use anyhow::Result;
use clap::Parser;
use rad_tools_bio_dose::{bed, BedModel, BedParams};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Compute the biologically effective dose for a tissue with a well defined alpha/beta ratio.",
    long_about = ""
)]
struct Cli {
    /// Dose per fraction (Gy)
    fraction_dose: f64,
    /// Number of fractions
    fractions: usize,
    /// Dose (Gy) (a/b) at which the lineair and quadratic compoment of cell kill are equal.
    ab: f64,
    /// Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let p = BedParams {
        d: cli.fraction_dose,
        n: cli.fractions,
        ab: cli.ab,
        model: BedModel::None,
    };
    let eq = bed(&p)?;
    if cli.verbose {
        println!("Number of fractions: {}", p.n);
        println!("Dose per fraction (Gy): {}", p.d);
        println!("Total dose (Gy): {}", p.d * p.n as f64);
        println!("BED dose (Gy): {}", eq);
    } else {
        println!("{}", eq);
    }
    Ok(())
}
