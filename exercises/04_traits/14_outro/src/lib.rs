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


// attempt 1 - self wrote - passed all tests

// use std::ops::{Add};

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct SaturatingU16 ( u16 );

// impl From<u16> for SaturatingU16 {
//     fn from(value: u16) -> Self {
//         Self(value)
//     }
// }
// impl From<u8> for SaturatingU16 {
//     fn from(value: u8) -> Self {
//         Self(value as u16)
//     }
// }
// impl From<&u16> for SaturatingU16 {
//     fn from(value: &u16) -> Self {
//         Self(*value)
//     }
// }
// impl From<&u8> for SaturatingU16 {
//     fn from(value: &u8) -> Self {
//         Self(*value as u16)
//     }
// }
// impl Add for SaturatingU16 {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0.saturating_add(rhs.0))
//     }
// }

// impl Add<&SaturatingU16> for SaturatingU16 {
//     type Output = Self;
    
//     fn add(self, rhs: &SaturatingU16) -> Self::Output {
//         Self(self.0.saturating_add(rhs.0))
//     }

    
// }

// impl Add<u16> for SaturatingU16 {
//     type Output = Self;
    
//     fn add(self, rhs: u16) -> Self::Output {
//         Self(self.0.saturating_add(rhs))
//     }
// }

// impl PartialEq<u16> for SaturatingU16 {
//     fn eq(&self, other: &u16) -> bool {
//         self.0 == *other
//     }
// }


// attempt 2 - more idiomatic, thought through with ai

use std::ops::Add;

// for a u16 only, we can derive Eq as well easily
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaturatingU16(u16);

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        // these .into() are better than as. more idiomatic rust
        Self(value.into())
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self((*value).into())
    }
}

/// Canonical implementation:
/// all actual arithmetic logic lives here.
impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

/// Forward reference RHS into the canonical impl.
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs
    }
}

/// Reuse conversion + canonical impl.
impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        self + Self::from(rhs)
    }
}

/// Same idea for &u16.
impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &u16) -> Self::Output {
        self + Self::from(*rhs)
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

/// Optional ergonomic symmetry:
/// allows `5u16 == saturating`
impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.0
    }
}