use regex::Regex;
use crate::validator::{Validator, LineError, ValidatorError};

pub struct CheckLineLength {
    size: usize,
    re: Regex,
}

impl CheckLineLength {
    pub fn new(size: usize) -> Self {
        let regex_string = format!(r"^.{{1,{}}}\w$", size);

        Self {
            size,
            re: Regex::new(&regex_string).unwrap(),
        }
    }
}

impl Validator for CheckLineLength {
    fn process(&self, line: &str) -> Result<(), LineError> {
        if !self.re.is_match(line) {
            let hint = match line.len() {
                0 => String::from("Cannot validate empty lines"),
                _ => String::from(format!("Line should not exceed {} characters", self.size)),
            };

            let error = match line.len() {
                0 => ValidatorError::EmptyLine,
                _ => ValidatorError::LineTooBig,
            };

            return Err(LineError {
                pos: 0,
                hint,
                error
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_below_fifty_characters() {
        let line = "feat: this line is below 50 characters";
        let validator = CheckLineLength::new(50);

        let result = validator.process(&line);

        assert!(result.is_ok());
    }

    #[test]
    fn line_above_fifty_characters() {
        let line = "feat: this line is abovel the estipulated 50 characters limit";
        let validator = CheckLineLength::new(50);

        let result = validator.process(&line);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.pos, 0);
            assert_eq!(err.hint, "Line should not exceed 50 characters");
            assert_eq!(err.error, ValidatorError::LineTooBig);
        }
    }

    #[test]
    fn line_above_twenty_characters() {
        let line = "feat: this line is abovel the estipulated 20 characters limit";
        let validator = CheckLineLength::new(20);

        let result = validator.process(&line);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.pos, 0);
            assert_eq!(err.hint, "Line should not exceed 20 characters");
            assert_eq!(err.error, ValidatorError::LineTooBig);
        }
    }

    #[test]
    fn should_not_validate_empty_line() {
        let line = "";
        let validator = CheckLineLength::new(20);

        let result = validator.process(&line);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.pos, 0);
            assert_eq!(err.hint, "Cannot validate empty lines");
            assert_eq!(err.error, ValidatorError::EmptyLine);
        }
    }
}
