// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

// impl<T> PartialEq for SaturatingU16
// where
//     u16: From<T>,
//     T: Copy,
// {
//     fn eq(&self, other: T) -> bool {
//         self.value == other.into()
//     }
// }

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl Add for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        let val: u16 = rhs.value;
        Self {
            value: self.value.saturating_add(val),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self {
        Self {
            value: self.value + rhs,
        }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: u16::from(value),
        }
    }
}

impl<T> From<&T> for SaturatingU16
where
    u16: From<T>,
    T: Copy,
{
    fn from(value: &T) -> Self {
        SaturatingU16 {
            value: u16::from(*value),
        }
    }
}
