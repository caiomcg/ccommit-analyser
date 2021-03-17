use regex::Regex;
use crate::validator::{Validator, LineError, ValidatorError};

pub struct CheckCommitType {
    re: Regex,
}

impl CheckCommitType {
    pub fn new(mut type_group: Vec<&str>) -> Self {
        if type_group.len() == 0 { // Fallback to deault types
            type_group = vec![
                "build",
                "chore",
                "ci",
                "docs",
                "feat",
                "fix",
                "perf",
                "refactor",
                "revert",
                "style",
                "test",
            ];
        }

        let regex_string = format!(r"^({})", type_group.join("|"));

        Self {
            re: Regex::new(&regex_string).unwrap(),
        }
    }
}

impl Validator for CheckCommitType {
    fn process(&self, line: &str) -> Result<(), LineError> {
        if !self.re.is_match(line) {
            return Err(LineError {
                pos: 0,
                hint: String::from("Unknown commit type"),
                error: ValidatorError::InvalidType,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_types() -> Vec<&'static str> {
        vec![
            "build",
            "chore",
            "ci",
            "docs",
            "feat",
            "fix",
            "perf",
            "refactor",
            "revert",
            "style",
            "test",
        ]
    }

    fn create_validator() -> Box<dyn Validator> {
        Box::new(CheckCommitType::new(get_test_types()))
    }

    fn check_result(error: LineError) {
        assert_eq!(error.pos, 0);
        assert_eq!(error.error, ValidatorError::InvalidType);
        assert_eq!(error.hint, "Unknown commit type");
    }
        
    #[test]
    fn should_find_type() {
        let validator = create_validator();

        for t in get_test_types() {
            let line = format!("{}: this is a random message for a type", t);
            assert!(validator.process(&line).is_ok());
        }
    }

    #[test]
    fn should_fail_if_invalid_type() {
        let validator = create_validator();
        let line = "random_type: this is a random message";
        let result = validator.process(line);

        check_result(result.unwrap_err());
    }

    #[test]
    fn should_not_validate_an_empty_line() {
        let validator = create_validator();
        let line = "";
        let result = validator.process(line);

        check_result(result.unwrap_err());
    }

    #[test]
    fn should_set_default_types_if_empty_input() {
        let validator = CheckCommitType::new(vec![]);

        for t in get_test_types() {
            let line = format!("{}: this is a random message for a type", t);
            assert!(validator.process(&line).is_ok());
        }
    }
}
