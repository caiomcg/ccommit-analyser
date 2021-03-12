#[derive(Debug, PartialEq)]
pub enum ValidatorError {
    LineTooBig,
    EmptyLine,
    InvalidType,
    MissingColon,
    MissingSpace,
    InvalidCharacters,
}
