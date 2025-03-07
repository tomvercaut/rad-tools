use std::io::Write;

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
pub fn ask_question<S: AsRef<str>>(question: S) -> String {
    use std::io::Write;

    loop {
        print!("{}: ", question.as_ref());
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

/// Prompts the user with a question and returns an optional response.
///
/// Similar to `ask_question`, but returns `None` if the input is empty or invalid
/// instead of repeatedly prompting the user. This is useful when you want to handle
/// empty responses differently in your application logic.
///
/// # Parameters
/// - `question`: A string slice that holds the question to be presented to the user.
///
/// # Returns
/// - `Some(String)` containing the user's response if input was non-empty
/// - `None` if the input was empty or there was an error reading the input
///
/// # Example
///
/// ```
/// use rad_tools_core::cli::ask_question_opt;
///
/// let name = ask_question_opt("What is your name");
/// match name {
///     Some(name) => println!("Hello, {}!", name),
///     None => println!("No name provided"),
/// }
/// ```
pub fn ask_question_opt<S: AsRef<str>>(question: S) -> Option<String> {
    use std::io::Write;

    print!("{}: ", question.as_ref());
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    if std::io::stdin().read_line(&mut response).is_ok() {
        let response = response.trim();
        if !response.is_empty() {
            return Some(response.to_string());
        }
    }
    None
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
pub fn ask_question_with_default<S: AsRef<str>>(question: S, default: S) -> String {
    let new_question = format!("{} [default={}]: ", question.as_ref(), default.as_ref());
    print!("{}", new_question);
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();
    let response = response.trim();
    if response.is_empty() {
        default.as_ref().to_string()
    } else {
        response.to_string()
    }
}
