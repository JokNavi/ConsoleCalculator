use std::fmt::{write, Display};

use crate::{expression_builder::ExpressionBuilderError, expression_item::ExpressionItem};

pub trait Eval {
    fn eval(&self) -> Result<Option<f32>, EvalError>;
}

/*
fn testing() -> Result<Option<f32>, EvalError> {
    let mut expression = vec![0.0f32, 0.0f32, 0.0f32];
    let (mut first, mut rest) = match expression.split_first().unzip() {
        (None, None) => return Ok(None),
        (None, Some(_)) => unreachable!(),
        (Some(first), None) => return Ok(Some(*first)),
        (Some(first), Some(rest)) => (first, rest),
    };
    expression = rest.chunks(2).fold(
        Ok(vec![first.clone()]),
        |acc: Result<Vec<f32>, EvalError>, chunk| {
            let acc = acc?;
            let left_operand = acc.last().unwrap();
            let operator = chunk.get(0).unwrap();
            let right_operand = chunk.get(1).ok_or(EvalError::ExpectedOperand)?;
            Ok(acc)
        },
    )?;
    todo!();
}
*/

impl Eval for ExpressionItem {
    fn eval(&self) -> Result<Option<f32>, EvalError> {
        match self {
            ExpressionItem::Operand(operand) => Ok(Some(*operand)),
            ExpressionItem::Operator(_) => Err(EvalError::ExpectedOperand),
            ExpressionItem::Parentheses(parentheses) => {
                const OPERATIONS_ORDER: [&[char]; 3] = [&['^'], &['*', '/', '%'], &['+', '-']];
                let mut expression = *parentheses.clone();
                for operations in OPERATIONS_ORDER {
                    let (first, rest) = match expression.split_first().unzip() {
                        (None, None) => return Ok(None),
                        (None, Some(_)) => unreachable!(),
                        (Some(first), None) => return first.eval(),
                        (Some(first), Some(rest)) => (first, rest),
                    };
                    expression = rest.chunks(2).fold(
                        Ok(vec![first.clone()]),
                        |acc: Result<Vec<ExpressionItem>, EvalError>, chunk: &[ExpressionItem]| {
                            let mut acc = acc?;
                            let left_operand = acc
                                .pop()
                                .unwrap()
                                .operand()
                                .ok_or(EvalError::ExpectedOperand)?;
                            let operator = chunk
                                .get(0)
                                .unwrap()
                                .operator()
                                .ok_or(EvalError::ExpectedOperator)?;
                            let right_operand = chunk
                                .get(1)
                                .ok_or(EvalError::ExpectedOperand)?
                                .operand()
                                .ok_or(EvalError::ExpectedOperand)?;
                            if operations.contains(&char::from(operator)) {}
                            Ok(acc)
                        },
                    )?;
                }
                todo!()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ExpressionBuilderError(ExpressionBuilderError),
    ExpectedOperand,
    ExpectedOperator,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::ExpressionBuilderError(err) => write!(f, "{}", err),
            EvalError::ExpectedOperand => write!(f, "Expected operand"),
            EvalError::ExpectedOperator => write!(f, "Expected operator"),
        }
    }
}

impl From<ExpressionBuilderError> for EvalError {
    fn from(err: ExpressionBuilderError) -> Self {
        match err {
            ExpressionBuilderError::ExpectedOperand => EvalError::ExpectedOperand,
            err => EvalError::ExpressionBuilderError(err),
        }
    }
}

impl Eval for String {
    fn eval(&self) -> Result<Option<f32>, EvalError> {
        todo!()
    }
}
