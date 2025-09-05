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
