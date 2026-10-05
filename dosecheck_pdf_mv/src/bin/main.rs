#![cfg_attr(
    all(
        feature = "WindowsDaemon",
        target_os = "windows",
        not(debug_assertions)
    ),
    windows_subsystem = "windows"
)]

#[path = "shared/args.rs"]
mod args;

use clap::Parser;
use rad_tools_dosecheck_pdf_mv::mv_dosecheck_pdfs_watch;
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false)
        .init();
}

fn main() -> anyhow::Result<()> {
    init_tracing();

    let args = args::Args::parse();
    args.validate()?;
    mv_dosecheck_pdfs_watch(&args.input, &args.output, args.watch)?;
    Ok(())
}
