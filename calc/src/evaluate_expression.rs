use core::fmt::Debug;
use num_traits::Float;

use crate::checked_operations::{CheckedFloatOperations, MathError};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rem,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionLayer<T> {
    Layer(Box<Vec<ExpressionLayer<T>>>),
    Operator(MathOperator),
    Number(T),
}

#[derive(PartialEq, Debug)]
pub enum OrderOfOperations {
    Parenthesis,
    Pow([MathOperator; 1]),
    MulDivAndRem([MathOperator; 3]),
    AddAndSub([MathOperator; 2]),
}

impl OrderOfOperations {
    const ORDERED: [Self; 4] = [
        OrderOfOperations::Parenthesis,
        OrderOfOperations::Pow([MathOperator::Pow]),
        OrderOfOperations::MulDivAndRem([
            MathOperator::Mul,
            MathOperator::Div,
            MathOperator::Rem,
        ]),
        OrderOfOperations::AddAndSub([MathOperator::Add, MathOperator::Sub]),
    ];

    const ORDERED_OPERATIONS: [Self; 3] = [
        OrderOfOperations::Pow([MathOperator::Pow]),
        OrderOfOperations::MulDivAndRem([
            MathOperator::Mul,
            MathOperator::Div,
            MathOperator::Rem,
        ]),
        OrderOfOperations::AddAndSub([MathOperator::Add, MathOperator::Sub]),
    ];
}

pub enum EvaluateExpressionError {
    MathError(MathError),
    EmptyParenthesis,
    MissingRightOperand,
    MissingLeftOperand,
    MissingOperator,
}

pub trait EvaluateExpression<T>
where
    T: Float + Sized + CheckedFloatOperations,
{

    fn eval_operation(self, operators: OrderOfOperations) -> Result<Vec<ExpressionLayer<T>>, EvaluateExpressionError>;

    fn eval_simple_expression(left_term: T, operator: MathOperator, right_term: T) -> Result<T, MathError> {
        match operator {
        MathOperator::Add => left_term.checked_add(right_term),
        MathOperator::Sub => left_term.checked_sub(right_term),
        MathOperator::Mul => left_term.checked_mul(right_term),
        MathOperator::Div => left_term.checked_div(right_term),
        MathOperator::Pow => left_term.checked_pow(right_term),
        MathOperator::Rem => left_term.checked_rem(right_term),
        }
    }

    fn eval(self) -> Result<T, EvaluateExpressionError>;
}

impl<T> EvaluateExpression<T> for Vec<ExpressionLayer<T>>
where
    T: Float + Sized + CheckedFloatOperations,
{
    fn eval_operation(self, operator: OrderOfOperations) -> Result<Vec<ExpressionLayer<T>>, EvaluateExpressionError> {
        debug_assert_ne!(operator, OrderOfOperations::Parenthesis);
        debug_assert!(self.iter().any(|expression| matches!(expression, ExpressionLayer::Layer(_))));

        let current_operations = match operator {
            OrderOfOperations::Parenthesis => unreachable!(),
            OrderOfOperations::Pow(pow) => pow.to_vec(),
            OrderOfOperations::MulDivAndRem(mul_div_rem) => mul_div_rem.to_vec(),
            OrderOfOperations::AddAndSub(add_sub) => add_sub.to_vec(),
        };

        let mut index = 0;
        while index < self.len()-3 {
            let window = self.get(index..index+3);
            let test = match expression_slice {
                [Some(ExpressionLayer::Number(left_operand)), Some(ExpressionLayer::Operator(operator)), Some(ExpressionLayer::Number(right_operand))] => Self::eval_simple_expression(*left_operand, *operator, *right_operand).or(|err| Err(EvaluateExpressionError::MathError(err))),
                [left_operand, Some(ExpressionLayer::Operator(_)), Some(ExpressionLayer::Number(_))] if !matches!(left_operand, ExpressionLayer::Number(_)) => Err(EvaluateExpressionError::MissingLeftOperand),
                [Some(ExpressionLayer::Number(_)), operator, Some(ExpressionLayer::Number(_))] if !matches!(operator, ExpressionLayer::Operator(_)) => Err(EvaluateExpressionError::MissingOperator),
                [Some(ExpressionLayer::Number(_)), Some(ExpressionLayer::Operator(_)), right_operand] if !matches!(right_operand, ExpressionLayer::Number(_)) => Err(EvaluateExpressionError::MissingRightOperand),
                _ => unreachable!(),
            };
            index -= 2;
        };
        todo!();
    }

    fn eval(self) -> Result<T, EvaluateExpressionError> {
        let mut evalutated_expressions: Vec<ExpressionLayer<T>> = Vec::new();

        for expression in self.into_iter() {
            let evaluted_expression = match expression {
                ExpressionLayer::Layer(layer) => ExpressionLayer::Number(layer.eval()?),
                ExpressionLayer::Operator(operator) => ExpressionLayer::Operator(operator),
                ExpressionLayer::Number(number) => ExpressionLayer::Number(number),
            };
            evalutated_expressions.push(evaluted_expression);
        };

        for operator in OrderOfOperations::ORDERED_OPERATIONS {
            evalutated_expressions = evalutated_expressions.eval_operation(operator)?;
        };

        let number = match evalutated_expressions.get(0) {
            Some(number) => match number {
                ExpressionLayer::Layer(_) => unreachable!(),
                ExpressionLayer::Operator(_) => unreachable!(),
                ExpressionLayer::Number(number) => *number,
            },
            None => unreachable!(),
        };

        Ok(number)
    }
}

#[cfg(test)]
mod evaluate_expression_tests {
    use super::*;
}
