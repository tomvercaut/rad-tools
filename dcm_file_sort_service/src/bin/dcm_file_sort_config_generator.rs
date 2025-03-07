use clap::Parser;
use rad_tools_dcm_file_sort_service::Config;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::Level;

use rad_tools_core::cli::{ask_question, ask_question_with_default};

/// A command line interface (CLI) application to generate a configuration file used by dcm_file_sort.
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "
A command line interface (CLI) application to generate a configuration file used by dcm_file_sort.
"
)]
struct Cli {
    /// Path where the config file is written.
    #[arg(short, long, default_value = "config.toml")]
    pub output: String,
    /// Interactive mode
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut config = Config::default();
    if cli.interactive {
        config.paths.input_dir = PathBuf::from(ask_question("Input directory"));
        config.paths.output_dir = PathBuf::from(ask_question("Output directory"));
        config.paths.unknown_dir = PathBuf::from(ask_question(
            "Directory for data that couldn't be processed",
        ));
        config.log.level = Level::from_str(ask_question_with_default("Log level", "info").as_str())
            .expect("Failed to parse log level");
        config.other.wait_time_millisec =
            ask_question_with_default("Wait time in milliseconds", "500")
                .parse::<u64>()
                .unwrap();
    }

    let s = toml::to_string_pretty(&config).unwrap();

    let mut file = std::fs::File::create(cli.output).expect("Failed to create output file");
    file.write_all(s.as_bytes())
        .expect("Failed to write to output file");
}
