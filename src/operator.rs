use std::error::Error;


#[derive(Debug, PartialEq)]
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
}
