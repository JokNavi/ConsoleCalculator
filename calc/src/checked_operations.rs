use num_traits::Float;

#[derive(PartialEq, Debug)]
pub enum MathError {
    DivisionBy0,
    IntegerOverflow,
    IntegerUnderflow,
    //NegativeSquareRoot,
    NAN,
}

pub trait FloatCheckedOperations where 
Self: Sized + Float, {
    
    fn checked_add(&self, rhs: Self) -> Result<Self, MathError>
    {
        let nan = Self::nan();
        let infinity = Self::infinity();
        let neg_infity = Self::neg_infinity();
        let max = Self::max_value();
        let min = Self::min_value();

        let outcome = self.add(rhs);
        match outcome {
            _ if outcome >= infinity => unimplemented!(),
            _ if outcome <= neg_infity => unimplemented!(),
            _ if outcome >= max => Err(MathError::IntegerOverflow),
            _ if outcome <= min => Err(MathError::IntegerUnderflow),
            _ if outcome == nan => unimplemented!(),
            other => Ok(other),
        }
    }

    fn checked_sub(&self, rhs: Self) -> Result<Self, MathError>
    {
        let nan = Self::nan();
        let infinity = Self::infinity();
        let neg_infity = Self::neg_infinity();
        let max = Self::max_value();
        let min = Self::min_value();

        let outcome = self.sub(rhs);
        match outcome {
            _ if outcome >= infinity => unimplemented!(),
            _ if outcome <= neg_infity => unimplemented!(),
            _ if outcome >= max => Err(MathError::IntegerOverflow),
            _ if outcome <= min => Err(MathError::IntegerUnderflow),
            _ if outcome == nan => unimplemented!(),
            other => Ok(other),
        }
    }

    fn checked_mul(&self, rhs: Self) -> Result<Self, MathError>
    {
        let nan = Self::nan();
        let infinity = Self::infinity();
        let neg_infity = Self::neg_infinity();
        let max = Self::max_value();
        let min = Self::min_value();

        let outcome = self.mul(rhs);
        match outcome {
            _ if outcome >= infinity => unimplemented!(),
            _ if outcome <= neg_infity => unimplemented!(),
            _ if outcome >= max => Err(MathError::IntegerOverflow),
            _ if outcome <= min => Err(MathError::IntegerUnderflow),
            _ if outcome == nan => unimplemented!(),
            other => Ok(other),
        }
    }

    fn checked_div(&self, rhs: Self) -> Result<Self, MathError>
    {
        let nan = Self::nan();
        let infinity = Self::infinity();
        let neg_infity = Self::neg_infinity();
        let max = Self::max_value();
        let min = Self::min_value();

        let outcome = self.div(rhs);
        match outcome {
            _ if outcome >= infinity => Err(MathError::DivisionBy0),
            _ if outcome <= neg_infity => unimplemented!(),
            _ if outcome >= max => Err(MathError::IntegerOverflow),
            _ if outcome <= min => Err(MathError::IntegerUnderflow),
            _ if outcome == nan => unimplemented!(),
            other => Ok(other),
        }
    }

    fn checked_rem(&self, rhs: Self) -> Result<Self, MathError>
    {
        let nan = Self::nan();
        let infinity = Self::infinity();
        let neg_infity = Self::neg_infinity();
        let max = Self::max_value();
        let min = Self::min_value();

        let outcome = self.rem(rhs);
        match outcome {
            _ if outcome >= infinity => unimplemented!(),
            _ if outcome <= neg_infity => unimplemented!(),
            _ if outcome >= max => Err(MathError::IntegerOverflow),
            _ if outcome <= min => Err(MathError::IntegerUnderflow),
            _ if outcome == nan => unimplemented!(),
            other => Ok(other),
        }
    }

    fn checked_pow(&self, rhs: Self) -> Result<Self, MathError>
    {
        let nan = Self::nan();
        let infinity = Self::infinity();
        let neg_infity = Self::neg_infinity();
        let max = Self::max_value();
        let min = Self::min_value();

        let outcome = self.powf(rhs);
        match outcome {
            _ if outcome >= infinity => unimplemented!(),
            _ if outcome <= neg_infity => unimplemented!(),
            _ if outcome >= max => Err(MathError::IntegerOverflow),
            _ if outcome <= min => Err(MathError::IntegerUnderflow),
            _ if outcome == nan => unimplemented!(),
            other => Ok(other),
        }
    }
}

impl FloatCheckedOperations for f32 {}
impl FloatCheckedOperations for f64 {}