pub mod dicom;
mod macros;
pub mod system;

/// Provides validation functionality for types that need to verify their internal state
/// or conformance to specific rules.
///
/// The `ResultType` type parameter specifies the return type of the validation operation,
/// typically a bool, `Result` or similar type that can indicate success or specific validation errors.
pub trait Validate<ResultType> {
    /// Performs a validation of the type's internal state.
    ///
    /// # Return
    /// Returns a `ResultType` indicating whether validation succeeded or failed.
    /// The exact meaning of success/failure is defined by the implementing type.
    ///
    /// # Example
    /// ```
    /// use rad_tools_common::Validate;
    ///
    /// struct Person {
    ///     age: i32,
    ///     name: String,
    /// }
    ///
    /// impl Validate<bool> for Person {
    ///     fn validate(&self) -> bool {
    ///         self.age >= 0 && !self.name.is_empty()
    ///     }
    /// }
    ///
    /// let person = Person {
    ///     age: 25,
    ///     name: String::from("John"),
    /// };
    /// assert!(person.validate());
    /// ```
    fn validate(&self) -> ResultType;
}

/// Provides functionality for types that need to be started or initialized.
///
/// The `ResultType` type parameter specifies the return type of the start operation,
/// typically a `Result` or similar type that can indicate success or specific errors.
///
/// # Example
/// ```
/// use rad_tools_common::Start;
///
/// struct Service {
///     is_running: bool
/// }
///
/// impl Start<Result<(), String>> for Service {
///     fn start(&mut self) -> Result<(), String> {
///         if self.is_running {
///             return Err("Service already running".to_string());
///         }
///         self.is_running = true;
///         Ok(())
///     }
/// }
///
/// let mut service = Service { is_running: false };
/// assert!(service.start().is_ok());
/// ```
pub trait Start<ResultType> {
    fn start(&mut self) -> ResultType;
}

/// Provides functionality for types that need to be stopped or cleaned up.
///
/// The `ResultType` type parameter specifies the return type of the stop operation,
/// typically a `Result` or similar type that can indicate success or specific errors.
///
/// # Example
/// ```
/// use rad_tools_common::Stop;
///
/// struct Service {
///     is_running: bool
/// }
///
/// impl Stop<Result<(), String>> for Service {
///     fn stop(&mut self) -> Result<(), String> {
///         if !self.is_running {
///             return Err("Service not running".to_string());
///         }
///         self.is_running = false;
///         Ok(())
///     }
/// }
///
/// let mut service = Service { is_running: true };
/// assert!(service.stop().is_ok());
/// ```
pub trait Stop<ResultType> {
    fn stop(&mut self) -> ResultType;
}
