use crate::cheap_expression_enum::Reduce;

pub mod cheap_expression_enum;

fn main() {
    println!("Hello, world!");
    println!("{}", "1+1".to_string().try_reduced().unwrap());
    println!("{}", "(2*2)/2".to_string().try_reduced().unwrap());
    println!("{}", "(-2*2)/4*(2)".to_string().try_reduced().unwrap());
}

