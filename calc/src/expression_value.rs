use num_traits::Float;

#[derive(Debug, PartialEq)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}

#[derive(Debug, PartialEq)]
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
pub enum ExpressionSolveError {

}

impl<V: Float> RecursiveExpression<V> {
    const ORDER_OF_OPERATIONS: [&[MathOperator]; 3] = [&[MathOperator::Pow], &[MathOperator::Mul, MathOperator::Div, MathOperator::Rem], &[MathOperator::Add, MathOperator::Sub]];

    pub fn solve(&self) -> Result<V, ExpressionSolveError> {
        todo!()
    }
}