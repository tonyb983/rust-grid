//! # `tri-state`: fearless booleans
//!
//! Gone are the days where a simple true/false boolean variable suffices. Modern software requires
//! modern solutions: `TriState`.
//!
//! ***Definitely*** Trusted by Microsoft.
//!
//! ## Old, slow, ancient, unsafe code
//! ```
//! let foo = true;
//! if foo {
//!     println!("Hello, world!");
//! }
//!
//! // Hard to read, intent unclear
//! let bar = 1 == 2;
//! match bar {
//!     false => println!("One does not equal two"),
//!     true => println!("One equals two"),
//!     // Restrictive, not web-scale
//! }
//! ```
//!
//! ## New, fast, web-scale, safe code
//! ```
//! // Clean and easy to read
//! let foo = TriState::True;
//! if foo.into() {
//!     println!("Hello, world!");
//! }
//!
//! // Simple, effortless conversion
//! let bar: TriState = (1 == 2).into();
//! match bar {
//!     TriState::False => println!("One does not equal two"),
//!     TriState::True => println!("One equals two"),
//!     // Highly future-proof and scalable
//!     _ => panic!(),
//! }
//!
//! // Compatible with all major brands
//! let has_a_3 = TriState::from(vec![1, 2, 4, 5].contains(&3));
//! println!("Has a 3: {}", has_a_3); // prints "Has a 3: False"
//! ```
#![deny(missing_docs)]

use std::fmt;

use serde::{Deserialize, Serialize};

/// Specifies a tri-state Boolean value.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
pub enum TriState {
    /// True.
    True = 1,
    /// False.
    False = 0,
    /// Invalid.
    Invalid = -1,
}

impl TriState {
    /// Returns the opposite of the current *boolean* value. [`TriState::Invalid`] remains unchanged.
    pub fn not(self) -> TriState {
        match self {
            TriState::True => TriState::False,
            TriState::False => TriState::True,
            TriState::Invalid => TriState::Invalid,
        }
    }

    /// Returns true if this [`TriState`] is [`TriState::True`] or [`TriState::False`].
    pub fn is_valid(self) -> bool {
        self != TriState::Invalid
    }

    /// Safely creates a bool from a [`TriState`] without panicking, converting `Invalid` to `false`.
    pub fn safe_bool(self) -> bool {
        self == TriState::True
    }
}

impl Default for TriState {
    /// Default for [`TriState`] is [`TriState::False`], **not** [`TriState::Invalid`].
    fn default() -> Self {
        TriState::False
    }
}

impl From<bool> for TriState {
    /// Converts a bool to a [`TriState`].
    fn from(b: bool) -> Self {
        if b {
            TriState::True
        } else {
            TriState::False
        }
    }
}

impl From<Option<bool>> for TriState {
    /// Converts an optional bool to a [`TriState`], using [`TriState::Invalid`] if the value is `None`.
    fn from(b: Option<bool>) -> Self {
        match b {
            Some(true) => TriState::True,
            Some(false) => TriState::False,
            None => TriState::Invalid,
        }
    }
}

impl From<TriState> for bool {
    /// Converts a [`TriState`] to a bool.
    ///
    /// ## ***Warning*** this function panics if the [`TriState`] is [`TriState::Invalid`].
    ///
    /// Use [`TriState::safe_bool`] for a lossy, non-panicking conversion.
    fn from(t: TriState) -> bool {
        match t {
            TriState::False => false,
            TriState::True => true,
            TriState::Invalid => panic!("Invalid state for boolean."),
        }
    }
}

impl From<TriState> for Option<bool> {
    /// Converts a [`TriState`] to an optional bool, returning `None` for [`TriState::Invalid`].
    fn from(t: TriState) -> Option<bool> {
        match t {
            TriState::False => Some(false),
            TriState::True => Some(true),
            TriState::Invalid => None,
        }
    }
}

impl From<usize> for TriState {
    /// Converts a `usize` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: usize) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<isize> for TriState {
    /// Converts a `isize` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: isize) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<i64> for TriState {
    /// Converts a `i64` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: i64) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<u64> for TriState {
    /// Converts a `u64` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: u64) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<i32> for TriState {
    /// Converts a `i32` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: i32) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<u32> for TriState {
    /// Converts a `u32` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: u32) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<i16> for TriState {
    /// Converts a `i16` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: i16) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<u16> for TriState {
    /// Converts a `u16` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: u16) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<i8> for TriState {
    /// Converts a `i8` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: i8) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl From<u8> for TriState {
    /// Converts a `u8` to a [`TriState`]. 1 is [`TriState::True`], 0 is [`TriState::False`], and any other value is considered [`TriState::Invalid`].
    fn from(i: u8) -> Self {
        match i {
            1 => TriState::True,
            0 => TriState::False,
            _ => TriState::Invalid,
        }
    }
}

impl fmt::Display for TriState {
    /// Displays a [`TriState`] as either "True", "False", or "Invalid".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TriState::True => "True",
                TriState::False => "False",
                TriState::Invalid => "Invalid",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_to_tri_state() {
        assert_eq!(TriState::from(false), TriState::False);
        assert_eq!(TriState::from(true), TriState::True);
    }

    #[test]
    fn tri_state_to_bool() {
        assert!(!bool::from(TriState::False));
        assert!(bool::from(TriState::True));

        std::panic::catch_unwind(|| bool::from(TriState::Invalid)).unwrap_err();
    }

    #[test]
    fn tri_state_into_bool_option() {
        assert_eq!(Option::<bool>::from(TriState::False), Some(false));
        assert_eq!(Option::<bool>::from(TriState::True), Some(true));
        assert_eq!(Option::<bool>::from(TriState::Invalid), None);
    }

    #[test]
    fn tri_state_into_conversions() {
        let foo = TriState::True;
        let foo_bool: bool = foo.into();
        assert!(foo_bool);

        let bar: TriState = (1 == 2).into();
        let bar_bool: Option<bool> = bar.into();
        assert_eq!(bar_bool, Some(false));

        let has_a_3 = TriState::from(vec![1, 2, 4, 5].contains(&3));
        assert_eq!(format!("Has a 3: {}", has_a_3), "Has a 3: False");
        assert_eq!(Option::<bool>::from(TriState::False), Some(false));
    }

    #[test]
    fn numeric_into_conversions() {
        let ts: TriState = 0usize.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1usize.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3usize.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0isize.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1isize.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3isize.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0i64.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1i64.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3i64.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0u64.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1u64.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3u64.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0i32.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1i32.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3i32.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0u32.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1u32.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3u32.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0i16.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1i16.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3i16.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0u16.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1u16.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3u16.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0i8.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1i8.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3i8.into();
        assert_eq!(ts, TriState::Invalid);

        let ts: TriState = 0u8.into();
        assert_eq!(ts, TriState::False);
        let ts: TriState = 1u8.into();
        assert_eq!(ts, TriState::True);
        let ts: TriState = 3u8.into();
        assert_eq!(ts, TriState::Invalid);
    }

    #[test]
    fn display() {
        assert_eq!(TriState::True.to_string(), "True");
        assert_eq!(TriState::False.to_string(), "False");
        assert_eq!(TriState::Invalid.to_string(), "Invalid");
    }
}
