//! Actions represent steps that can be taken to transition a model from one
//! state to another.

/// Represents an action that can be applied to the model's current state.
pub trait Actioner<'a> {
    /// Returns a string representation of the given action.
    /// Implementors shoud take care to ensure this is a consistent hash for a
    /// given state.
    fn id(&self) -> &'a str;
}
