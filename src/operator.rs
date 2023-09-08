use std::{error::Error, ops::AddAssign};


#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Remainder,
}

impl Operator {
    pub fn new(operator: &char) -> Result<Self, OperatorError> {
        match operator {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            '^' => Ok(Operator::Power),
            '%' => Ok(Operator::Remainder),
            _ => Err(OperatorError::UnexpectedOperator),
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = OperatorError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Operator::new(&value)
    }
}


#[derive(Debug, PartialEq)]
pub enum OperatorError {
    UnexpectedOperator,
}

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorError::UnexpectedOperator => write!(f, "Unexpected operator"),
        }
    }
}

impl Error for OperatorError {}

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn operator() {
        for operator in ['+', '-', '*', '/', '^', '%'] {
            assert!(Operator::new(&operator).is_ok());
        }
        assert!(Operator::new(&' ').is_err_and(|e| e == OperatorError::UnexpectedOperator));
    }

    #[test]
    fn try_from() {
        for operator in ['+', '-', '*', '/', '^', '%'] {
            assert!(Operator::try_from(operator).is_ok());
        }
        assert!(Operator::try_from(' ').is_err_and(|e| e == OperatorError::UnexpectedOperator));
    }
}
