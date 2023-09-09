use std::fmt::Display;

use crate::operator::{Operator, OperatorError};

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionItem {
    Operand(f32),
    Operator(Operator),
    Parentheses(Parentheses),
}

pub type Parentheses = Box<Vec<ExpressionItem>>;

impl ExpressionItem {
    pub fn operand(self) -> Option<f32> {
        match self {
            ExpressionItem::Operand(operand) => Some(operand),
            ExpressionItem::Operator(_) | ExpressionItem::Parentheses(_) => None,
        }
    }

    pub fn operator(self) -> Option<Operator> {
        match self {
            ExpressionItem::Operator(operator) => Some(operator),
            ExpressionItem::Operand(_) | ExpressionItem::Parentheses(_) => None,
        }
    }

    pub fn parentheses(self) -> Option<Parentheses> {
        match self {
            ExpressionItem::Parentheses(parentheses) => Some(parentheses),
            ExpressionItem::Operator(_) | ExpressionItem::Operand(_) => None,
        }
    }
}

impl From<&f32> for ExpressionItem {
    fn from(operand: &f32) -> Self {
        ExpressionItem::Operand(*operand)
    }
}

impl From<Parentheses> for ExpressionItem {
    fn from(parentheses: Parentheses) -> Self {
        ExpressionItem::Parentheses(parentheses)
    }
}

impl From<Vec<ExpressionItem>> for ExpressionItem {
    fn from(parentheses: Vec<ExpressionItem>) -> Self {
        ExpressionItem::Parentheses(Box::new(parentheses))
    }
}

impl From<Operator> for ExpressionItem {
    fn from(operator: Operator) -> Self {
        ExpressionItem::Operator(operator)
    }
}

impl TryFrom<char> for ExpressionItem {
    type Error = OperatorError;
    fn try_from(character: char) -> Result<Self, Self::Error> {
        Ok(ExpressionItem::from(Operator::new(&character)?))
    }
}

impl Display for ExpressionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionItem::Operand(operand) => write!(f, "{}", operand),
            ExpressionItem::Operator(operator) => write!(f, "{}", operator),
            ExpressionItem::Parentheses(parentheses) => {
                write!(f, "(")?;
                for item in parentheses.iter() {
                    write!(f, "{}", item)?
                }
                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod expression_item_tests {

    use super::*;

    #[test]
    fn from_f32() {
        for number in 0..50 {
            assert_eq!(
                ExpressionItem::from(&(number as f32)),
                ExpressionItem::Operand(number as f32),
            );
        }
    }

    #[test]
    fn from_operator() {
        for operator in ['+', '-', '*', '/', '^', '%'] {
            assert_eq!(
                ExpressionItem::from(Operator::new(&operator).unwrap()),
                ExpressionItem::Operator(Operator::new(&operator).unwrap())
            );
        }
    }

    #[test]
    fn from_vec() {
        let mut vector = vec![];
        for expression_item in &[
            ExpressionItem::from(&0.0f32),
            ExpressionItem::from(Operator::Add),
            ExpressionItem::Parentheses(Box::new(vec![ExpressionItem::from(Operator::Remainder)])),
        ] {
            vector.push(expression_item.clone());
            assert_eq!(
                ExpressionItem::from(Box::new(vector.clone())),
                ExpressionItem::Parentheses(Box::new(vector.clone()))
            );
        }
    }

    #[test]
    fn display() {
        let operand = ExpressionItem::from(&-0.1f32);
        assert_eq!(format!("{}", operand), "-0.1");
        let operator = ExpressionItem::from(Operator::Add);
        assert_eq!(format!("{}", operator), "+");
        let parentheses = ExpressionItem::from(Parentheses::new(vec![
            operand.clone(),
            operator.clone(),
            operand.clone(),
        ]));
        assert_eq!(format!("{}", parentheses), "(-0.1+-0.1)");
    }
}
