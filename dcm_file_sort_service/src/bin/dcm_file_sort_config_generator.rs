use clap::Parser;
use rad_tools_dcm_file_sort_service::Config;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::Level;

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

/// Prompts the user with a question and waits for a non-empty response.
///
/// The function prints the provided question, followed by a colon and a space, to
/// the standard output. It then waits for the user's input. If the input is valid
/// and non-empty, it returns the response as a `String`. If the input is empty,
/// an error message is displayed, and the user is prompted again until a valid
/// non-empty response is provided.
///
/// # Parameters
/// - `question`: A string slice that holds the question to be presented to the user.
///
/// # Returns
/// A `String` containing the user's response.
fn ask_question(question: &str) -> String {
    use std::io::Write;

    loop {
        print!("{}: ", question);
        std::io::stdout().flush().unwrap();

        let mut response = String::new();
        if std::io::stdin().read_line(&mut response).is_ok() {
            let response = response.trim();
            if !response.is_empty() {
                return response.to_string();
            }
        }

        println!("Response cannot be empty. Please try again.");
    }
}

/// Prompts the user with a question and a default value, and waits for a response.
///
/// The function displays the provided question along with a default value in square brackets.
/// The user can either input a value or press Enter to accept the default.
/// If the input is empty, the default value is returned.
///
/// # Parameters
/// - `question`: A string slice representing the question to present to the user.
/// - `default`: A string slice representing the default value to use if no input is provided.
///
/// # Returns
/// A `String` containing either the user's response or the provided default value if
/// the user does not input anything.
fn ask_question_with_default(question: &str, default: &str) -> String {
    let new_question = format!("{} [default={}]: ", question, default);
    print!("{}", new_question);
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();
    let response = response.trim();
    if response.is_empty() {
        default.to_string()
    } else {
        response.to_string()
    }
}
