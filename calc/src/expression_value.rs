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
    InvalidExpressionValueAmount,
}

trait EvaluateOperator {
    fn eval_operators(&self, selected_operators: &[MathOperator]) -> Result<Self, EvaluateLayerError> where Self: Sized;
}

impl<V: Float + Debug> EvaluateOperator for Vec<ExpressionValue<V>> {

    fn eval_operators(&self, selected_operators: &[MathOperator]) -> Result<Self, EvaluateLayerError> where Self: Sized {

            let mut new_layer: Vec<ExpressionValue<V>> = Vec::new();
            let mut index = 0;
            let layer_iter = self.iter().as_slice();

            if layer_iter.len() < 2 {
                return Err(EvaluateLayerError::InvalidExpressionValueAmount);
            }
            while index < layer_iter.len() - 2 {
                let left_operand = match layer_iter.get(index + 0) {
                    Some(ExpressionValue::Operator(_)) => Err(EvaluateLayerError::MissingOperand),
                    Some(ExpressionValue::Operand(operand)) => Ok(operand),
                    None => Err(EvaluateLayerError::EmptyExpressionLayer),
                }?;

                let operator = match layer_iter.get(index + 1) {
                    Some(ExpressionValue::Operator(operator)) => Ok(operator),
                    Some(ExpressionValue::Operand(_)) => Err(EvaluateLayerError::MissingOperator),
                    None => Err(EvaluateLayerError::EmptyExpressionLayer),
                }?;

                let right_operand = match layer_iter.get(index + 2) {
                    Some(ExpressionValue::Operator(_)) => Err(EvaluateLayerError::MissingOperand),
                    Some(ExpressionValue::Operand(operand)) => Ok(operand),
                    None => Err(EvaluateLayerError::EmptyExpressionLayer),
                }?;

                dbg!(&selected_operators);
                if selected_operators.contains(&operator) {
                    let solution = match operator {
                        MathOperator::Add => left_operand.add(*right_operand),
                        MathOperator::Sub => left_operand.sub(*right_operand),
                        MathOperator::Mul => left_operand.mul(*right_operand),
                        MathOperator::Div => left_operand.div(*right_operand),
                        MathOperator::Rem => left_operand.rem(*right_operand),
                        MathOperator::Pow => left_operand.powf(*right_operand),
                    };
                    dbg!(&solution);
                    new_layer.push(ExpressionValue::Operand(solution));
                    index += 3;
                } else {
                    new_layer.push(ExpressionValue::Operand(*left_operand));
                    index += 1;
                }
            }
            
            new_layer.extend_from_slice(layer_iter.get(index..).unwrap());
            dbg!(&new_layer);
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
                RecursiveExpression::Expression(layer) => 
                    layer.answer().map(|value| ExpressionValue::Operand(value))
            })
            .collect::<Result<Vec<ExpressionValue<V>>, EvaluateLayerError>>()?;

        dbg!(&expression_value_layer);

        let mut new_layer = expression_value_layer;
        for current_order_of_operations_operators in Self::ORDER_OF_OPERATIONS.iter() {
            dbg!(&current_order_of_operations_operators);
            new_layer = new_layer.eval_operators(&current_order_of_operations_operators)?;
            if new_layer.len() == 1 {
                return match new_layer.get(0).expect("If all goes well there should be a single value") {
                    ExpressionValue::Operand(value) => Ok(*value),
                    ExpressionValue::Operator(_) => unreachable!(),
                };
            }
        }   
    Err(EvaluateLayerError::InvalidExpressionValueAmount)
    }
}
