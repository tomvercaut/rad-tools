use clap::Parser;
use dicom_object::OpenFileOptions;
use std::io;
use std::io::BufRead;

/// A command line interface (CLI) application to convert a DICOM file into JSON format.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Filename to a DICOM file, if not specified, the filename will read from standard input.
    #[arg(value_name = "FILE")]
    input: Option<String>,
}

fn read_filename_from_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let filename = cli.input.unwrap_or_else(|| {
        read_filename_from_stdin().expect("Failed to read the filename from standard input.")
    });
    let obj = OpenFileOptions::new().open_file(filename)?;
    let s = dicom_json::to_string_pretty(obj)?;
    println!("{}", s);
    Ok(())
}
