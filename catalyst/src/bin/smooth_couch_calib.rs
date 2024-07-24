use std::str::FromStr;

use catalyst::algo::{self, CmaWidth};
use catalyst::CouchProfileLog;
use chrono::NaiveDateTime;
use clap::Parser;

/// Smooth the catalyst couch calibration data.
///
/// Smoothing the catalyst couch calibration data is done by computing a central moving average of
/// the input.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = r#"
Smooth the catalyst couch calibration data.

Smoothing the catalyst couch calibration data is done by computing a central moving average of the input.
"#
)]
struct Cli {
    /// Input couch calibration file.
    #[arg(short, long)]
    input: String,

    /// Output CSV couch calibration file.
    #[arg(short, long)]
    output: String,

    /// Central moving average width for the no weight measurements
    ///
    /// Width can be specified in absolute values (e.g. 3) or as a percentage of all the
    /// measurement points.
    ///
    /// The format for percentages is:
    /// - 0.02% => 2 percent
    /// - 0.02p => 2 percent
    #[arg(long)]
    cma_nw: String,
    /// Central moving average width for the weight data measurements
    ///
    /// Width can be specified in absolute values (e.g. 3) or as a percentage of all the
    /// measurement points.
    ///
    /// The format for percentages is:
    /// - 0.02% => 2 percent
    /// - 0.02p => 2 percent
    #[arg(long)]
    cma_w: String,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn time_stamp_str(dt: &NaiveDateTime) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.verbose {
        let _ = env_logger::builder()
            .is_test(false)
            .filter_level(log::LevelFilter::Trace)
            .try_init();
    }

    let cpl = CouchProfileLog::read_file(&cli.input)?;

    let avg_nw_width = CmaWidth::from_str(&cli.cma_nw)?;
    let avg_w_width = CmaWidth::from_str(&cli.cma_w)?;

    let cpl_avg = algo::central_moving_average(&cpl, avg_nw_width, avg_w_width)
        .expect("Failed to compute the central moving average.");

    let mut wtr = csv::Writer::from_path(cli.output)?;
    wtr.write_record([time_stamp_str(&cpl.date_time).as_str(), "", "", ""])?;
    wtr.write_record(["Primary X", &cpl.primary_x.to_string(), "", ""])?;
    wtr.write_record(["Primary Y", &cpl.primary_y.to_string(), "", ""])?;
    wtr.write_record(["Primary Z", &cpl.primary_z.to_string(), "", ""])?;
    wtr.write_record(["No weight", "", "With weight", ""])?;
    wtr.write_record(["Amplitude", "CouchPosition", "Amplitude", "CouchPosition"])?;

    let ac_nw = cpl_avg.ac_nw();
    let ac_w = cpl_avg.ac_w();
    let n_nw = ac_nw.len();
    let n_w = ac_w.len();

    let n = if n_w > n_nw { n_w } else { n_nw };

    for i in 0..n {
        if i < n_nw {
            wtr.write_field(ac_nw[i].0.to_string())?;
            wtr.write_field(ac_nw[i].1.to_string())?;
        } else {
            wtr.write_field("")?;
            wtr.write_field("")?;
        }
        if i < n_w {
            wtr.write_field(ac_w[i].0.to_string())?;
            wtr.write_field(ac_w[i].1.to_string())?;
        } else {
            wtr.write_field("")?;
            wtr.write_field("")?;
        }
        wtr.write_record(None::<&[u8]>)?;
    }

    wtr.flush()?;
    Ok(())
}
