#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[path = "shared/args.rs"]
mod args;

use clap::Parser;
use rad_tools_dosecheck_pdf_mv::mv_dosecheck_pdfs_watch;
use tracing_subscriber::EnvFilter;

fn init_tracing() -> anyhow::Result<()> {
    let builder = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_file(true)
        .with_line_number(true);
    #[cfg(target_os = "windows")]
    {
        use tracing_appender::rolling;
        let log_path = std::env::current_exe()?.with_file_name("logs");
        let appender = rolling::RollingFileAppender::builder()
            .rotation(rolling::Rotation::DAILY)
            .filename_prefix("dosecheck_pdf_mv.log")
            .build(log_path)?;
        builder.with_ansi(false).with_writer(appender).init();
    }

    #[cfg(not(target_os = "windows"))]
    {
        builder.with_ansi(true).init();
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_tracing()?;

    let args = args::Args::parse();
    args.validate()?;
    mv_dosecheck_pdfs_watch(&args.input, &args.output, args.watch)?;
    Ok(())
}
