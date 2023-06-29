pub mod expression_value;
use expression_value::*;

fn main() {
    println!("Hello, world!");
    let layer_one = Box::new(vec![RecursiveExpression::<f32>::Value(ExpressionValue::Operand(1.0)), RecursiveExpression::<f32>::Value(ExpressionValue::Operator(MathOperator::Add)), RecursiveExpression::<f32>::Value(ExpressionValue::Operand(1.0))]);
    let mut layer_two: Layer<f32> = vec![RecursiveExpression::<f32>::Value(ExpressionValue::Operand(1.0)), RecursiveExpression::<f32>::Value(ExpressionValue::Operator(MathOperator::Add)), RecursiveExpression::<f32>::Expression(layer_one)];
    dbg!(&layer_two); 

    let test = vec![0;10];

}

