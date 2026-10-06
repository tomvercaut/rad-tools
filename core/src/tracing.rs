use std::io::IsTerminal;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::{DefaultFields, Format};
use tracing_subscriber::fmt::SubscriberBuilder;


/// Creates a preconfigured `tracing_subscriber` format builder with default environment settings.
///
/// This function configures a `SubscriberBuilder` with:
/// - Environment filter derived from default environment variables (e.g. `RUST_LOG`)
/// - Target enabled
/// - Thread IDs enabled
/// - Source file names enabled
/// - Line numbers enabled
/// - ANSI terminal colors disabled if the standard output is not a terminal.
///
/// # Returns
///
/// A `SubscriberBuilder` initialized with the default formatting and filter settings.
pub fn default_env_subscriber() -> SubscriberBuilder<DefaultFields, Format, EnvFilter> {
    let is_term = std::io::stdout().is_terminal();
     tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(is_term)
}