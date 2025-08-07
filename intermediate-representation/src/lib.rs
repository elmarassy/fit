use std::f64;

pub mod binary_operation;
pub mod builtin;
pub mod constant;
pub mod expression;
pub mod variable;

#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

pub trait FloatConsts: Sized + Copy {
    const PI: Self;
    const E: Self;
}

impl FloatConsts for f32 {
    const PI: Self = std::f32::consts::PI;
    const E: Self = std::f32::consts::E;
}

impl FloatConsts for f64 {
    const PI: Self = std::f64::consts::PI;
    const E: Self = std::f64::consts::E;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
