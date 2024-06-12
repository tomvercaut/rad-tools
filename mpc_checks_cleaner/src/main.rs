use std::path::Path;

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use clap::Parser;
use tracing::{info, Level, trace};

const TDS: &str = "TDS";
const MPC_CHECKS: &str = "MPCChecks";

/// A command line interface (CLI) application to clean the MPC checks in the VA_TRANSFER share.
///
/// The application removes old MPC checks from the VA_TRANSFER share. The application doesn't remove the Result.csv as it is small and can be usefull for external analysis. MPC checks are kept for a number of days before they are removed. Default value is 365 days.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "
A command line interface (CLI) application to clean the MPC checks in the VA_TRANSFER share.

The application removes old MPC checks from the VA_TRANSFER share. The application doesn't remove the Result.csv as it is small and can be usefull for external analysis. MPC checks are kept for a number of days before they are removed. Default value is 365 days.
")]
struct Cli {
    /// VA_TRANSFER share path
    #[arg(short, long, value_name = "DIR")]
    dir: String,
    /// Number of days the MPC checks are kept. Checks that are older, will be removed.
    #[arg(short, long, default_value_t = 365)]
    keep: i64,
    /// Enable logging at DEBUG level.
    #[arg(long, default_value_t = false)]
    dry_run: bool,
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
        .init();

    if cli.dry_run {
        println!("{}", text_box("DRY RUN (no data will be removed)"))
    }

    trace!("Commandline arguments: {:#?}", &cli);

    let va_transfer_path = Path::new(&cli.dir);
    if !va_transfer_path.exists() {
        panic!("VA_TRANSFER share path does not exist.");
    }
    if !va_transfer_path.is_dir() {
        panic!("VA_TRANSFER share path is not a directory.");
    }

    let tds_path = Path::join(va_transfer_path, TDS);
    if !tds_path.exists() {
        panic!("{:#?} share path does not exist.", tds_path);
    }
    if !tds_path.is_dir() {
        panic!("{:#?} share path is not a directory.", tds_path);
    }

    match std::fs::read_dir(&tds_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(machine_id_entry) => {
                        match machine_id_entry.metadata() {
                            Ok(meta) => {
                                if !meta.is_dir() {
                                    panic!("Expecting all directory entries in {:#?} to be a directory [with a machine ID as a name]", tds_path);
                                }
                                let mpc_checks_path = Path::join(&machine_id_entry.path(), MPC_CHECKS);
                                clean_mpc_checks_path(&mpc_checks_path, cli.keep, cli.dry_run);
                            }
                            Err(e) => {
                                panic!("Unable to get machine directories in {:#?}.\n{:#?}", tds_path, e);
                            }
                        }
                    }
                    Err(e) => {
                        panic!("Unable to get machine directories in {:#?}.\n{:#?}", tds_path, e);
                    }
                }
            }
        }
        Err(e) => {
            panic!("Unable to get machine directories in {:#?}.\n{:#?}", tds_path, e);
        }
    }
}

fn newline() -> &'static str {
    if std::env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    }
}

fn text_box(s: &str) -> String {
    let line = "+".repeat(s.len() + 4);
    let empty_line = "|".to_string() + &" ".repeat(s.len() + 2) + "|";
    let t = format!("| {} |", s);
    format!("{}{}{}{}{}{}{}{}{}",
            line, newline(), 
            &empty_line, newline(), 
            t, newline(), 
            &empty_line, newline(), 
            &line)
}

/// Cleans up the MPC checks path by removing checks that are older than a specified number of days.
///
/// # Arguments
///
/// * `path` - The path to the directory containing the MPC checks (e.g. <va_transfer share>/TDS/<machine_id>/MPCChecks).
/// * `keep` - The number of days to keep the checks.
/// * `dry_run` - Whether to perform a dry run or actually remove the checks.
///
/// # Panics
///
/// This function panics in the following situations:
///
/// * Unable to read the MPC checks directory or its metadata.
fn clean_mpc_checks_path(path: &Path, keep: i64, dry_run: bool) {
    let now = Local::now().naive_local();
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        match entry.metadata() {
                            Ok(meta) => {
                                if !meta.is_dir() {
                                    continue;
                                }
                                let os_fn = entry.file_name();
                                let file_name = os_fn.to_string_lossy();
                                let date_time = datetime_from_dir(&file_name);
                                let duration = now.signed_duration_since(date_time);
                                if duration.num_days() >= keep {
                                    clean_mpc_path(&entry.path(), dry_run);
                                    // std::fs::remove_dir_all(entry.path())?;
                                }
                            }
                            Err(e) => {
                                panic!("Unable to read MPC checks in {:#?}.\n{:#?}", path, e);
                            }
                        }
                    }
                    Err(e) => {
                        panic!("Unable to read MPC checks in {:#?}.\n{:#?}", path, e);
                    }
                }
            }
        }
        Err(e) => {
            panic!("Unable to read MPC checks in {:#?}.\n{:#?}", path, e);
        }
    }
}

/// Cleans an MPC check directory by removing all files except "Results.xml" and "Results.csv".
/// If `dry_run` is `true`, it only logs the files to be removed without actually removing them.
///
/// # Arguments
///
/// * `p` - The path to the MPC check directory.
/// * `dry_run` - Specifies whether it is a dry run or not.
///
/// # Panics
///
/// This function will panic if:
///
/// * Unable to read the directory at `p`.
/// * Unable to retrieve the metadata of a file.
/// * The directory contains a subdirectory.
/// * The directory contains a symbolic link.
/// * Unable to remove a file.
fn clean_mpc_path(p: &Path, dry_run: bool) {
    match std::fs::read_dir(p) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        match entry.metadata() {
                            Ok(meta) => {
                                if meta.is_dir() {
                                    panic!("Unable to clean MPC checks directory {:#?}.\nOnly files are expected in this directory, not directories.", p);
                                }
                                if meta.is_symlink() {
                                    panic!("Unable to clean MPC checks directory {:#?}.\nOnly files are expected in this directory, not symbolic links.", p);
                                }
                                let os_fn = entry.file_name();
                                let file_name = os_fn.to_string_lossy();
                                if file_name.as_ref() != "Results.xml" && file_name.as_ref() != "Results.csv" {
                                    if dry_run {
                                        info!("Removing: {:#?}", entry.path());
                                    } else {
                                        trace!("Removing: {:#?}", entry.path());
                                        let err_msg = format!("Unable to clean MPC checks file: {:#?}", entry.path());
                                        std::fs::remove_file(entry.path()).expect(&err_msg);
                                    }
                                }
                            }
                            Err(e) => {
                                panic!("Unable to clean MPC checks directory {:#?}.\n{:#?}", p, e);
                            }
                        }
                    }
                    Err(e) => {
                        panic!("Unable to clean MPC checks directory {:#?}.\n{:#?}", p, e);
                    }
                }
            }
        }
        Err(e) => {
            panic!("Unable to clean MPC checks directory {:#?}.\n{:#?}", p, e);
        }
    }
}

/// Converts a directory name into a `NaiveDateTime` object.
///
/// The directory name should follow the format: `NDS-WKS-SN5783-2024-01-11-07-42-57-0000-BeamCheckTemplate6xFFF`.
///
/// # Arguments
///
/// * `s` - A string slice representing the directory name.
///
/// # Returns
///
/// Returns a `NaiveDateTime` object representing the date and time extracted from the directory name.
///
/// # Panics
///
/// This function will panic if the directory name does not have the correct format.
fn datetime_from_dir(s: &str) -> NaiveDateTime {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 11 {
        panic!("Invalid directory name format detected in {:#?}", s);
    }
    let year = parts[3].parse::<i32>().unwrap();
    let month = parts[4].parse::<u32>().unwrap();
    let day = parts[5].parse::<u32>().unwrap();
    let hour = parts[6].parse::<u32>().unwrap();
    let minute = parts[7].parse::<u32>().unwrap();
    let second = parts[8].parse::<u32>().unwrap();
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        NaiveTime::from_hms_opt(hour, minute, second).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_datetime_from_dir_valid() {
        let input = "NDS-WKS-SN5783-2024-01-11-07-42-57-0000-BeamCheckTemplate6xFFF";
        let date_time = datetime_from_dir(input);
        assert_eq!(NaiveDateTime::new(
           NaiveDate::from_ymd_opt(2024,01,11).unwrap() ,
            NaiveTime::from_hms_opt(7,42,57).unwrap()
        ), date_time);
    }
    
    #[test]
    #[should_panic(expected = "Invalid directory name format detected in \"NDS-WKS\"")]
    fn test_datetime_from_dir_invalid_format() {
        let input = "NDS-WKS";
        let _ = datetime_from_dir(input);
    }
}
