use crate::expression_item::ExpressionItem;

pub trait Eval {
    fn eval(&self) -> f32;
}

impl Eval for ExpressionItem {
    fn eval(&self) -> f32 {
        todo!()
    }
}

impl Eval for String {
    fn eval(&self) -> f32 {
        todo!()
    }
}