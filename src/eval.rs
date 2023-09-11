use std::{error::Error, fmt::Display};

use crate::{
    expression_builder::{ExpressionBuilder, ExpressionBuilderError},
    expression_item::ExpressionItem,
    operator::Operator,
};

pub trait Evaluate {
    fn eval(&self) -> Result<Option<f32>, EvalError>;
}

impl Evaluate for ExpressionItem {
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
                                .unwrap().eval()?
                                .ok_or(EvalError::ExpectedOperand)?;
                            let operator = chunk
                                .get(0)
                                .unwrap()
                                .operator()
                                .ok_or(EvalError::ExpectedOperator)?;
                            let right_operand = chunk
                                .get(1)
                                .ok_or(EvalError::ExpectedOperand)?
                                .eval()?
                                .ok_or(EvalError::ExpectedOperand)?;
                            if operations.contains(&char::from(&operator)) {
                                acc.push(ExpressionItem::from(&match operator {
                                    Operator::Power => left_operand.powf(right_operand),
                                    Operator::Multiply => left_operand * right_operand,
                                    Operator::Divide => left_operand / right_operand,
                                    Operator::Remainder => left_operand % right_operand,
                                    Operator::Add => left_operand + right_operand,
                                    Operator::Subtract => left_operand - right_operand,
                                }));
                            } else {
                                acc.append(&mut vec![
                                    ExpressionItem::from(&left_operand),
                                    ExpressionItem::from(operator),
                                    ExpressionItem::from(&right_operand),
                                ]);
                            }
                            Ok(acc)
                        },
                    )?;
                }
                
                expression.first().unwrap().eval()
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

impl Error for EvalError {}

impl From<ExpressionBuilderError> for EvalError {
    fn from(err: ExpressionBuilderError) -> Self {
        match err {
            ExpressionBuilderError::ExpectedOperator => EvalError::ExpectedOperator,
            ExpressionBuilderError::ExpectedOperand => EvalError::ExpectedOperand,
            err => EvalError::ExpressionBuilderError(err),
        }
    }
}

impl Evaluate for &str {
    fn eval(&self) -> Result<Option<f32>, EvalError> {
        ExpressionItem::from(ExpressionBuilder::new(self).get_expression()?).eval()
    }
}


#[cfg(test)]
mod eval_test {
    use super::*;
    use crate::expression_builder::ExpressionBuilder;



    #[test]
    fn eval_str() {
        let expression = "1+1";
        let expression_items = ExpressionItem::from(ExpressionBuilder::new("1+1").get_expression().unwrap());
        let expression_eval = expression_items.eval();
        assert!(expression_eval.is_ok());
        assert!(expression_eval.as_ref().unwrap().is_some());
        assert_eq!(expression_eval.as_ref().unwrap().unwrap(), 2.0);
        assert_eq!(expression.eval(), expression_eval);
    }

    #[test]
    fn eval_ok() {
        assert!(r"+1".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 1.0)));
        assert!(r"1+1".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 2.0)));
        assert!(r"(1)+0".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 1.0)));
        assert!(r"1.5+(1)".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 2.5)));
        assert!(r"1".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 1.0)));
        assert!(r"-100.0".eval().is_ok_and(|ok| ok.is_some_and(|some| some == -100.0)));
        assert!(r"1+1+1+1".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 4.0)));
        assert!(r"5+(10*2)".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 25.0)));
        assert!(r"1+-1+1+1".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 2.0)));
        assert!(r"2+2-2*2/2%2^2".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 2.0)));
        assert!(r"1+(1)".eval().is_ok_and(|ok| ok.is_some_and(|some| some == 2.0)));
    }

    #[test]
    fn eval_err() {
        assert!(r"+".eval().is_err_and(|err| err == EvalError::ExpectedOperand));
        assert!(r"".eval().is_err_and(|err| err == EvalError::ExpectedOperand));
        assert!(r"(1".eval().is_err_and(|err| err == ExpressionBuilderError::ExpectedClosingParentheses.into()));
        assert!(r"1)".eval().is_err_and(|err| err == EvalError::ExpectedOperator));
        assert!(r"1+".eval().is_err_and(|err| err == EvalError::ExpectedOperand));
    }
}