use core::num;
use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
enum CheapEquationItem {
    Operand(f32),
    Operator(char),
    Parenthesis(Parenthesis),
}

pub type Parenthesis = Box<Vec<CheapEquationItem>>;

impl CheapEquationItem {
    pub fn operand(&self) -> Option<f32> {
        match self {
            CheapEquationItem::Operand(operand) => Some(*operand),
            CheapEquationItem::Operator(_) => None,
            CheapEquationItem::Parenthesis(_) => None,
        }
    }

    pub fn operator(&self) -> Option<&char> {
        match self {
            CheapEquationItem::Operand(_) => None,
            CheapEquationItem::Operator(operator)
                if matches!(operator, '^' | '*' | '/' | '%' | '+' | '-') =>
            {
                Some(operator)
            }
            CheapEquationItem::Parenthesis(_) => None,
            _ => None,
        }
    }

    pub fn parenthesis(&self) -> Option<&Parenthesis> {
        match self {
            CheapEquationItem::Operand(_) => None,
            CheapEquationItem::Operator(_) => None,
            CheapEquationItem::Parenthesis(parenthesis) => Some(parenthesis),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReduceCheapEquationItemError {
    ExpectedOperand,
    ExpectedOperator,
}

pub trait Reduce {
    type ReduceError;
    /// returns the shorstest representation of self.
    fn try_reduced(&self) -> Result<Self, Self::ReduceError>
    where
        Self: Sized;
}

impl Reduce for Vec<CheapEquationItem> {
    type ReduceError = ReduceCheapEquationItemError;

    fn try_reduced(&self) -> Result<Self, Self::ReduceError> {
        const OPERATION_ORDER: [&'static [char]; 3] = [&['^'], &['*', '/', '%'], &['+', '-']];
        if matches!(self.as_slice(), [CheapEquationItem::Operand(_)]) {
            return Err(ReduceCheapEquationItemError::ExpectedOperator);
        }
        let mut expression = self.clone();
        for operations in OPERATION_ORDER.into_iter() {
            expression = expression
                .get(1..)
                .ok_or(ReduceCheapEquationItemError::ExpectedOperand)?
                .chunks(2)
                .fold(
                    Ok(vec![CheapEquationItem::Operand(
                        expression
                            .first()
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperand)?
                            .try_reduced()?
                            .operand()
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperand)?,
                    )]),
                    |collector_result: Result<
                        Vec<CheapEquationItem>,
                        ReduceCheapEquationItemError,
                    >,
                     chunk: &[CheapEquationItem]| {
                        let mut collector = collector_result?;
                        let left_operand = collector
                            .pop()
                            .unwrap()
                            .try_reduced()?
                            .operand()
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperand)?;
                        let operator = *chunk
                            .get(0)
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperator)?
                            .try_reduced()?
                            .operator()
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperator)?;
                        let right_operand = chunk
                            .get(1)
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperand)?
                            .try_reduced()?
                            .operand()
                            .ok_or(ReduceCheapEquationItemError::ExpectedOperand)?;
                        if operations.contains(&operator) {
                            let answer = match operator {
                                '^' => Ok(left_operand.powf(right_operand)),
                                '*' => Ok(left_operand * right_operand),
                                '/' => Ok(left_operand / right_operand),
                                '%' => Ok(left_operand % right_operand),
                                '+' => Ok(left_operand + right_operand),
                                '-' => Ok(left_operand - right_operand),
                                _ => unreachable!(),
                            }?;
                            collector.push(CheapEquationItem::Operand(answer));
                        } else {
                            collector.push(CheapEquationItem::Operand(left_operand));
                            collector.push(CheapEquationItem::Operator(operator));
                            collector.push(CheapEquationItem::Operand(right_operand));
                        }
                        Ok(collector)
                    },
                )?;
        }
        Ok(expression)
    }
}

impl Reduce for CheapEquationItem {
    type ReduceError = ReduceCheapEquationItemError;
    fn try_reduced(&self) -> Result<Self, Self::ReduceError> {
        match self {
            CheapEquationItem::Operand(operand) => Ok(CheapEquationItem::Operand(*operand)),
            CheapEquationItem::Operator(operator) => Ok(CheapEquationItem::Operator(*operator)),
            CheapEquationItem::Parenthesis(parenthesis) => Ok(CheapEquationItem::Operand(
                parenthesis
                    .try_reduced()?
                    .get(0)
                    .expect("I coded .evaluate() well.")
                    .operand()
                    .expect("I did rigorous testing on .evaluate()."),
            )),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum EvaluteStringError {
    EvaluteError(ReduceCheapEquationItemError),
    UnClosedParenthesis,
    EmptyParenthesis,
}

trait GetEquationItem {
    fn get_operand(&mut self) -> Option<CheapEquationItem>;
    fn get_operator(&mut self) -> Option<CheapEquationItem>;
    fn get_parenthesis(&mut self) -> Result<CheapEquationItem, EvaluteStringError>;
    fn skip_whitespace(&mut self) -> ();
    fn get_next(
        &mut self,
        layer: &Vec<CheapEquationItem>,
    ) -> Option<Result<CheapEquationItem, EvaluteStringError>>;
}

impl GetEquationItem for Peekable<Chars<'_>> {
    fn get_operand(&mut self) -> Option<CheapEquationItem> {
        let mut number_string = String::new();
        if let Some(character) = self.next_if(|character| character == &'-' || character == &'+') {
            number_string.push(character);
        };

        while let Some(character) = self.next_if(|character| {
            character.is_ascii_digit() || (character == &'.' && !number_string.contains('.'))
        }) {
            number_string.push(character);
        }
        number_string
            .parse::<f32>()
            .ok()
            .map(|number| CheapEquationItem::Operand(number))
    }

    fn get_operator(&mut self) -> Option<CheapEquationItem> {
        self.next_if(|character| matches!(character, '^' | '*' | '/' | '%' | '+' | '-'))
            .map(|operator| CheapEquationItem::Operator(operator))
    }

    fn get_next(
        &mut self,
        layer: &Vec<CheapEquationItem>,
    ) -> Option<Result<CheapEquationItem, EvaluteStringError>> {
        match layer.last() {
            Some(CheapEquationItem::Operand(_)) | Some(CheapEquationItem::Parenthesis(_)) => {
                if self.next_if_eq(&')').is_some() {
                    return None;
                };
                Some(self.get_operator().ok_or(EvaluteStringError::EvaluteError(
                    ReduceCheapEquationItemError::ExpectedOperator,
                )))
            }
            Some(CheapEquationItem::Operator(_)) | None => {
                if self.next_if_eq(&'(').is_some() {
                    return Some(self.get_parenthesis());
                };
                Some(self.get_operand().ok_or(EvaluteStringError::EvaluteError(
                    ReduceCheapEquationItemError::ExpectedOperand,
                )))
            }
        }
    }

    fn get_parenthesis(&mut self) -> Result<CheapEquationItem, EvaluteStringError> {
        //"(-1.5)+(1.0)"
        let mut layer: Vec<CheapEquationItem> = Vec::new();

        while self.peek().is_some() {
            self.skip_whitespace();
            if let Some(next) = self.get_next(&layer) {
                layer.push(next?);
            } else {
                break;
            }
        }
        Ok(CheapEquationItem::Parenthesis(Box::new(layer)))
    }

    fn skip_whitespace(&mut self) -> () {
        while let Some(_) = self.next_if(|character| character.is_ascii_whitespace()) {}
    }
}

impl TryFrom<&str> for CheapEquationItem {
    type Error = EvaluteStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod reduce_tests {
    use super::*;

    #[test]
    fn try_reduced() {
        let expression: Vec<CheapEquationItem> = vec![
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
            CheapEquationItem::Parenthesis(Box::new(vec![
                CheapEquationItem::Operand(1.0),
                CheapEquationItem::Operator('+'),
                CheapEquationItem::Operand(1.0),
            ])),
            CheapEquationItem::Operator('-'),
            CheapEquationItem::Operand(1.0),
        ];
        assert_eq!(
            expression.try_reduced().unwrap(),
            vec![CheapEquationItem::Operand(2.0)]
        );
        let nested_expression = vec![
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
            CheapEquationItem::Parenthesis(Box::new(expression)),
        ];
        assert_eq!(
            nested_expression.try_reduced().unwrap(),
            vec![CheapEquationItem::Operand(3.0)]
        );

        let broken_expression: Vec<CheapEquationItem> = vec![
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
        ];
        assert_eq!(
            broken_expression.try_reduced().unwrap_err(),
            ReduceCheapEquationItemError::ExpectedOperand,
        );

        let broken_expression: Vec<CheapEquationItem> = vec![CheapEquationItem::Operand(1.0)];
        assert_eq!(
            broken_expression.try_reduced().unwrap_err(),
            ReduceCheapEquationItemError::ExpectedOperator,
        );
        let broken_expression: Vec<CheapEquationItem> = vec![
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
        ];
        assert_eq!(
            broken_expression.try_reduced().unwrap_err(),
            ReduceCheapEquationItemError::ExpectedOperand,
        );

        let broken_expression: Vec<CheapEquationItem> = vec![
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operand(1.0),
        ];

        assert_eq!(
            broken_expression.try_reduced().unwrap_err(),
            ReduceCheapEquationItemError::ExpectedOperator,
        );
    }
}

#[cfg(test)]
mod get_equation_item_tests {
    use super::*;

    #[test]
    fn match_number_test() {
        assert_eq!(
            "1".chars().peekable().get_operand(),
            Some(CheapEquationItem::Operand(1f32))
        );
        assert_eq!(
            "1.0".chars().peekable().get_operand(),
            Some(CheapEquationItem::Operand(1f32))
        );
        assert_eq!(
            "-1.15".chars().peekable().get_operand(),
            Some(CheapEquationItem::Operand(-1.15f32))
        );
        assert_eq!(
            "1.5".chars().peekable().get_operand(),
            Some(CheapEquationItem::Operand(1.5f32))
        );
    }

    #[test]
    fn get_parenthesis_test() {
        assert_eq!(
            "1".chars().peekable().get_parenthesis().unwrap(),
            CheapEquationItem::Parenthesis(Box::new(vec![CheapEquationItem::Operand(1f32)]))
        );
        assert_eq!(
            "-1.5+1.0".chars().peekable().get_parenthesis().unwrap(),
            CheapEquationItem::Parenthesis(Box::new(vec![
                CheapEquationItem::Operand(-1.5f32),
                CheapEquationItem::Operator('+'),
                CheapEquationItem::Operand(1.0f32),
            ]))
        );
        assert_eq!(
            "(-1.5)+(1.0)".chars().peekable().get_parenthesis().unwrap(),
            CheapEquationItem::Parenthesis(Box::new(vec![
                CheapEquationItem::Parenthesis(Box::new(vec![CheapEquationItem::Operand(-1.5f32)])),
                CheapEquationItem::Operator('+'),
                CheapEquationItem::Parenthesis(Box::new(vec![CheapEquationItem::Operand(1f32)])),
            ]))
        );
        assert_eq!(
            "-1.5+(1.0*2)".chars().peekable().get_parenthesis().unwrap(),
            CheapEquationItem::Parenthesis(Box::new(vec![
                CheapEquationItem::Operand(-1.5f32),
                CheapEquationItem::Operator('+'),
                CheapEquationItem::Parenthesis(Box::new(vec![CheapEquationItem::Operand(1f32), CheapEquationItem::Operator('*'), CheapEquationItem::Operand(2f32)])),
            ]))
        );
    }
}
