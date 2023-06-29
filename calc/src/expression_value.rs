use num_traits::Float;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExpressionValue<V> {
    Operand(V),
    Operator(MathOperator),
}

#[derive(Debug, PartialEq)]
pub enum RecursiveExpression<V> {
    Value(ExpressionValue<V>),
    Expression(Box<Layer<V>>),
}

pub type Layer<V> = Vec<RecursiveExpression<V>>;

#[derive(Debug, PartialEq)]
pub enum EvaluateLayerError {
    MissingOperand,
    MissingOperator,
    EmptyExpressionLayer,
    InvalidExpressionValueCount,
}

trait EvaluateOperator {
    fn eval_operators(
        &self,
        selected_operators: &[MathOperator],
    ) -> Result<Self, EvaluateLayerError>
    where
        Self: Sized;
}

impl<V: Float + Debug> EvaluateOperator for Vec<ExpressionValue<V>> {
    fn eval_operators(
        &self,
        selected_operators: &[MathOperator],
    ) -> Result<Self, EvaluateLayerError>
    where
        Self: Sized,
    {
        let mut new_layer: Vec<ExpressionValue<V>> = Vec::new();
        let mut self_iter = self.iter();
        {
            let first_value = match self_iter.next() {
                Some(expression_value) => match expression_value {
                    ExpressionValue::Operand(operand) => Ok(ExpressionValue::Operand(*operand)),
                    ExpressionValue::Operator(_) => Err(EvaluateLayerError::MissingOperand),
                },
                None => Err(EvaluateLayerError::EmptyExpressionLayer),
            }?;
            new_layer.push(first_value);
        }

        for current_equation in self_iter.as_slice().chunks(2) {
            
            let left_operand = match new_layer.pop() {
                Some(ExpressionValue::Operator(_)) => Err(EvaluateLayerError::MissingOperand),
                Some(ExpressionValue::Operand(operand)) => Ok(operand),
                None => panic!(),
            }?;

            let operator = match current_equation.get(0) {
                Some(ExpressionValue::Operator(operator)) => Ok(*operator),
                Some(ExpressionValue::Operand(_)) => Err(EvaluateLayerError::MissingOperator),
                None => panic!(),
            }?;

            let right_operand = match current_equation.get(1) {
                Some(ExpressionValue::Operator(_)) => Err(EvaluateLayerError::MissingOperand),
                Some(ExpressionValue::Operand(operand)) => Ok(*operand),
                None => Err(EvaluateLayerError::MissingOperand),
            }?;

            if selected_operators.contains(&operator) {
                let solution = match operator {
                    MathOperator::Add => left_operand.add(right_operand),
                    MathOperator::Sub => left_operand.sub(right_operand),
                    MathOperator::Mul => left_operand.mul(right_operand),
                    MathOperator::Div => left_operand.div(right_operand),
                    MathOperator::Rem => left_operand.rem(right_operand),
                    MathOperator::Pow => left_operand.powf(right_operand),
                };
                dbg!(&solution);
                new_layer.push(ExpressionValue::Operand(solution));
                
            } else {
                dbg!(current_equation);
                new_layer.push(ExpressionValue::Operand(left_operand));
                new_layer.extend_from_slice(current_equation);
            }
        }

        if (self_iter.len() - 1) % 2 == 0 {
            match self_iter.next_back() {
                Some(ExpressionValue::Operand(_)) => Err(EvaluateLayerError::MissingOperator),
                Some(ExpressionValue::Operator(_)) => Err(EvaluateLayerError::MissingOperand),
                None => unreachable!(),
            }?;
        }

        Ok(new_layer)
    }
}

pub trait EvaluateLayer<V> {
    const ORDER_OF_OPERATIONS: [&'static [MathOperator]; 3] = [
        &[MathOperator::Pow],
        &[MathOperator::Mul, MathOperator::Div, MathOperator::Rem],
        &[MathOperator::Add, MathOperator::Sub],
    ];

    fn answer(&self) -> Result<V, EvaluateLayerError>;
}

impl<V: Float + Debug> EvaluateLayer<V> for Layer<V> {
    const ORDER_OF_OPERATIONS: [&'static [MathOperator]; 3] = [
        &[MathOperator::Pow],
        &[MathOperator::Mul, MathOperator::Div, MathOperator::Rem],
        &[MathOperator::Add, MathOperator::Sub],
    ];

    fn answer(&self) -> Result<V, EvaluateLayerError> {
        let expression_value_layer: Vec<ExpressionValue<V>> = self
            .iter()
            .map(|value_or_layer| match value_or_layer {
                RecursiveExpression::Value(value) => Ok(*value),
                RecursiveExpression::Expression(layer) => {
                    layer.answer().map(|value| ExpressionValue::Operand(value))
                }
            })
            .collect::<Result<Vec<ExpressionValue<V>>, EvaluateLayerError>>()?;

        let mut new_layer = expression_value_layer;
        for current_order_of_operations_operators in Self::ORDER_OF_OPERATIONS.iter() {
            dbg!(&current_order_of_operations_operators);
            new_layer = new_layer.eval_operators(&current_order_of_operations_operators)?;
            if new_layer.len() == 1 {
                return match new_layer
                    .get(0)
                    .expect("If all goes well there should be a single value")
                {
                    ExpressionValue::Operand(value) => Ok(*value),
                    ExpressionValue::Operator(_) => unreachable!(),
                };
            }
        }
        Err(EvaluateLayerError::InvalidExpressionValueCount)
    }
}

#[cfg(test)]
mod evaluate_tests {
    use super::*;

    #[test]
    fn eval_operators() {
        //.into_iter().map(|v| RecursiveExpression::Value(v)).collect();
        let test_expression: Vec<ExpressionValue<f32>> = vec![
            ExpressionValue::Operand(1.0),
            ExpressionValue::Operator(MathOperator::Add),
            ExpressionValue::Operand(1.0),
            ExpressionValue::Operator(MathOperator::Sub),
            ExpressionValue::Operand(1.0),
        ];
        let test_expression_answer: Vec<ExpressionValue<f32>> = vec![
            ExpressionValue::Operand(2.0),
            ExpressionValue::Operator(MathOperator::Sub),
            ExpressionValue::Operand(1.0),
        ];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap(),
            test_expression_answer
        );
    }
}
