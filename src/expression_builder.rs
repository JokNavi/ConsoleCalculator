use crate::{
    expression_item::{ExpressionItem, Parentheses},
    operator::Operator,
};
use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct ExpressionBuilder<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> ExpressionBuilder<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            chars: expression.chars().peekable(),
        }
    }

    fn get_operand(&mut self) -> Option<f32> {
        let mut number_string = String::new();
        if let Some(operator) = self.get_operator() {
            number_string.push(char::from(&operator));
        }
        while let Some(number_item) = self.chars.next_if(|number_item| {
            number_item.is_ascii_digit() || (number_item == &'.' && !number_string.contains('.'))
        }) {
            number_string.push(number_item);
        }
        number_string.parse::<f32>().ok()
    }

    fn get_operator(&mut self) -> Option<Operator> {
        self.chars
            .next_if(|character| matches!(character, '^' | '*' | '/' | '%' | '+' | '-'))
            .and_then(|operator| Operator::new(&operator).ok())
    }

    fn skip_whitespace(&mut self) {
        while self
            .chars
            .next_if(|character| character.is_ascii_whitespace())
            .is_some()
        {}
    }

    fn get_next(
        &mut self,
        vec_last: Option<&ExpressionItem>,
    ) -> Result<ExpressionItem, ExpressionBuilderError> {
        let expression_item: ExpressionItem = match vec_last {
            None | Some(ExpressionItem::Operator(_)) => self
                .get_operand()
                .map_or(self.get_parentheses()?, |operand| {
                    Some(ExpressionItem::from(&operand))
                })
                .ok_or(ExpressionBuilderError::ExpectedOperand)?,
            Some(ExpressionItem::Operand(_)) | Some(ExpressionItem::Parentheses(_)) => {
                ExpressionItem::from(
                    self.get_operator()
                        .ok_or(ExpressionBuilderError::ExpectedOperator)?,
                )
            }
        };
        Ok(expression_item)
    }

    fn get_parentheses(&mut self) -> Result<Option<ExpressionItem>, ExpressionBuilderError> {
        let mut parentheses: Vec<ExpressionItem> = Vec::new();
        if self.chars.next_if_eq(&'(').is_none() {
            return Ok(None);
        }
        self.skip_whitespace();
        while self.chars.next_if_eq(&')').is_none() {
            self.skip_whitespace();
            parentheses.push(self.get_next(parentheses.last())?);
            if self.chars.peek().is_none() {
                return Err(ExpressionBuilderError::ExpectedClosingParentheses);
            }
        }
        Ok(Some(ExpressionItem::Parentheses(Parentheses::new(
            parentheses,
        ))))
    }

    pub fn get_expression(&mut self) -> Result<Vec<ExpressionItem>, ExpressionBuilderError> {
        let mut expressions: Vec<ExpressionItem> = vec![];
        while self.chars.peek().is_some() {
            expressions.push(self.get_next(expressions.last())?);
        }
        if matches!(expressions.last(), Some(ExpressionItem::Operator(_))) || expressions.last().is_none() {
            return Err(ExpressionBuilderError::ExpectedOperand);
        }
        Ok(expressions)
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpressionBuilderError {
    ExpectedClosingParentheses,
    ExpectedOperand,
    ExpectedOperator,
}

impl Display for ExpressionBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionBuilderError::ExpectedClosingParentheses => {
                write!(f, "Expected closing parentheses.")
            }
            ExpressionBuilderError::ExpectedOperand => write!(f, "Expected operand."),
            ExpressionBuilderError::ExpectedOperator => write!(f, "Expected operator."),
        }
    }
}

impl Error for ExpressionBuilderError {}

#[cfg(test)]
mod expresion_builder_tests {
    use super::*;

    #[test]
    fn get_operator() {
        let operator_chars = &['+', '-', '*', '/', '^', '%'];
        let operator_enums = operator_chars.map(|operator| Operator::new(&operator).unwrap());
        for (operator_char, operator_enum) in operator_chars.iter().zip(operator_enums) {
            assert!(ExpressionBuilder::new(&operator_char.to_string())
                .get_operator()
                .is_some_and(|operator| operator == operator_enum));
        }
    }

    #[test]
    fn get_operand() {
        for sign in &['+', '-'] {
            for number in 0..100u16 {
                for rest in 0..100 {
                    let expression = format!("{}{}.{}", sign, number, rest);
                    assert!(ExpressionBuilder::new(&expression)
                        .get_operand()
                        .is_some_and(|operand| operand == expression.parse::<f32>().unwrap()));
                }
            }
        }
        assert!(ExpressionBuilder::new(".").get_operand().is_none());
        assert!(ExpressionBuilder::new(".0")
            .get_operand()
            .is_some_and(|some| some == 0.0));
        assert!(ExpressionBuilder::new("0.")
            .get_operand()
            .is_some_and(|some| some == 0.0));
    }

    struct F32Iterator {
        sign: usize,
        number: u16,
        rest: u16,
        max: u16,
    }

    impl F32Iterator {
        pub fn new(max: u16) -> Self {
            Self {
                sign: 0,
                number: 0,
                rest: 0,
                max,
            }
        }
    }

    impl Iterator for F32Iterator {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.sign >= 2 {
                return None;
            }
            const SIGNS: [&str; 2] = ["+", "-"];
            let number = format!("{}{}.{}", SIGNS[self.sign], self.number, self.rest)
                .parse::<f32>()
                .ok();
            self.rest += 1;
            if self.rest >= self.max {
                self.rest = 0;
                self.number += 1;
            }
            if self.number >= self.max {
                self.number = 0;
                self.sign += 1;
            }
            if self.sign >= 2 {
                return None;
            }
            number
        }
    }

    #[test]
    fn get_next_operand() {
        let operand = ExpressionItem::from(&1.0);
        let operator = ExpressionItem::try_from('+').unwrap();
        let parentheses = ExpressionItem::from(vec![
            ExpressionItem::from(&1.0),
            ExpressionItem::try_from('+').unwrap(),
            ExpressionItem::from(&2.0),
        ]);

        assert!(ExpressionBuilder::new(&operand.to_string())
            .get_next(None)
            .is_ok_and(|ok| ok == operand));
        assert!(ExpressionBuilder::new(&operand.to_string())
            .get_next(Some(&operand))
            .is_err_and(&|err| err == ExpressionBuilderError::ExpectedOperator));
        assert!(ExpressionBuilder::new(&operand.to_string())
            .get_next(Some(&operator))
            .is_ok_and(|ok| ok == operand));
        assert!(ExpressionBuilder::new(&operand.to_string())
            .get_next(Some(&parentheses))
            .is_err_and(&|err| err == ExpressionBuilderError::ExpectedOperator));
    }

    #[test]
    fn get_next_operator() {
        let operand = ExpressionItem::from(&1.0);
        let operator = ExpressionItem::try_from('+').unwrap();
        let parentheses = ExpressionItem::from(vec![
            ExpressionItem::from(&1.0),
            ExpressionItem::try_from('+').unwrap(),
            ExpressionItem::from(&2.0),
        ]);

        assert!(ExpressionBuilder::new(&operator.to_string())
            .get_next(None)
            .is_err_and(&|err| err == ExpressionBuilderError::ExpectedOperand));

        assert!(ExpressionBuilder::new(&operator.to_string())
            .get_next(Some(&operand))
            .is_ok_and(|ok| ok == operator));

        assert!(ExpressionBuilder::new(&operator.to_string())
            .get_next(Some(&operator))
            .is_err_and(&|err| err == ExpressionBuilderError::ExpectedOperand));

        assert!(ExpressionBuilder::new(&operator.to_string())
            .get_next(Some(&parentheses))
            .is_ok_and(|ok| ok == operator));
    }

    #[test]
    fn get_next_parentheses() {
        let operand = ExpressionItem::from(&1.0);
        let operator = ExpressionItem::try_from('+').unwrap();
        let parentheses = ExpressionItem::from(vec![
            ExpressionItem::from(&1.0),
            ExpressionItem::try_from('+').unwrap(),
            ExpressionItem::from(&2.0),
        ]);

        assert!(ExpressionBuilder::new(&parentheses.to_string())
            .get_next(None)
            .is_ok_and(|ok| ok == parentheses));
        assert!(ExpressionBuilder::new(&parentheses.to_string())
            .get_next(Some(&operand))
            .is_err_and(&|err| err == ExpressionBuilderError::ExpectedOperator));
        assert!(ExpressionBuilder::new(&parentheses.to_string())
            .get_next(Some(&operator))
            .is_ok_and(|ok| ok == parentheses));
        assert!(ExpressionBuilder::new(&parentheses.to_string())
            .get_next(Some(&parentheses))
            .is_err_and(&|err| err == ExpressionBuilderError::ExpectedOperator));
    }

    #[test]
    fn get_parentheses_operands_operators() {
        let mut parentheses: Vec<ExpressionItem> = Vec::new();
        for operator in &['+', '-', '*', '/', '^', '%'] {
            for operand in F32Iterator::new(10) {
                parentheses.push(ExpressionItem::from(&operand));
                let parentheses_item = ExpressionItem::from(parentheses.clone());
                assert!(ExpressionBuilder::new(&format!("{}", &parentheses_item))
                    .get_parentheses()
                    .is_ok_and(|ok| ok.is_some_and(|some| { some == parentheses_item })));
                parentheses.push(Operator::new(operator).unwrap().into());
            }
        }
    }

    #[test]
    fn get_parentheses() {
        let parentheses = ExpressionItem::from(vec![
            ExpressionItem::from(&1.0),
            ExpressionItem::try_from('+').unwrap(),
            ExpressionItem::from(&2.0),
        ]);
        assert!(ExpressionBuilder::new(&format!("{}", parentheses))
            .get_parentheses()
            .is_ok_and(|ok| ok.is_some_and(|some| some == parentheses)));
        let expression_item = ExpressionItem::from(vec![
            parentheses,
            ExpressionItem::try_from('-').unwrap(),
            ExpressionItem::from(&2.0),
        ]);
        assert!(ExpressionBuilder::new(&format!("{}", expression_item))
            .get_parentheses()
            .is_ok_and(|ok| ok.is_some_and(|some| some == expression_item)));

        let single_vec_expression = ExpressionItem::from(vec![ExpressionItem::from(&1.0)]);
        assert!(
            ExpressionBuilder::new(&format!("{}", single_vec_expression))
                .get_parentheses()
                .is_ok_and(|ok| ok.is_some_and(|some| some == single_vec_expression))
        );
        assert!(
            ExpressionBuilder::new("(1")
                .get_parentheses()
                .is_err_and(|err| err == ExpressionBuilderError::ExpectedClosingParentheses)
        );
    }

    #[test]
    fn get_expression() {
        let parentheses = ExpressionItem::from(vec![
            ExpressionItem::from(&1.0),
            ExpressionItem::try_from('+').unwrap(),
            ExpressionItem::from(&2.0),
        ]);
        let expression_item = ExpressionItem::from(vec![
            parentheses,
            ExpressionItem::try_from('-').unwrap(),
            ExpressionItem::from(&2.0),
        ]);
        assert!(ExpressionBuilder::new(&format!("{}", expression_item))
            .get_expression()
            .is_ok_and(|ok| ok == vec![expression_item]));
    }
}
