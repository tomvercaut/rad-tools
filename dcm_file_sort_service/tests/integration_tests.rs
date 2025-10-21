use crate::common::{dcm_file_sort_app, has_no_files};
use std::process::{Command, Stdio};
use tracing::{debug, error};

mod common;

#[test]
fn integration_test() {
    common::init_logger(tracing::Level::TRACE);
    let test_dir = common::test_dir("integration_test");

    // Set up the test directory and data
    if test_dir.exists() {
        std::fs::remove_dir_all(&test_dir).expect("Failed to remove the test directory");
    }
    if !test_dir.exists() {
        std::fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    }

    let idir = test_dir.join("input");
    let odir = test_dir.join("output");
    let udir = test_dir.join("unknown");

    std::fs::create_dir_all(&idir).expect("Failed to create the \"input\" directory");
    std::fs::create_dir_all(&odir).expect("Failed to create the \"output\" directory");
    std::fs::create_dir_all(&udir).expect("Failed to create the \"unknown\" directory");

    let expected = common::create_test_data(&idir, &odir, true, true, true, true);

    // Configure the application
    let sconfig = format!(
        r#"
[paths]
input_dir = {:#?}
output_dir = {:#?}
unknown_dir = {:#?}

[log]
level = "DEBUG"

[other]
wait_time_millisec = 1
io_timeout_millisec = 50
copy_attempts = 10
remove_attempts = 10
mtime_delay_secs = 10
limit_unique_filenames = 10
"#,
        &idir, &odir, &udir
    );
    let config_path = test_dir.join("config.toml");
    std::fs::write(&config_path, sconfig).expect("Failed to write the config file");

    // Spawn the application
    let app = dcm_file_sort_app();
    debug!("Starting the application in a separate thread: {:#?}", &app);
    // The process is killed after the data has been processed.
    #[allow(clippy::zombie_processes)]
    let mut process = Command::new(app)
        .arg("--config")
        .arg(&config_path)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    debug!("Waiting for the application to finish");
    let max = 200;
    let mut i = 0;
    while !has_no_files(&idir) {
        std::thread::sleep(std::time::Duration::from_millis(500));
        i += 1;
        if i > max {
            error!("Max retries exceeded for waiting for files to be processed");
            break;
        }
    }

    // Send a CTRL-C to the application, after which it should stop gracefully.
    debug!("Stopping the process ...");
    process.kill().unwrap();
    debug!("Stopped the process");

    for td in &expected {
        debug!("Checking test data: {:#?}", &td);
        assert!(
            td.result_path.exists(),
            "{}",
            format!("File not found: {:#?}", &td.result_path)
        );
    }
}
