use std::path::{Path, PathBuf};
use std::process::{Command, exit};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "
A command line application that synchronizes the data from a source directory to a destination directory. The data is synchronized in one way / direction, it doesn't create a mirror between two directories.
")]
struct Cli {
    /// Source file or directory
    #[arg(short, long, value_name = "DIR")]
    src: String,
    /// Destination directory
    #[arg(short, long, value_name = "DIR")]
    dest: String,
    /// Enable log
    #[arg(short, long, default_value_t = false)]
    log: bool,
}

fn main() {
    let cli = Cli::parse();
    one_way_sync(&cli);
}

/// This function performs a one-way synchronization using the `robocopy` command on Windows.
///
/// # Arguments
///
/// * `cli`: A reference to a `Cli` struct containing the necessary parameters for the sync operation.
#[cfg(windows)]
fn one_way_sync(cli: &Cli) {
    let mut cmd = Command::new("robocopy");
    cmd.args([&cli.src
                  , &cli.dest
                  , "*"
                  , "/BYTES"
                  , "/TEE"
                  , "/S"
                  , "/E"
                  , "/DCOPY:DA"
                  , "/COPY:DAT"
                  , "/IM"
                  , "/IT"
                  , "/MT"
                  , "/R:0"
                  , "/W:30"]);
    if cli.log {
        let logfile = Path::join(&PathBuf::from(&cli.dest), "robocopy.log");
        // let mut t = format!("{:#?}", &logfile);
        // if !t.is_empty() {
        //     t = t[1..t.len()-1].to_string();
        // }
        cmd.arg("/v");
        cmd.arg(format!("/UNILOG+:{}", &logfile.as_os_str().to_str().unwrap()));
    }
    let exit_status = cmd.status().expect("Something went wrong while running robocopy.");
    match exit_status.code() {
        None => {}
        Some(code) => {
            if code == 8 {
                exit(code);
            } else {
                exit(0);
            }
        }
    }
    exit(0);
}

#[cfg(not(windows))]
fn one_way_sync(cli: &Cli) {
    unimplemented!("Functionality is not implemented on platforms other than Windows");
}
