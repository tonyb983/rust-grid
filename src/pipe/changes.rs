use crate::util::TriState;

/// Contains a single change to a [`MapGrid`][`crate::data::MapGrid`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridChange {
    pub row: usize,
    pub col: usize,
    pub prev_value: TriState,
    pub new_value: TriState,
}

/// A list of [`GridChange`]s.
#[derive(Debug, Default)]
pub struct Changelist(Vec<GridChange>);

impl Changelist {
    /// Create a new empty [`Changelist`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_changes(changes: impl Iterator<Item = GridChange>) -> Self {
        let mut new = Self::default();
        for change in changes {
            new.add_change(change);
        }

        new
    }

    /// Add a [`GridChange`] to the [`Changelist`].
    pub fn add_change<C: Into<GridChange>>(&mut self, input: C) {
        self.0.push(input.into());
    }

    /// Create a new [`GridChange`] and add it to the list.
    pub fn add_change_from(
        &mut self,
        row: usize,
        col: usize,
        prev_value: TriState,
        new_value: TriState,
    ) {
        self.0.push(GridChange {
            row,
            col,
            prev_value,
            new_value,
        });
    }

    /// Get the list of changes.
    pub fn data(&self) -> &Vec<GridChange> {
        &self.0
    }
}

impl<T> From<T> for Changelist
where
    T: Iterator<Item = GridChange>,
{
    fn from(input: T) -> Self {
        let inner = input.collect();
        Changelist(inner)
    }
}
