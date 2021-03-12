// TODO: Make this serializable

use super::validator_error::ValidatorError;

#[derive(Debug)]
pub struct LineError {
    pub pos: usize,
    pub hint: String,
    pub error: ValidatorError,
}
