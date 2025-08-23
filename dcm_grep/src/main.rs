use clap::Parser;
use rad_tools_dcm_grep::grep;
use std::io::{self, BufRead};

/// Extract the values of one or more (nested) DICOM tags.
///
/// A pattern-based syntax is used to extract the value of the DICOM elements.
#[derive(Parser, Debug, Clone)]
#[command(version)]
struct Args {
    /// Filename to a DICOM file, if not specified, the filename will read from standard input.
    #[clap(short, long, value_name = "FILE")]
    input: Option<String>,

    /// Use a pattern to select the DICOM elements to extract.
    ///
    /// The pattern can be one of the following:
    ///
    /// - (group,element)
    ///
    /// - (group,element)[<selector>]
    ///
    /// Where group and element are DICOM tag identifiers in hexadecimal.
    ///
    /// The optional [<selector>] can be specified on DICOM sequences:
    ///
    /// - [*] suffix matches all nested elements
    ///
    /// - [<index>] index (zero-based)
    ///
    /// - [<start>-<stop>] a range of indices (zero-based)
    ///
    /// Nested DICOM elements can be found be appending a `/` to the pattern.
    ///
    /// For example:
    ///
    /// - (0008,0016): selects the SOP Class UID
    ///
    /// - (3006,0010)[*]/(0020,0052): selects all the Frame Of Reference UIDs in the Referenced Frame Of Reference Sequence.
    ///
    /// - (3006,0010)[0]/(0020,0052): selects the Frame Of Reference UID in the first Referenced Frame Of Reference Sequence item.
    ///
    /// - (3006,0010)[1]/(0020,0052): selects the Frame Of Reference UID in the second Referenced Frame Of Reference Sequence item.
    #[clap(short = 'e', value_name = "PATTERN")]
    patterns: Vec<String>,

    /// Recursively search for the pattern in nested DICOM elements.
    #[arg(short, long, default_value_t = false)]
    recursive: bool,

    #[arg(long, default_value_t = false)]
    show_path: bool,

    #[arg(long, default_value_t = false)]
    verbose: bool,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Enable logging at TRACE level.
    #[arg(long, default_value_t = false)]
    trace: bool,
}

fn read_filename_from_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() {
    let args = Args::parse();
    let level = rad_tools_common::get_log_level!(args);
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    let filename = args.input.unwrap_or_else(|| {
        read_filename_from_stdin().expect("Failed to read the filename from standard input.")
    });
    let obj = rad_tools_common::dicom::open_file(filename).expect("Failed to open the file.");

    let mut results = vec![];

    for pattern in args.patterns {
        let v = grep(&obj, pattern.as_str(), args.recursive).expect("Failed to grep data.");
        results.extend(v);
    }

    if args.show_path {
        for result in results {
            println!("{}: {}", result.path, result.value);
        }
    } else {
        for result in results {
            println!("{}", result.value);
        }
    }
}
