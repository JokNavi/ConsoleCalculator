#[derive(Debug, PartialEq, Clone)]
enum CheapEquationItem {
    Operand(f32),
    Operator(char),
    Parenthesis(Parenthesis),
}

type Parenthesis = Box<Vec<CheapEquationItem>>;

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
pub enum EvaluteError {
    ExpectedOperand,
    ExpectedOperator,
}

pub trait Evaluate {
    /// evaluates an expression and returns the result.
    fn evaluate(&self) -> Result<Self, EvaluteError>
    where
        Self: Sized;
}

impl Evaluate for Vec<CheapEquationItem> {
    fn evaluate(&self) -> Result<Self, EvaluteError> {
        const OPERATION_ORDER: [&'static [char]; 3] = [&['^'], &['*', '/', '%'], &['+', '-']];
        if matches!(self.as_slice(), [CheapEquationItem::Operand(_)]) {
            return Err(EvaluteError::ExpectedOperator);
        }
        let mut expression = self.clone();
        for operations in OPERATION_ORDER.into_iter() {
            expression = expression
                .get(1..)
                .ok_or(EvaluteError::ExpectedOperand)?
                .chunks(2)
                .fold(
                    Ok(vec![CheapEquationItem::Operand(
                        expression
                            .first()
                            .ok_or(EvaluteError::ExpectedOperand)?
                            .evaluate()?
                            .operand()
                            .ok_or(EvaluteError::ExpectedOperand)?,
                    )]),
                    |collector_result: Result<Vec<CheapEquationItem>, EvaluteError>,
                     chunk: &[CheapEquationItem]| {
                        let mut collector = collector_result?;
                        let left_operand = collector
                            .pop()
                            .unwrap()
                            .evaluate()?
                            .operand()
                            .ok_or(EvaluteError::ExpectedOperand)?;
                        dbg!(&left_operand);
                        let operator = *chunk
                            .get(0)
                            .ok_or(EvaluteError::ExpectedOperator)?
                            .evaluate()?
                            .operator()
                            .ok_or(EvaluteError::ExpectedOperator)?;
                        dbg!(&operator);
                        let right_operand = chunk
                            .get(1)
                            .ok_or(EvaluteError::ExpectedOperand)?
                            .evaluate()?
                            .operand()
                            .ok_or(EvaluteError::ExpectedOperand)?;
                        dbg!(&right_operand);
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
                            dbg!(&answer);
                            collector.push(CheapEquationItem::Operand(answer));
                        } else {
                            collector.push(CheapEquationItem::Operand(left_operand));
                            collector.push(CheapEquationItem::Operator(operator));
                            collector.push(CheapEquationItem::Operand(right_operand));
                        }
                        dbg!(&collector);
                        Ok(collector)
                    },
                )?;
        }
        Ok(expression)
    }
}

impl Evaluate for CheapEquationItem {
    fn evaluate(&self) -> Result<Self, EvaluteError> {
        match self {
            CheapEquationItem::Operand(operand) => Ok(CheapEquationItem::Operand(*operand)),
            CheapEquationItem::Operator(operator) => Ok(CheapEquationItem::Operator(*operator)),
            CheapEquationItem::Parenthesis(parenthesis) => Ok(CheapEquationItem::Operand(
                parenthesis
                    .evaluate()?
                    .get(0)
                    .expect("I coded .evaluate() well.")
                    .operand()
                    .expect("I did rigorous testing on .evaluate()."),
            )),
        }
    }
}

#[cfg(test)]
mod cheap_equation_item_tests {
    use super::*;

    #[test]
    fn evaluate() {
         let expression: Vec<CheapEquationItem> = vec![
             CheapEquationItem::Operand(1.0),
             CheapEquationItem::Operator('+'),
             CheapEquationItem::Operand(1.0),
         ];
             assert_eq!(
                 expression.evaluate().unwrap(),
                 vec![CheapEquationItem::Operand(2.0)]
             );
         let nested_expression = vec![
              CheapEquationItem::Operand(1.0),
              CheapEquationItem::Operator('+'),
              CheapEquationItem::Parenthesis(Box::new(expression)),
         ];
          assert_eq!(
              nested_expression.evaluate().unwrap(),
              vec![CheapEquationItem::Operand(3.0)]
          );

        let broken_expression: Vec<CheapEquationItem> = vec![
            CheapEquationItem::Operand(1.0),
            CheapEquationItem::Operator('+'),
        ];
        assert_eq!(
            broken_expression.evaluate().unwrap_err(),
            EvaluteError::ExpectedOperand,
        );

          let broken_expression: Vec<CheapEquationItem> = vec![
              CheapEquationItem::Operand(1.0),
          ];
          assert_eq!(
              broken_expression.evaluate().unwrap_err(),
              EvaluteError::ExpectedOperator,
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
             broken_expression.evaluate().unwrap_err(),
             EvaluteError::ExpectedOperand,
         );
    }
}
