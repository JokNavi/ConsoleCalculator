use crate::operator::Operator;

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

impl From<f32> for ExpressionItem {
    fn from(operand: f32) -> Self {
        ExpressionItem::Operand(operand)
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

impl From<String> for ExpressionItem {
    fn from(string: String) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod expression_item_tests {

    use super::*;


    #[test]
    fn from_f32() {
        for number in 0..50 {
            assert_eq!(
                ExpressionItem::from(number as f32),
                ExpressionItem::Operand(number as f32),
            );
        }
    }

    #[test]
    fn from_operand() {
        for number in 0..50 {
            assert_eq!(
                ExpressionItem::from(number as f32),
                ExpressionItem::Operand(number as f32),
            );
        }
    }

    #[test]
    fn from_vec() {
        let mut vector = vec![];
        for expression_item in &[ExpressionItem::from(0.0f32), ExpressionItem::from(Operator::try_from('+').unwrap())] {
            vector.push(expression_item.clone());
            assert_eq!(
                ExpressionItem::from(Box::new(vector.clone())),
                ExpressionItem::Parentheses(Box::new(vector.clone()))
            );
        }
    }
}
