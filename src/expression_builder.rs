use std::str::Chars;
use crate::{operator::Operator, expression_item::Parentheses};

pub struct ExpressionBuilder<'a> {
    chars: Chars<'a>,
}

impl<'a> ExpressionBuilder<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            chars: expression.chars(),
        }
    }

    pub fn get_operand(&mut self) -> Option<f32> {
        todo!();
    }

    pub fn get_operator(&mut self) -> Option<Operator> {
        todo!();
    }

    pub fn get_parentheses(&mut self) -> Option<Parentheses> {
        todo!();
    }
}



#[cfg(test)]
mod expresion_builder_tests {
    use crate::expression_item::ExpressionItem;
    use super::*;


    fn get_expression() -> ExpressionItem {
        todo!();
    }
}