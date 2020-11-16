//! States represent the disposition of a model at some point.

use crate::actions::Actioner;
use crate::errors::LearnerError;

/// Represents the current disposition of the model.
pub trait Stater<'a, A>
where
    A: Actioner<'a>,
{
    /// Provides a slice of Actions that are applicable to this state.
    fn possible_actions(&self) -> Vec<&'a A>;

    /// Checks whether or not the supplied action is compatible with this state.
    fn action_is_compatible(&self, actioner: &'a A) -> bool;

    /// Returns an action of the specified name, or an error if no action exists
    /// of that name fot this state.
    fn get_action(&self, action_name: &str) -> Result<&'a A, LearnerError>;

    /// Returns a string representation of this state.
    /// Implementors should take care to ensure this is a consistent hash for a
    /// given state.
    fn id(&self) -> &str;

    /// Executes the supplied action.
    fn apply(&self, actioner: &'a A) -> Result<(), LearnerError>;
}
