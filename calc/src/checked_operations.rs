use num_traits::Float;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum MathError {
    DivisionBy0,
    Overflow,
    Underflow,
    //NegativeSquareRoot,
    Infinity,
    NegativeInfinity,
    Nan,
}

pub trait CheckedFloatOperations where 
Self: Sized + Float, {
    
    fn checked_add(&self, rhs: Self) -> Result<Self, MathError>
    {
        let outcome = self.add(rhs);
        match outcome {
            _ if outcome.is_nan() => Err(MathError::Nan),
            _ if outcome == Self::infinity() => Err(MathError::Infinity),
            _ if outcome == Self::neg_infinity() => Err(MathError::NegativeInfinity),
            _ if outcome == Self::max_value() => Err(MathError::Overflow),
            _ if outcome == Self::min_value() => Err(MathError::Underflow),
            other => Ok(other),
        }
    }

    fn checked_sub(&self, rhs: Self) -> Result<Self, MathError>
    {
        let outcome = self.sub(rhs);
        match outcome {
            _ if outcome.is_nan() => Err(MathError::Nan),
            _ if outcome == Self::infinity() => Err(MathError::Infinity),
            _ if outcome == Self::neg_infinity() => Err(MathError::NegativeInfinity),
            _ if outcome == Self::max_value() => Err(MathError::Overflow),
            _ if outcome == Self::min_value() => Err(MathError::Underflow),
            other => Ok(other),
        }
    }

    fn checked_mul(&self, rhs: Self) -> Result<Self, MathError>
    {
        let outcome = self.mul(rhs);
        match outcome {
            _ if outcome.is_nan() => Err(MathError::Nan),
            _ if outcome == Self::infinity() => Err(MathError::Infinity),
            _ if outcome == Self::neg_infinity() => Err(MathError::NegativeInfinity),
            _ if outcome == Self::max_value() => Err(MathError::Overflow),
            _ if outcome == Self::min_value() => Err(MathError::Underflow),
            other => Ok(other),
        }
    }

    fn checked_div(&self, rhs: Self) -> Result<Self, MathError>
    {
        if rhs.is_zero() {
            return Err(MathError::DivisionBy0);
        }

        let outcome = self.div(rhs);
        match outcome {
            _ if outcome.is_nan() => Err(MathError::Nan),
            _ if outcome == Self::infinity() => Err(MathError::Infinity),
            _ if outcome == Self::neg_infinity() => Err(MathError::NegativeInfinity),
            _ if outcome == Self::max_value() => Err(MathError::Overflow),
            _ if outcome == Self::min_value() => Err(MathError::Underflow),
            other => Ok(other),
        }
    }

    fn checked_rem(&self, rhs: Self) -> Result<Self, MathError>
    {
        let outcome = self.rem(rhs);
        match outcome {
            _ if outcome.is_nan() => Err(MathError::Nan),
            _ if outcome == Self::infinity() => Err(MathError::Infinity),
            _ if outcome == Self::neg_infinity() => Err(MathError::NegativeInfinity),
            _ if outcome >= Self::max_value() => unreachable!(),
            _ if outcome <= Self::min_value() => unreachable!(),
            other => Ok(other),
        }
    }

    fn checked_pow(&self, rhs: Self) -> Result<Self, MathError>
    {
        let outcome = self.powf(rhs);
        match outcome {
            _ if outcome.is_nan() => Err(MathError::Nan),
            _ if outcome == Self::infinity() => Err(MathError::Infinity),
            _ if outcome == Self::neg_infinity() => Err(MathError::NegativeInfinity),
            _ if outcome == Self::max_value() => Err(MathError::Overflow),
            _ if outcome == Self::min_value() => Err(MathError::Underflow),
            other => Ok(other),
        }
    }
}

impl CheckedFloatOperations for f32 {}
impl CheckedFloatOperations for f64 {}


#[cfg(test)]
mod checked_float_operations_tests {
    use super::*;


    #[test]
    fn add() {
        let one: f32 = 1.0;
        assert_eq!(f32::NAN.checked_add(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::INFINITY.checked_add(one).unwrap_err(), MathError::Infinity);
        assert_eq!(f32::NEG_INFINITY.checked_add(one).unwrap_err(), MathError::NegativeInfinity);
        assert_eq!(f32::MAX.checked_add(one).unwrap_err(), MathError::Overflow);
        assert_eq!(f32::MIN.checked_add(-one).unwrap_err(), MathError::Underflow);
        assert_eq!(one.checked_add(one).unwrap(), 2.0);
    }

    #[test]
    fn sub() {
        let one: f32 = 1.0;
        assert_eq!(f32::NAN.checked_sub(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::INFINITY.checked_sub(one).unwrap_err(), MathError::Infinity);
        assert_eq!(f32::NEG_INFINITY.checked_sub(one).unwrap_err(), MathError::NegativeInfinity);
        assert_eq!(f32::MAX.checked_sub(-one).unwrap_err(), MathError::Overflow);
        assert_eq!(f32::MIN.checked_sub(one).unwrap_err(), MathError::Underflow);
        assert_eq!(one.checked_sub(one).unwrap(), 0.0);
    }

    #[test]
    fn mul() {
        let one: f32 = 1.0;
        assert_eq!(f32::NAN.checked_mul(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::INFINITY.checked_mul(one).unwrap_err(), MathError::Infinity);
        assert_eq!(f32::NEG_INFINITY.checked_mul(one).unwrap_err(), MathError::NegativeInfinity);
        assert_eq!(f32::MAX.checked_mul(one).unwrap_err(), MathError::Overflow);
        assert_eq!(f32::MIN.checked_mul(one).unwrap_err(), MathError::Underflow);
        assert_eq!(one.checked_mul(one).unwrap(), 1.0);
    }

    #[test]
    fn div() {
        let one: f32 = 1.0;
        let zero: f32 = 0.0;
        assert_eq!(f32::NAN.checked_div(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::INFINITY.checked_div(one).unwrap_err(), MathError::Infinity);
        assert_eq!(f32::NEG_INFINITY.checked_div(one).unwrap_err(), MathError::NegativeInfinity);
        assert_eq!(f32::MAX.checked_div(one).unwrap_err(), MathError::Overflow);
        assert_eq!(f32::MIN.checked_div(one).unwrap_err(), MathError::Underflow);
        assert_eq!(f32::MIN.checked_div(zero).unwrap_err(), MathError::DivisionBy0);
        assert_eq!(f32::MIN.checked_div(-zero).unwrap_err(), MathError::DivisionBy0);
        assert_eq!(one.checked_div(one).unwrap(), 1.0);
    }

    #[test]
    fn rem() {
        let one: f32 = 1.0;
        assert_eq!(f32::NAN.checked_rem(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::INFINITY.checked_rem(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::NEG_INFINITY.checked_rem(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::MAX.checked_rem(one).unwrap(), 0.0);
        assert_eq!(f32::MIN.checked_rem(one).unwrap(), -0.0);
        assert_eq!(one.checked_div(one).unwrap(), 1.0);
    }

    #[test]
    fn pow() {
        let one: f32 = 1.0;
        assert_eq!(f32::NAN.checked_pow(one).unwrap_err(), MathError::Nan);
        assert_eq!(f32::INFINITY.checked_pow(one).unwrap_err(), MathError::Infinity);
        assert_eq!(f32::NEG_INFINITY.checked_pow(one).unwrap_err(), MathError::NegativeInfinity);
        assert_eq!(f32::MAX.checked_pow(one).unwrap_err(), MathError::Overflow);
        assert_eq!(f32::MIN.checked_pow(one).unwrap_err(), MathError::Underflow);
        assert_eq!(one.checked_pow(one).unwrap(), 1.0);
    }

}