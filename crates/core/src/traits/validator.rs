use crate::{Messages, ValidatorError};

/// A trait for types that can validate configuration files or other resources
///
/// # Examples
///
/// ```rust
/// use doctor_core::traits::Validator;
/// use doctor_core::{Messages, ValidatorError};
///
/// struct MyValidator;
///
/// impl Validator for MyValidator {
///     fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
///         // Validation logic here
///         Ok(vec![])
///     }
///
/// }
/// ```
pub trait Validator {
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError>;
}
