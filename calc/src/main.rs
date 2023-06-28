pub mod simple_calculator;
pub mod checked_operations;

fn main() {
    println!("Hello, world!");
    let test: f32 = 5.0;
    dbg!(10.0 <= 3.40282347E+38);
    assert_eq!(test >= f32::MAX, true);
    assert_eq!(test >= f32::INFINITY, false);
    assert_eq!(test == f32::NAN, false);
    assert_eq!(f32::NAN, f32::NAN);
   
}


