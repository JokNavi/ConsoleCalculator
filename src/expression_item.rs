use crate::operator::Operator;

#[derive(Debug, PartialEq)]
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

impl From<Operator> for ExpressionItem {
    fn from(operator: Operator) -> Self {
        ExpressionItem::Operator(operator)
    }
}

#[cfg(test)]
mod expression_item_tests {

    use super::*;

    #[test]
    fn from_operator() {
        for operator_char in ['+', '-', '*', '/', '^', '%'] {
            let operator = Operator::new(&operator_char).unwrap();
            assert_eq!(
                ExpressionItem::from(operator),
                ExpressionItem::Operator(Operator::new(&operator_char).unwrap())
            );
        }
    }
}
