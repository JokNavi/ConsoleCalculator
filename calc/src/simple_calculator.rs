use num_traits::{Float};
use core::{fmt::Debug};

use crate::checked_operations::{CheckedFloatOperations, MathError};

#[derive(PartialEq, Debug)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rem,
}

#[derive(PartialEq, Debug)]
pub enum OperationItem<T> {
    Operator(MathOperator),
    Number(T),
}

#[derive(PartialEq, Debug)]
pub enum ExpressionLayer<T> {
    OperationItem(OperationItem<T>),
    Layer(Box<ExpressionLayer<T>>),
}

trait EvaluateExpression {

    /// Evaluates a simple arithmetic expression consisting of two operands and a given operator.
    fn checked_evaluate_simple_expression<Y: CheckedFloatOperations>(
        left_operand: Y,
        operator: MathOperator,
        right_operand: Y,
    ) -> Result<Y, MathError>
    where
        Y: Float,
        Self: Sized;
}

impl<T> EvaluateExpression for ExpressionLayer<T> {   

    fn checked_evaluate_simple_expression<Y: CheckedFloatOperations>(
        left_operand: Y,
        operator: MathOperator,
        right_operand: Y,
    ) -> Result<Y, MathError>
    where
        Y: Float,
        Self: Sized,
    {
        match operator {
            MathOperator::Add => left_operand.checked_add(right_operand),
            MathOperator::Sub => left_operand.checked_sub(right_operand),
            MathOperator::Mul => left_operand.checked_mul(right_operand),
            MathOperator::Div => left_operand.checked_div(right_operand),
            MathOperator::Pow => left_operand.checked_pow(right_operand),
            MathOperator::Rem => left_operand.checked_rem(right_operand),
        }
    }
}


#[cfg(test)]
mod solve_expression_test {
    use super::*;

    #[test]
    fn solve_simple_expression() {
        
        assert_eq!(
            ExpressionLayer::<f32>::checked_evaluate_simple_expression(5.0f32, MathOperator::Div, 0f32).unwrap_err(),
            MathError::DivisionBy0
        );
    }
}
