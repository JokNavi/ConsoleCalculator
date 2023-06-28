#[derive(Debug, PartialEq)]
pub enum MathOperators {
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
    Operator(MathOperators),
}

#[derive(Debug, PartialEq)]
pub enum RecursiveExpression<V> {
    Value(ExpressionValue<V>),
    Expression(Box<Layer<V>>),
}

pub type Layer<V> = Vec<RecursiveExpression<V>>;