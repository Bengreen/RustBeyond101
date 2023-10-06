use jsonschema::ValidationError;

/// Error type for handling errors on Sample
#[derive(Debug)]
pub enum MyError {
    /// Error with a message
    Message(&'static str),
    /// Error when we call cancel
    Cancelled,
    /// Serde error including a message
    Serde(&'static str),
    /// IO error including a message
    Io(&'static str),
    /// Error with JSON validation
    JsonValidation(Vec<String>),
    /// Validation error
    /// Improve this to carry an object to describe the details of the validation failure
    ValidationError(),
}

impl From<serde_json::Error> for MyError {
    fn from(_value: serde_json::Error) -> Self {
        Self::Serde("UNKNOWN &value.to_string()")
    }
}

impl From<std::io::Error> for MyError {
    fn from(_value: std::io::Error) -> Self {
        Self::Io("UNKNOWN")
    }
}

impl From<ValidationError<'_>> for MyError {
    fn from(_value: ValidationError<'_>) -> Self {
        Self::ValidationError()
    }
}
