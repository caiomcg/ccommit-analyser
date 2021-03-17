pub mod line_error;
pub mod validator_error;
mod check_line_length;
mod check_commit_type;

pub use line_error::LineError;
pub use validator_error::ValidatorError;

pub trait Validator {
    fn process(&self, line: &str) -> Result<(), LineError>;
}
