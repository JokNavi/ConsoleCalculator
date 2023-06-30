pub mod expression_value;
use expression_value::*;

fn main() {
    println!("Hello, world!");
    let layer_one = Box::new(vec![ExpressionEnum::<f32>::Value(EquationItem::Operand(1.0)), ExpressionEnum::<f32>::Value(EquationItem::Operator(MathOperator::Add)), ExpressionEnum::<f32>::Value(EquationItem::Operand(1.0))]);
    let mut layer_two: Expression<f32> = vec![ExpressionEnum::<f32>::Value(EquationItem::Operand(1.0)), ExpressionEnum::<f32>::Value(EquationItem::Operator(MathOperator::Add)), ExpressionEnum::<f32>::Expression(layer_one)];
    dbg!(5 % 2); 

    let test = vec![0;10];

}

