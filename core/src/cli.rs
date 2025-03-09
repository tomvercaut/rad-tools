use std::io::{BufRead, Write};

/// Prompts the user with a question and waits for a non-empty response.
///
/// This function writes the provided question to the specified writer, followed by a colon and space,
/// then reads input from the provided reader. If the input is empty, it prompts the user again until
/// a non-empty response is provided.
///
/// # Parameters
/// - `reader`: Any type that implements `BufRead` for reading user input
/// - `writer`: Any type that implements `Write` for displaying the question
/// - `question`: The question to present to the user
///
/// # Returns
/// A `String` containing the user's non-empty response.
pub fn ask_question<R: BufRead, W: Write, S: AsRef<str>>(
    mut reader: R,
    mut writer: W,
    question: S,
) -> String {
    loop {
        writer
            .write_fmt(format_args!("{}: ", question.as_ref()))
            .expect("Failed to write to Writer");
        writer.flush().expect("Failed to flush Writer");

        let mut response = String::new();
        reader
            .read_line(&mut response)
            .expect("Failed to read from Reader");
        if reader.read_line(&mut response).is_ok() {
            let response = response.trim();
            if !response.is_empty() {
                return response.to_string();
            }
        }

        let _ = writer
            .write("Response cannot be empty. Please try again.".as_ref())
            .expect("Failed to write to Writer");
    }
}

/// Prompts the user with a question and returns an optional response.
///
/// This function writes the provided question to the specified writer, followed by a colon and space,
/// then reads input from the provided reader. Unlike `ask_question`, this function returns `None` if
/// the input is empty or if there was an error reading the input.
///
/// # Parameters
/// - `reader`: Any type that implements `BufRead` for reading user input
/// - `writer`: Any type that implements `Write` for displaying the question
/// - `question`: The question to present to the user
///
/// # Returns
/// - `Some(String)` containing the user's non-empty response
/// - `None` if the input was empty or there was an error reading the input
pub fn ask_question_opt<R: BufRead, W: Write, S: AsRef<str>>(
    mut reader: R,
    mut writer: W,
    question: S,
) -> Option<String> {
    writer
        .write_fmt(format_args!("{}: ", question.as_ref()))
        .expect("Failed to write to Writer");
    writer.flush().expect("Failed to flush Writer");

    let mut response = String::new();
    if reader.read_line(&mut response).is_ok() {
        let response = response.trim();
        if !response.is_empty() {
            return Some(response.to_string());
        }
    }
    None
}

/// Prompts the user with a question and returns the response or a default value.
///
/// This function writes the provided question to the specified writer, followed by the default value
/// in square brackets, then reads input from the provided reader. If the input is empty or contains
/// only whitespaces, the default value is returned.
///
/// # Parameters
/// - `reader`: Any type that implements `BufRead` for reading user input
/// - `writer`: Any type that implements `Write` for displaying the question
/// - `question`: The question to present to the user
/// - `default`: The default value to use if no input is provided
///
/// # Returns
/// A `String` containing either the user's non-empty response or the default value.
pub fn ask_question_with_default<R: BufRead, W: Write, S: AsRef<str>>(
    mut reader: R,
    mut writer: W,
    question: S,
    default: S,
) -> String {
    writer
        .write_fmt(format_args!(
            "{} [default={}]: ",
            question.as_ref(),
            default.as_ref()
        ))
        .expect("Failed to write to Writer");
    writer.flush().expect("Failed to flush Writer");

    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read from Reader");
    let response = response.trim();
    if response.is_empty() {
        default.as_ref().to_string()
    } else {
        response.to_string()
    }
}

/// Prompts the user with a question and numbered options, returns the selected option.
///
/// The function displays the question followed by a numbered list of options.
/// The user must select an option by entering its number.
/// If an invalid number is entered, the question and options are displayed again.
///
/// # Parameters
/// - `question`: A string slice representing the question to present to the user.
/// - `options`: A slice of strings representing the available options.
///
/// # Returns
/// A `String` containing the selected option's value.
pub fn ask_question_with_options<S: AsRef<str>>(question: S, options: &[String]) -> String {
    loop {
        match ask_question_with_options_opt(question.as_ref(), options) {
            Some(selection) => return selection,
            None => {
                println!("Invalid selection. Please try again.");
                continue;
            }
        }
    }
}

/// Prompts the user with a question and numbered options, returns an optional selected option.
///
/// The function displays the question followed by a numbered list of options.
/// The user must select an option by entering its number.
/// Unlike `ask_question_with_options`, this function returns `None` for invalid input
/// instead of repeatedly prompting the user.
///
/// # Parameters
/// - `question`: A string slice representing the question to present to the user.
/// - `options`: A slice of strings representing the available options.
///
/// # Returns
/// - `Some(String)` containing the selected option's value if a valid selection was made
/// - `None` if the input was invalid or there was an error reading the input
///
/// # Example
///
/// ```
/// use rad_tools_core::cli::ask_question_with_options_opt;
///
/// let options = vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()];
/// match ask_question_with_options_opt("Choose a color", &options) {
///     Some(color) => println!("You chose: {}", color),
///     None => println!("Invalid selection"),
/// }
/// ```
pub fn ask_question_with_options_opt<S: AsRef<str>>(
    question: S,
    options: &[String],
) -> Option<String> {
    println!("\n{}", question.as_ref());
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    print!("Select: ");
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    if std::io::stdin().read_line(&mut response).is_ok() {
        if let Ok(selection) = response.trim().parse::<usize>() {
            if selection > 0 && selection <= options.len() {
                return Some(options[selection - 1].clone());
            }
        }
    }
    None
}

pub mod in_out {
    use std::io::{stdin, stdout};

    pub fn ask_question<S: AsRef<str>>(question: S) -> String {
        super::ask_question(stdin().lock(), stdout(), question)
    }
    pub fn ask_question_opt<S: AsRef<str>>(question: S) -> Option<String> {
        super::ask_question_opt(stdin().lock(), stdout(), question)
    }

    pub fn ask_question_with_default<S: AsRef<str>>(question: S, default: S) -> String {
        super::ask_question_with_default(stdin().lock(), stdout(), question, default)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ask_question() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("this is a test\n"));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question(reader, writer, "Question");
        assert_eq!(response, "this is a test");
    }

    #[test]
    fn test_ask_question_opt() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("test response\n"));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question_opt(reader, writer, "Question");
        assert_eq!(response, Some("test response".to_string()));
    }

    #[test]
    fn test_ask_question_opt_empty() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("\n"));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question_opt(reader, writer, "Question");
        assert_eq!(response, None);
    }

    #[test]
    fn test_ask_question_opt_error() {
        let reader = std::io::BufReader::new(std::io::Cursor::new(Vec::new()));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question_opt(reader, writer, "Question");
        assert_eq!(response, None);
    }

    #[test]
    fn test_ask_question_with_default_custom_response() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("custom response\n"));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question_with_default(reader, writer, "Question", "default");
        assert_eq!(response, "custom response");
    }

    #[test]
    fn test_ask_question_with_default_empty() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("\n"));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question_with_default(reader, writer, "Question", "default");
        assert_eq!(response, "default");
    }

    #[test]
    fn test_ask_question_with_default_whitespace() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("   \n"));
        let writer = std::io::BufWriter::new(std::io::Cursor::new(Vec::new()));
        let response = super::ask_question_with_default(reader, writer, "Question", "default");
        assert_eq!(response, "default");
    }
}
