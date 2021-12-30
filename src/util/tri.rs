use serde::{Deserialize, Serialize};

/// Specifies a 3 state Boolean value.
///
/// ### Variants
/// - `True` - The `on` or `true` state.
/// - `False` - The `off` or `false` state.
/// - `Invalid` - The `invalid` state.
///
/// ### Traits
/// The following traits are implemented for [`TriState`]:
/// - [`From`]
///     - [`bool`], [`Option<bool>`], [`Result<bool, _>`], [`usize`], [`isize`], [`u64`], [`i64`], [`u32`], [`i32`], [`u16`], [`i16`], [`u8`], [`i8`]
///     - Numerical conversions are done using 0 for `False` and 1 for `True`, and any other value as `Invalid`
/// - [`Into`]
///     - [`bool`], [`Option<bool>`]
/// - [`Default`] (Default value is [`TriState::False`])
/// - [`std::fmt::Display`]
/// - [`std::ops::Not`], [`std::ops::BitAnd`], [`std::ops::BitOr`], [`std::ops::BitXor`]
///     - Bitwise operations are the same as a boolean value would be, except that `Invalid` is given higher priority than `True` (Anything AND `Invalid` is `Invalid`, anything XOR `Invalid` is `Invalid`, etc.).
/// - [`serde::Deserialize`] and [`serde::Serialize`]
/// - Auto-Traits:
///    - [`Debug`], [`PartialEq`], [`Eq`], [`Clone`], [`Copy`], [`Hash`], [`PartialOrd`], [`Ord`]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub enum TriState {
    /// True.
    True = 1,
    /// False.
    False = 0,
    /// Invalid.
    Invalid = -1,
}

impl TriState {
    /// Creates a [`TriState`] with a value of [`True`].
    ///
    /// ## Returns
    /// A newly created [`TriState::True`].
    /// ## Errors
    /// - Function errors if yada-yada-yada
    /// ## Panics
    /// - Function panics if yada-yada-yada
    /// ## Example(s)
    /// This is how to use this function and win at [`Life`] and [`Death`]
    /// ```
    /// # // Hidden line
    /// let foo = 2usize;
    /// assert_eq!(foo, 2usize);
    /// ```
    /// Doc comment links are done as such:
    /// [`Life`]: <https://en.wikipedia.org/wiki/Life_(video_game)>
    /// [`Death`]: traits.Death.html
    #[must_use]
    pub fn on() -> Self {
        TriState::True
    }

    /// Create a [`TriState::False`].
    #[must_use]
    pub fn off() -> Self {
        TriState::False
    }

    /// Create a [`TriState::Invalid`].
    #[must_use]
    pub fn invalid() -> Self {
        TriState::Invalid
    }

    /// Returns the opposite of the current *boolean* value. [`TriState::Invalid`] remains unchanged.
    #[must_use]
    pub fn toggle(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Invalid => Self::Invalid,
        }
    }

    /// Returns true if this [`TriState`] is [`TriState::True`] or [`TriState::False`].
    #[must_use]
    pub fn is_valid(self) -> bool {
        self != TriState::Invalid
    }

    /// Safely creates a bool from a [`TriState`] without panicking, converting `Invalid` to `false`.
    #[must_use]
    pub fn safe_bool(self) -> bool {
        self == TriState::True
    }
}

impl std::ops::Not for TriState {
    type Output = Self;

    fn not(self) -> Self {
        self.toggle()
    }
}

impl std::ops::BitAnd for TriState {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Invalid, _) | (_, Self::Invalid) => Self::Invalid,
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
        }
    }
}

impl std::ops::BitOr for TriState {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Invalid, _) | (_, Self::Invalid) => Self::Invalid,
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
        }
    }
}

impl std::ops::BitXor for TriState {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Invalid, _) | (_, Self::Invalid) => Self::Invalid,
            (Self::True, Self::False) | (Self::False, Self::True) => Self::True,
            (Self::True, Self::True) | (Self::False, Self::False) => Self::False,
        }
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

impl<ErrorType> From<Result<bool, ErrorType>> for TriState {
    /// Converts a result of bool to a [`TriState`], using [`TriState::Invalid`] if the value is `Err`.
    fn from(b: Result<bool, ErrorType>) -> Self {
        match b {
            Ok(true) => TriState::True,
            Ok(false) => TriState::False,
            Err(_) => TriState::Invalid,
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

impl std::fmt::Display for TriState {
    /// Displays a [`TriState`] as either "True", "False", or "Invalid".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    }

    #[test]
    #[should_panic]
    fn tri_state_to_bool_panics() {
        let _ = bool::from(TriState::Invalid);
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
    fn bit_and() {
        use std::ops::BitAnd;

        assert_eq!(TriState::True.bitand(TriState::True), TriState::True);
        assert_eq!(TriState::True.bitand(TriState::False), TriState::False);
        assert_eq!(TriState::True.bitand(TriState::Invalid), TriState::Invalid);

        assert_eq!(TriState::False.bitand(TriState::True), TriState::False);
        assert_eq!(TriState::False.bitand(TriState::False), TriState::False);
        assert_eq!(TriState::False.bitand(TriState::Invalid), TriState::Invalid);

        assert_eq!(TriState::Invalid.bitand(TriState::True), TriState::Invalid);
        assert_eq!(TriState::Invalid.bitand(TriState::False), TriState::Invalid);
        assert_eq!(
            TriState::Invalid.bitand(TriState::Invalid),
            TriState::Invalid
        );

        assert_eq!(TriState::True & TriState::True, TriState::True);
        assert_eq!(TriState::True & TriState::False, TriState::False);
        assert_eq!(TriState::True & TriState::Invalid, TriState::Invalid);

        assert_eq!(TriState::False & TriState::True, TriState::False);
        assert_eq!(TriState::False & TriState::False, TriState::False);
        assert_eq!(TriState::False & TriState::Invalid, TriState::Invalid);

        assert_eq!(TriState::Invalid & TriState::True, TriState::Invalid);
        assert_eq!(TriState::Invalid & TriState::False, TriState::Invalid);
        assert_eq!(TriState::Invalid & TriState::Invalid, TriState::Invalid);
    }

    #[test]
    fn bit_or() {
        use std::ops::BitOr;

        assert_eq!(TriState::True.bitor(TriState::True), TriState::True);
        assert_eq!(TriState::True.bitor(TriState::False), TriState::True);
        assert_eq!(TriState::True.bitor(TriState::Invalid), TriState::Invalid);

        assert_eq!(TriState::False.bitor(TriState::True), TriState::True);
        assert_eq!(TriState::False.bitor(TriState::False), TriState::False);
        assert_eq!(TriState::False.bitor(TriState::Invalid), TriState::Invalid);

        assert_eq!(TriState::Invalid.bitor(TriState::True), TriState::Invalid);
        assert_eq!(TriState::Invalid.bitor(TriState::False), TriState::Invalid);
        assert_eq!(
            TriState::Invalid.bitor(TriState::Invalid),
            TriState::Invalid
        );

        assert_eq!(TriState::True | TriState::True, TriState::True);
        assert_eq!(TriState::True | TriState::False, TriState::True);
        assert_eq!(TriState::True | TriState::Invalid, TriState::Invalid);

        assert_eq!(TriState::False | TriState::True, TriState::True);
        assert_eq!(TriState::False | TriState::False, TriState::False);
        assert_eq!(TriState::False | TriState::Invalid, TriState::Invalid);

        assert_eq!(TriState::Invalid | TriState::True, TriState::Invalid);
        assert_eq!(TriState::Invalid | TriState::False, TriState::Invalid);
        assert_eq!(TriState::Invalid | TriState::Invalid, TriState::Invalid);
    }

    #[test]
    fn bit_xor() {
        use std::ops::BitXor;

        assert_eq!(TriState::True.bitxor(TriState::True), TriState::False);
        assert_eq!(TriState::True.bitxor(TriState::False), TriState::True);
        assert_eq!(TriState::True.bitxor(TriState::Invalid), TriState::Invalid);

        assert_eq!(TriState::False.bitxor(TriState::True), TriState::True);
        assert_eq!(TriState::False.bitxor(TriState::False), TriState::False);
        assert_eq!(TriState::False.bitxor(TriState::Invalid), TriState::Invalid);

        assert_eq!(TriState::Invalid.bitxor(TriState::True), TriState::Invalid);
        assert_eq!(TriState::Invalid.bitxor(TriState::False), TriState::Invalid);
        assert_eq!(
            TriState::Invalid.bitxor(TriState::Invalid),
            TriState::Invalid
        );

        assert_eq!(TriState::True ^ TriState::True, TriState::False);
        assert_eq!(TriState::True ^ TriState::False, TriState::True);
        assert_eq!(TriState::True ^ TriState::Invalid, TriState::Invalid);

        assert_eq!(TriState::False ^ TriState::True, TriState::True);
        assert_eq!(TriState::False ^ TriState::False, TriState::False);
        assert_eq!(TriState::False ^ TriState::Invalid, TriState::Invalid);

        assert_eq!(TriState::Invalid ^ TriState::True, TriState::Invalid);
        assert_eq!(TriState::Invalid ^ TriState::False, TriState::Invalid);
        assert_eq!(TriState::Invalid ^ TriState::Invalid, TriState::Invalid);
    }

    #[test]
    fn not_impl() {
        use std::ops::Not;

        assert_eq!(TriState::True.not(), TriState::False);
        assert_eq!(TriState::False.not(), TriState::True);
        assert_eq!(TriState::Invalid.not(), TriState::Invalid);

        assert_eq!(!TriState::True, TriState::False);
        assert_eq!(!TriState::False, TriState::True);
        assert_eq!(!TriState::Invalid, TriState::Invalid);
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
