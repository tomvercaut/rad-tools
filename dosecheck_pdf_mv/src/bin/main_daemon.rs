#[path = "shared/args.rs"]
mod args;

use clap::Parser;
use rad_tools_dosecheck_pdf_mv::mv_dosecheck_pdfs_watch;
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    let builder = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_file(true)
        .with_line_number(true);
    #[cfg(all(
        feature = "WindowsDaemon",
        target_os = "windows",
        not(debug_assertions)
    ))]
    {
        use tracing_appender::rolling;
        let log_path = std::env::current_exe().unwrap().join("logs");
        let appender = rolling::daily(log_path, "dosecheck_pdf_mv.log");
        builder.with_ansi(false).with_writer(appender).init();
    }

    #[cfg(not(all(
        feature = "WindowsDaemon",
        target_os = "windows",
        not(debug_assertions)
    )))]
    {
        builder.with_ansi(true).init();
    }
}

fn main() -> anyhow::Result<()> {
    init_tracing();

    let args = args::Args::parse();
    args.validate()?;
    mv_dosecheck_pdfs_watch(&args.input, &args.output, args.watch)?;
    Ok(())
}
