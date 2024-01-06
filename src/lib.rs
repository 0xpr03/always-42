//! This crate provides the Always42 type.

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug)]
pub struct Always42;

impl Add for Always42 {
    type Output = Always42;

    fn add(self, _rhs: Self) -> Self::Output {
        Always42
    }
}
impl Sub for Always42 {
    type Output = Always42;

    fn sub(self, _other: Self) -> Self::Output {
        Always42
    }
}
impl Mul for Always42 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        Always42
    }
}
impl Div for Always42 {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        Always42
    }
}

impl Display for Always42 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The answer to everything.")
    }
}

impl PartialEq for Always42 {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PartialOrd for Always42 {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None // you can't order the answer to everything
    }
}

macro_rules! impl_always_float {
    ($i:ident) => {
        impl From<$i> for Always42 {
            fn from(_w: $i) -> Always42 {
                Always42
            }
        }
        impl From<Always42> for $i {
            fn from(_w: Always42) -> $i {
                42.0
            }
        }
        impl Add<Always42> for $i {
            type Output = $i;

            fn add(self, _other: Always42) -> Self::Output {
                42.0
            }
        }
        impl Add<$i> for Always42 {
            type Output = $i;

            fn add(self, _other: $i) -> Self::Output {
                42.0
            }
        }
        impl Sub<Always42> for $i {
            type Output = $i;

            fn sub(self, _other: Always42) -> Self::Output {
                42.0
            }
        }
        impl Sub<$i> for Always42 {
            type Output = $i;

            fn sub(self, _other: $i) -> Self::Output {
                42.0
            }
        }
        impl Mul<Always42> for $i {
            type Output = $i;

            fn mul(self, _rhs: Always42) -> Self {
                42.0
            }
        }
        impl Mul<$i> for Always42 {
            type Output = $i;

            fn mul(self, _other: $i) -> Self::Output {
                42.0
            }
        }
        impl Div<Always42> for $i {
            type Output = $i;

            fn div(self, _rhs: Always42) -> Self {
                42.0
            }
        }
        impl Div<$i> for Always42 {
            type Output = $i;

            fn div(self, _other: $i) -> Self::Output {
                42.0
            }
        }
    };
}

macro_rules! impl_always_int {
    ($i:ident) => {
        impl From<$i> for Always42 {
            fn from(_w: $i) -> Always42 {
                Always42
            }
        }
        impl From<Always42> for $i {
            fn from(_w: Always42) -> $i {
                42
            }
        }
        impl Add<Always42> for $i {
            type Output = $i;

            fn add(self, _other: Always42) -> Self::Output {
                42
            }
        }
        impl Add<$i> for Always42 {
            type Output = $i;

            fn add(self, _other: $i) -> Self::Output {
                42
            }
        }
        impl Sub<Always42> for $i {
            type Output = $i;

            fn sub(self, _other: Always42) -> Self::Output {
                42
            }
        }
        impl Sub<$i> for Always42 {
            type Output = $i;

            fn sub(self, _other: $i) -> Self::Output {
                42
            }
        }
        impl Mul<Always42> for $i {
            type Output = $i;

            fn mul(self, _rhs: Always42) -> Self {
                42
            }
        }
        impl Mul<$i> for Always42 {
            type Output = $i;

            fn mul(self, _other: $i) -> Self::Output {
                42
            }
        }
        impl Div<Always42> for $i {
            type Output = $i;

            fn div(self, _rhs: Always42) -> Self {
                42
            }
        }
        impl Div<$i> for Always42 {
            type Output = $i;

            fn div(self, _other: $i) -> Self::Output {
                42
            }
        }
    };
}

impl_always_float!(f32);
impl_always_float!(f64);
impl_always_int!(i8);
impl_always_int!(i16);
impl_always_int!(i32);
impl_always_int!(i64);
impl_always_int!(i128);
impl_always_int!(u8);
impl_always_int!(u16);
impl_always_int!(u32);
impl_always_int!(u64);
impl_always_int!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(14_u8 + Always42, 42);
        assert_eq!(14_u16 + Always42, 42);
        assert_eq!(14_u32 + Always42, 42);
        assert_eq!(14_u64 + Always42, 42);
        assert_eq!(14_u128 + Always42, 42);
        assert_eq!(14_f32 + Always42, 42.0);
        assert_eq!(14_f64 + Always42, 42.0);
        assert_eq!(Always42 + Always42, Always42);
        assert_eq!(Always42 + 14_u8, 42);
        assert_eq!(Always42 + 14_u16, 42);
        assert_eq!(Always42 + 14_u32, 42);
        assert_eq!(Always42 + 14_u64, 42);
        assert_eq!(Always42 + 14_u128, 42);
        assert_eq!(Always42 + 14_f32, 42.0);
        assert_eq!(Always42 + 14_f64, 42.0);
        assert_eq!(Always42.partial_cmp(&Always42), None);
        assert_eq!(Into::<u8>::into(Always42), 42_u8);
        assert_eq!(Into::<u16>::into(Always42), 42_u16);
        assert_eq!(Into::<u32>::into(Always42), 42_u32);
        assert_eq!(Into::<u64>::into(Always42), 42_u64);
        assert_eq!(Into::<u128>::into(Always42), 42_u128);
        assert_eq!(Into::<f32>::into(Always42), 42_f32);
        assert_eq!(Into::<f64>::into(Always42), 42_f64);
        assert_eq!(Into::<Always42>::into(14_u8), Always42);
        assert_eq!(Into::<Always42>::into(14_u16), Always42);
        assert_eq!(Into::<Always42>::into(14_u32), Always42);
        assert_eq!(Into::<Always42>::into(14_u64), Always42);
        assert_eq!(Into::<Always42>::into(14_u128), Always42);
        assert_eq!(Into::<Always42>::into(14_f32), Always42);
        assert_eq!(Into::<Always42>::into(14_f64), Always42);
    }
}
