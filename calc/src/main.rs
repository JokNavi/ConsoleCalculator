use crate::evaluate_expression::{MathOperator, ExpressionLayer};
pub mod evaluate_expression;
pub mod checked_operations;

fn main() {
    println!("Hello, world!");

    let operations: (MathOperator, MathOperator) = (MathOperator::Add, MathOperator::Div);
    let operation = MathOperator::Add;
    let test: f32 = 0.0;
    let f32_nan: f32 = f32::NAN;
    dbg!(f32::NAN == f32::NAN);

    let ExpressionLayer = ExpressionLayer::Layer(Box::new(vec![ExpressionLayer::Number(5.0), ExpressionLayer::Operator(MathOperator::Add), ExpressionLayer::Layer(Box::new(vec![ExpressionLayer::Number(3.0), ExpressionLayer::Operator(MathOperator::Sub), ExpressionLayer::Number(1.0)]))]));
    let stringi = "5+2";
}


