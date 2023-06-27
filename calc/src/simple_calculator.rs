use num_traits::{Float, Zero};
use std::ops::{Add, Div, Mul, Rem, Sub};

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

#[derive(PartialEq, Debug)]
pub enum MathError {
    DivisionBy0,
    IntegerOverflow,
    IntegerUnderflow,
    NegativeSquareRoot,
}

trait SolveExpression<T> {

    /// Evaluates a simple arithmetic expression consisting of two operands and a given operator.
    fn evaluate(
        left_operand: T,
        operator: MathOperator,
        right_operand: T,
    ) -> Result<T, MathError>
    where
        T: Float,
        Self: Sized;
}

impl<T: Float> SolveExpression<T> for ExpressionLayer<T> {   

    fn evaluate(
        left_operand: T,
        operator: MathOperator,
        right_operand: T,
    ) -> Result<T, MathError>
    where
        T: Float,
        Self: Sized,
    {
        let outcome: T = match operator {
            MathOperator::Add => left_operand + right_operand,
            MathOperator::Sub => left_operand - right_operand,
            MathOperator::Mul => left_operand * right_operand,
            MathOperator::Div => left_operand / right_operand,
            MathOperator::Pow => left_operand.powf(right_operand),
            MathOperator::Rem => left_operand % right_operand,
        };
        
        let t_nan: T = T::nan();
        let t_infinity: T = T::infinity();
        let neg_infity: T = T::neg_infinity();

        match operator {
            MathOperator::Add =>  match outcome {
                _ if outcome == t_infinity => Err(MathError::IntegerOverflow),
                _ if outcome == neg_infity => Err(MathError::IntegerUnderflow),
                _ if outcome == t_nan => unimplemented!(),
                _ => Ok(outcome)
            },
            MathOperator::Sub =>  match outcome {
                _ if outcome == t_infinity => Err(MathError::IntegerOverflow),
                _ if outcome == neg_infity => Err(MathError::IntegerUnderflow),
                _ if outcome == t_nan => unimplemented!(),
                _ => Ok(outcome)
            },
            MathOperator::Mul => match outcome {
                _ if outcome == t_infinity => Err(MathError::IntegerOverflow),
                _ if outcome == neg_infity => Err(MathError::IntegerUnderflow),
                _ if outcome == t_nan => unimplemented!(),
                _ => Ok(outcome)
            },
            MathOperator::Div => match outcome {
                _ if outcome == t_infinity => Err(MathError::IntegerOverflow),
                _ if outcome == neg_infity => Err(MathError::IntegerUnderflow),
                _ if outcome == t_nan => Err(MathError::DivisionBy0),
                _ => Ok(outcome)
            },
            MathOperator::Pow => match outcome {
                _ if outcome == t_infinity => Err(MathError::IntegerOverflow),
                _ if outcome == neg_infity => Err(MathError::IntegerUnderflow),
                _ if outcome == t_nan => unimplemented!(),
                _ => Ok(outcome)
            },
            MathOperator::Rem => match outcome {
                _ if outcome == t_infinity => Err(MathError::IntegerOverflow),
                _ if outcome == neg_infity => Err(MathError::IntegerUnderflow),
                _ if outcome == t_nan => unimplemented!(),
                _ => Ok(outcome)
            },
        }
    }
}


#[cfg(test)]
mod solve_expression_test {
    use super::*;

    #[test]
    fn solve_simple_expression() {
        assert_eq!(
            ExpressionLayer::evaluate(f32::, MathOperator::Add, 5.0).unwrap(), 10.0
        );
        assert_eq!(
            ExpressionLayer::evaluate(5.0, MathOperator::Add, 5.0).unwrap(), 10.0
        );
    }
}
