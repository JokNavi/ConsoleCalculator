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
pub enum EquationItem<V> {
    Operand(V),
    Operator(MathOperator),
}

impl<V> EquationItem<V> {
    pub fn operand(self) -> Option<V> {
        match self {
            EquationItem::Operand(operand) => Some(operand),
            EquationItem::Operator(_) => None,
        }
    }
    pub fn operator(self) -> Option<MathOperator> {
        match self {
            EquationItem::Operand(_) => None,
            EquationItem::Operator(operator) => Some(operator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpressionEnum<V> {
    Value(EquationItem<V>),
    Expression(Box<Expression<V>>),
}

pub type Expression<V> = Vec<ExpressionEnum<V>>;

#[derive(Debug, PartialEq)]
pub enum ExpressionError {
    ExpectedOperand,
    ExpectedOperator,
    EmptyExpressionLayer,
    InvalidExpressionValueCount,
}

pub trait TryPushEpression<V> {
    fn try_push(&mut self, item: ExpressionEnum<V>) -> Result<(), ExpressionError>;
}

impl<V> TryPushEpression<V> for Vec<ExpressionEnum<V>> {
    fn try_push(&mut self, item: ExpressionEnum<V>) -> Result<(), ExpressionError> {
        let last_item = self
            .last()
            .or(Some(&ExpressionEnum::Value(EquationItem::Operator(
                MathOperator::Add,
            ))))
            .unwrap();

        if matches!(item, ExpressionEnum::Value(EquationItem::Operator(_))) {
            if matches!(last_item, &ExpressionEnum::Value(EquationItem::Operator(_))) {
                return Err(ExpressionError::ExpectedOperand);
            }
        } else if matches!(last_item, &ExpressionEnum::Value(EquationItem::Operand(_)))
            || matches!(last_item, &ExpressionEnum::Expression(_))
        {
            return Err(ExpressionError::ExpectedOperator);
        }
        self.push(item);
        Ok(())
    }
}

pub trait TryPushEquation<V> {
    fn try_push(&mut self, item: EquationItem<V>) -> Result<(), ExpressionError>;
}

impl<V> TryPushEquation<V> for Vec<EquationItem<V>> {
    fn try_push(&mut self, item: EquationItem<V>) -> Result<(), ExpressionError> {
        let last_item = self
            .last()
            .unwrap_or(&EquationItem::Operator(MathOperator::Add));

        match item {
            EquationItem::Operand(_) if matches!(last_item, EquationItem::Operand(_)) => {
                return Err(ExpressionError::ExpectedOperator)
            }
            EquationItem::Operator(_) if matches!(last_item, EquationItem::Operator(_)) => {
                return Err(ExpressionError::ExpectedOperand)
            }
            other => {
                self.push(other);
                return Ok(());
            }
        }
    }
}

trait EvaluateOperator {
    fn eval_operators(&self, selected_operators: &[MathOperator]) -> Result<Self, ExpressionError>
    where
        Self: Sized;
}

impl<V: Float + Debug> EvaluateOperator for Vec<EquationItem<V>> {
    fn eval_operators(&self, selected_operators: &[MathOperator]) -> Result<Self, ExpressionError>
    where
        Self: Sized,
    {
        let mut new_layer: Vec<EquationItem<V>> = Vec::new();

        new_layer.push(EquationItem::Operand(self.first().ok_or(ExpressionError::EmptyExpressionLayer)?.operand().ok_or(ExpressionError::ExpectedOperand)?));

        let self_iter = self[1..].chunks_exact(2);

        for current_equation in self_iter.clone() {
            let operator = current_equation
                .get(0)
                .ok_or(ExpressionError::ExpectedOperator)?
                .operator()
                .ok_or(ExpressionError::ExpectedOperator)?;

            if selected_operators.contains(&operator) {
                let left_operand = new_layer
                    .pop()
                    .ok_or(ExpressionError::ExpectedOperand)?
                    .operand()
                    .ok_or(ExpressionError::ExpectedOperand)?;
                let right_operand = current_equation
                    .get(1)
                    .ok_or(ExpressionError::ExpectedOperand)?
                    .operand()
                    .ok_or(ExpressionError::ExpectedOperand)?;
                let solution = match operator {
                    MathOperator::Add => left_operand.add(right_operand),
                    MathOperator::Sub => left_operand.sub(right_operand),
                    MathOperator::Mul => left_operand.mul(right_operand),
                    MathOperator::Div => left_operand.div(right_operand),
                    MathOperator::Rem => left_operand.rem(right_operand),
                    MathOperator::Pow => left_operand.powf(right_operand),
                };
                new_layer.push(EquationItem::Operand(solution));
            } else {
                new_layer.try_push(
                    *current_equation
                        .get(0)
                        .ok_or(ExpressionError::ExpectedOperand)?,
                )?;
                new_layer.try_push(
                    *current_equation
                        .get(1)
                        .ok_or(ExpressionError::ExpectedOperand)?,
                )?;
            }
        }

        match self_iter.remainder().get(0) {
            Some(EquationItem::Operand(_)) => Err(ExpressionError::ExpectedOperator),
            Some(EquationItem::Operator(_)) => Err(ExpressionError::ExpectedOperand),
            None => Ok(()),
        }?;

        Ok(new_layer)
    }
}

pub trait SolveExpression<V> {
    const ORDER_OF_OPERATIONS: [&'static [MathOperator]; 3] = [
        &[MathOperator::Pow],
        &[MathOperator::Mul, MathOperator::Div, MathOperator::Rem],
        &[MathOperator::Add, MathOperator::Sub],
    ];

    fn solve(&self) -> Result<V, ExpressionError>;
}

impl<V: Float + Debug> SolveExpression<V> for Expression<V> {
    const ORDER_OF_OPERATIONS: [&'static [MathOperator]; 3] = [
        &[MathOperator::Pow],
        &[MathOperator::Mul, MathOperator::Div, MathOperator::Rem],
        &[MathOperator::Add, MathOperator::Sub],
    ];

    fn solve(&self) -> Result<V, ExpressionError> {
        let mut equation_items: Vec<EquationItem<V>> = self
            .iter()
            .map(|value_or_layer| match value_or_layer {
                ExpressionEnum::Value(value) => Ok(*value),
                ExpressionEnum::Expression(layer) => {
                    layer.solve().map(|value| EquationItem::Operand(value))
                }
            })
            .collect::<Result<Vec<EquationItem<V>>, ExpressionError>>()?;

        for current_order_of_operations_operators in Self::ORDER_OF_OPERATIONS.iter() {
            equation_items =
                equation_items.eval_operators(current_order_of_operations_operators)?;
        }

        equation_items
            .get(0)
            .ok_or(ExpressionError::EmptyExpressionLayer)?
            .operand()
            .ok_or(ExpressionError::ExpectedOperand)
    }
}

#[cfg(test)]
mod evaluate_tests {
    use super::*;

    #[test]
    fn eval_operators_ok() {
        let test_expression: Vec<EquationItem<f32>> = vec![
            EquationItem::Operand(1.0),
            EquationItem::Operator(MathOperator::Add),
            EquationItem::Operand(1.0),
            EquationItem::Operator(MathOperator::Sub),
            EquationItem::Operand(1.0),
        ];
        let test_expression_answer: Vec<EquationItem<f32>> = vec![
            EquationItem::Operand(2.0),
            EquationItem::Operator(MathOperator::Sub),
            EquationItem::Operand(1.0),
        ];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap(),
            test_expression_answer
        );
        let test_expression: Vec<EquationItem<f32>> = vec![EquationItem::Operand(1.0)];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap(),
            vec![EquationItem::Operand(1.0)]
        );
    }

    #[test]
    fn eval_operators_err() {

        let test_expression: Vec<EquationItem<f32>> = vec![
            EquationItem::Operand(1.0),
            EquationItem::Operator(MathOperator::Add),
            EquationItem::Operand(1.0),
            EquationItem::Operator(MathOperator::Sub),
        ];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap_err(),
            ExpressionError::ExpectedOperand
        );

        let test_expression: Vec<EquationItem<f32>> = vec![];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap_err(),
            ExpressionError::EmptyExpressionLayer
        );

        let test_expression: Vec<EquationItem<f32>> = vec![
            EquationItem::Operand(1.0),
            EquationItem::Operator(MathOperator::Add),
            EquationItem::Operand(1.0),
            EquationItem::Operand(1.0),
        ];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap_err(),
            ExpressionError::ExpectedOperator
        );
        let test_expression: Vec<EquationItem<f32>> = vec![
            EquationItem::Operator(MathOperator::Add),
        ];
        assert_eq!(
            test_expression
                .eval_operators(&[MathOperator::Add])
                .unwrap_err(),
            ExpressionError::ExpectedOperand
        );
        
        
    }

    #[test]
    fn try_push() {
        let mut test_expression: Vec<ExpressionEnum<f32>> = vec![];
        assert_eq!(
            test_expression
                .try_push(ExpressionEnum::Value(EquationItem::Operator(
                    MathOperator::Add
                )))
                .unwrap_err(),
            ExpressionError::ExpectedOperand
        );
        assert_eq!(test_expression.last(), None);
        assert!(test_expression
            .try_push(ExpressionEnum::Value(EquationItem::Operand(5.0)))
            .is_ok());
        assert_eq!(
            test_expression.last(),
            Some(&ExpressionEnum::Value(EquationItem::Operand(5.0f32)))
        );
        assert!(test_expression
            .try_push(ExpressionEnum::Value(EquationItem::Operator(
                MathOperator::Add
            )))
            .is_ok());
        assert_eq!(
            test_expression.last(),
            Some(&ExpressionEnum::Value(EquationItem::Operator(
                MathOperator::Add
            )))
        );
    }

    #[test]
    fn solve_ok() {
        let expression: Vec<ExpressionEnum<f32>> =  vec![EquationItem::Operand(1.0), EquationItem::Operator(MathOperator::Add), EquationItem::Operand(1.0)].into_iter().map(|f| ExpressionEnum::Value(f)).collect();
        let nested_expression = vec![ExpressionEnum::Expression(Box::new(expression)),  ExpressionEnum::Value(EquationItem::Operator(MathOperator::Add)),  ExpressionEnum::Value(EquationItem::Operand(1.0))];
        assert_eq!(nested_expression.solve().unwrap(), 3.0);
        }

    #[test]
    fn solve_err() {
            let nested_expression = vec![ExpressionEnum::Expression(Box::new(vec![])),  ExpressionEnum::Value(EquationItem::Operator(MathOperator::Add)),  ExpressionEnum::Value(EquationItem::Operand(1.0))];
            assert_eq!(nested_expression.solve().unwrap_err(), ExpressionError::EmptyExpressionLayer);
        }
}
