use clap::{Command, arg, ArgMatches};

use crate::eval::Evaluate;
const VERSION: &str = "1.0.0";

pub mod operator;
pub mod expression_item;
pub mod eval;
pub mod expression_builder;

fn get_cmd() -> ArgMatches {
    Command::new("calc").about("Calculator").args(&[
        arg!(-v --version "Prints the version number"),
        arg!(-e --equation <String> "Equation")
    ]).get_matches()
} 

fn main() {
    let matches = get_cmd();
    if matches.get_one::<bool>("version").is_some_and(|bool| *bool) {
        println!("{}", VERSION);
    };
    if let Some(equation) = matches.get_one::<String>("equation") {
        match equation.as_str().eval() {
            Ok(result) => println!("{}", result.unwrap()),
            Err(err) => println!("{}", err)
        }
    }
}
