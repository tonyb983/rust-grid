use fastrand;
use serde::{Deserialize, Serialize};

use crate::util::tri::TriState;

/// A simple cell that can be either `on` or `off`. Uses a simple bool for internal state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Deserialize, Serialize)]
pub struct BasicCell(bool);

impl BasicCell {
    /// Creates a new [`BasicCell`] with the given value.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Creates a [`BasicCell`] that is `on` or `true`.
    pub fn on() -> Self {
        Self(true)
    }

    /// Creates a [`BasicCell`] that is `off` or `false`.
    pub fn off() -> Self {
        Self(false)
    }
}

impl BasicCell {
    /// Sets the state of this [`BasicCell`] to the given `value`.
    pub fn set_state(&mut self, value: bool) {
        self.0 = value;
    }

    /// Gets the state of this [`BasicCell`].
    pub fn state(self) -> bool {
        self.0
    }

    /// Returns `true` if this [`BasicCell`] is `off` or `false`.
    pub fn is_off(self) -> bool {
        !self.state()
    }

    /// Returns `true` if this [`BasicCell`] is `on` or `true`.
    pub fn is_on(self) -> bool {
        self.state()
    }

    /// Flips the state of this [`BasicCell`], turning True to False and vice versa.
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

impl Default for BasicCell {
    /// Creates a default (*off*) [`BasicCell`].
    fn default() -> Self {
        Self::off()
    }
}

/// A simple cell that can be either `on`, `off`, or `invalid`. Uses [`TriState`] for the internal state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Deserialize, Serialize)]
pub struct TriCell(TriState);

impl TriCell {
    /// Creates a new [`TriCell`] with the given value.
    pub fn new(value: TriState) -> Self {
        Self(value)
    }

    /// Creates a [`TriCell`] that is `on` or `true`.
    pub fn on() -> Self {
        Self(true.into())
    }

    /// Creates a [`TriCell`] that is `off` or `false`.
    pub fn off() -> Self {
        Self(false.into())
    }

    /// Creates a [`TriCell`] that is `invalid`.
    pub fn invalid() -> Self {
        Self(TriState::Invalid)
    }

    /// Creates a [`TriCell`] with a random state.
    pub fn random() -> Self {
        Self(fastrand::bool().into())
    }
}

impl TriCell {
    /// Get the current state of this [`TriCell`].
    pub fn state(self) -> TriState {
        self.0
    }

    /// Sets the current state of this [`TriCell`] to the given `value`.
    pub fn set_state(&mut self, value: TriState) {
        self.0 = value;
    }

    pub fn set_random(&mut self) {
        self.0 = fastrand::bool().into();
    }

    /// Returns true if this [`TriCell`] is `off` or `false`.
    pub fn is_off(self) -> bool {
        self.state() == TriState::False
    }

    /// Returns true if this [`TriCell`] is `on` or `true`.
    pub fn is_on(self) -> bool {
        self.state() == TriState::True
    }

    /// Returns `true` if this [`TriCell`] is `on` or `off`, but not `invalid`.
    pub fn is_valid(self) -> bool {
        self.state() != TriState::Invalid
    }

    /// Returns `true` if this [`TriCell`] is `invalid`.
    pub fn is_invalid(self) -> bool {
        !self.is_valid()
    }

    /// Flips the internal state of this [`TriCell`], turning True to False and vice versa.
    /// 
    /// *Invalid is kept as is.*
    pub fn toggle(&mut self) {
        self.0 = self.0.not();
    }
}

impl Default for TriCell {
    /// Creates a default (***invalid***) [`TriCell`].
    fn default() -> Self {
        Self::invalid()
    }
}

impl From<BasicCell> for TriCell {
    /// Convert a [`BasicCell`] to a [`TriCell`].
    fn from(cell: BasicCell) -> Self {
        if cell.is_on() {
            Self::on()
        } else {
            Self::off()
        }
    }
}

impl From<TriCell> for BasicCell {
    /// Convert a [`TriCell`] to a [`BasicCell`].
    ///
    /// *This conversion is lossy, [`crate::util::tri::TriState::Invalid`] will be converted to false.*
    fn from(cell: TriCell) -> Self {
        if cell.is_on() {
            Self::on()
        } else {
            Self::off()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Deserialize, Serialize)]
pub enum Tile {
    Invalid,
    Floor,
    Wall
}

impl Tile {
    fn random() -> Self {
        if fastrand::bool() {
            Tile::Wall
        } else {
            Tile::Floor
        }
    }

    fn toggle(self) -> Self {
        match self {
            Tile::Invalid => Tile::Invalid,
            Tile::Floor => Tile::Wall,
            Tile::Wall => Tile::Floor
        }
    }
}

impl From<bool> for Tile {
    fn from(b: bool) -> Self {
        if b {
            Tile::Wall
        } else {
            Tile::Floor
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Deserialize, Serialize)]
struct TileCell(Tile);

impl TileCell {
    /// Get the current state of this [`TriCell`].
    pub fn state(self) -> Tile {
        self.0
    }

    /// Sets the current state of this [`TriCell`] to the given `value`.
    pub fn set_state(&mut self, value: Tile) {
        self.0 = value;
    }

    pub fn set_random(&mut self) {
        self.0 = Tile::random();
    }

    /// Returns true if this [`TriCell`] is `off` or `false`.
    pub fn is_off(self) -> bool {
        self.state() == Tile::Floor
    }

    /// Returns true if this [`TriCell`] is `on` or `true`.
    pub fn is_on(self) -> bool {
        self.state() == Tile::Wall
    }

    /// Returns `true` if this [`TriCell`] is `on` or `off`, but not `invalid`.
    pub fn is_valid(self) -> bool {
        self.state() != Tile::Invalid
    }

    /// Returns `true` if this [`TriCell`] is `invalid`.
    pub fn is_invalid(self) -> bool {
        !self.is_valid()
    }

    /// Flips the internal state of this [`TriCell`], turning True to False and vice versa.
    /// 
    /// *Invalid is kept as is.*
    pub fn toggle(&mut self) {
        self.0 = self.0.toggle();
    }
}

pub trait MapBlock {
    type StateType = Tile;

    fn set_state(&mut self, state: Self::StateType);
    fn state(&self) -> Self::StateType;
    fn toggle(&mut self);
    fn is_state(&self, state: Self::StateType) -> bool;
}

impl MapBlock for TriCell {
    type StateType = TriState;

    fn set_state(&mut self, state: TriState) {
        TriCell::set_state(self, state);
    }

    fn state(&self) -> TriState {
        TriCell::state(*self)
    }

    fn toggle(&mut self) {
        TriCell::toggle(self);
    }

    fn is_state(&self, state: TriState) -> bool {
        TriCell::state(*self) == state
    }
}

impl MapBlock for TileCell {
    type StateType = Tile;

    fn set_state(&mut self, state: Tile) {
        TileCell::set_state(self, state);
    }

    fn state(&self) -> Tile {
        TileCell::state(*self)
    }

    fn toggle(&mut self) {
        TileCell::toggle(self);
    }

    fn is_state(&self, state: Tile) -> bool {
        TileCell::state(*self) == state
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn tricell_toggle_works() {
        init();

        let mut cell = TriCell::on();
        assert_eq!(cell, TriCell::on());
        assert_ne!(cell, TriCell::off());
        assert_ne!(cell, TriCell::invalid());

        cell.toggle();
        assert_ne!(cell, TriCell::on());
        assert_eq!(cell, TriCell::off());
        assert_ne!(cell, TriCell::invalid());

        cell.toggle();
        assert_eq!(cell, TriCell::on());
        assert_ne!(cell, TriCell::off());
        assert_ne!(cell, TriCell::invalid());

        cell.toggle();
        assert_ne!(cell, TriCell::on());
        assert_eq!(cell, TriCell::off());
        assert_ne!(cell, TriCell::invalid());

        let mut cell = TriCell::invalid();
        assert_ne!(cell, TriCell::on());
        assert_ne!(cell, TriCell::off());
        assert_eq!(cell, TriCell::invalid());

        cell.toggle();
        assert_ne!(cell, TriCell::on());
        assert_ne!(cell, TriCell::off());
        assert_eq!(cell, TriCell::invalid());
    }
}
