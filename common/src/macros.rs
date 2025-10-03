/// Converts verbosity flags to a tracing log level.
///
/// This macro takes a struct instance containing boolean flags for different
/// verbosity levels (`trace`, `debug`, and `verbose`) and returns the appropriate
/// [`tracing::Level`].
///
/// # Arguments
///
/// * `$instance` - A struct instance containing the following boolean fields:
///   * `trace` - Enable trace-level logging
///   * `debug` - Enable debug-level logging
///   * `verbose` - Enable info-level logging
///
/// # Returns
///
/// Returns a [`tracing::Level`] based on the following priority:
/// * `TRACE` if `trace` is true
/// * `DEBUG` if `debug` is true
/// * `INFO` if `verbose` is true
/// * `WARN` otherwise (default)
///
/// # Example
///
/// ```
/// use rad_tools_common::get_log_level;
///
/// struct Cli {
///     verbose: bool,
///     trace: bool,
///     debug: bool,
/// }
///
/// let cli = Cli {
///     verbose: true,
///     trace: false,
///     debug: false,
/// };
/// let level = get_log_level!(cli);
/// assert_eq!(level, tracing::Level::INFO);
/// ```
#[macro_export]
macro_rules! get_log_level {
    ($instance:ident) => {{
        if $instance.trace {
            ::tracing::Level::TRACE
        } else if $instance.debug {
            ::tracing::Level::DEBUG
        } else if $instance.verbose {
            ::tracing::Level::INFO
        } else {
            ::tracing::Level::WARN
        }
    }};
}

#[cfg(test)]
mod test {
    use tracing::Level;

    #[derive(Debug, Default)]
    struct Cli {
        verbose: bool,
        trace: bool,
        debug: bool,
    }

    #[test]
    fn test_verbose() {
        let cli = Cli {
            verbose: true,
            trace: false,
            debug: false,
        };
        let level = get_log_level!(cli);
        assert_eq!(level, Level::INFO);
    }

    #[test]
    fn test_trace() {
        let cli = Cli {
            verbose: false,
            trace: true,
            debug: false,
        };
        let level = get_log_level!(cli);
        assert_eq!(level, Level::TRACE);
    }

    #[test]
    fn test_debug() {
        let cli = Cli {
            verbose: false,
            trace: false,
            debug: true,
        };
        let level = get_log_level!(cli);
        assert_eq!(level, Level::DEBUG);
    }

    #[test]
    fn test_default() {
        let cli = Cli::default();
        let level = get_log_level!(cli);
        assert_eq!(level, Level::WARN);
    }
}
