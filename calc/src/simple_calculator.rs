use core::fmt::Debug;
use num_traits::Float;

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
    fn checked_eval_simple_expression(
        left_operand: Self,
        operator: MathOperator,
        right_operand: Self,
    ) -> Result<Self, MathError>
    where
        Self: Float + Sized + CheckedFloatOperations {
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

impl EvaluateExpression for f32 {}
impl EvaluateExpression for f64 {}

#[cfg(test)]
mod enumvaluate_expression_tests {
    use super::*;

    #[test]
    fn checked_eval_simple_expression() {
        assert_eq!(f32::checked_eval_simple_expression(6.0, MathOperator::Add, 2.0).unwrap(), 8.0);
        assert_eq!(f32::checked_eval_simple_expression(6.0, MathOperator::Sub, 2.0).unwrap(), 4.0);
        assert_eq!(f32::checked_eval_simple_expression(6.0, MathOperator::Mul, 2.0).unwrap(), 12.0);
        assert_eq!(f32::checked_eval_simple_expression(6.0, MathOperator::Div, 2.0).unwrap(), 3.0);
        assert_eq!(f32::checked_eval_simple_expression(6.0, MathOperator::Rem, 2.0).unwrap(), 0.0);
        assert_eq!(f32::checked_eval_simple_expression(6.0, MathOperator::Pow, 2.0).unwrap(), 36.0);
    }
}
