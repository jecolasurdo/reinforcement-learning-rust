use crate::errors::LearnerError;

/// Represents the current state of the model.
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

/// Represents an action that can be applied to the model's current state.
pub trait Actioner<'a> {
    /// Returns a string representation of the given action.
    /// Implementors shoud take care to ensure this is a consistent hash for a
    /// given state.
    fn id(&self) -> &'a str;
}

/// Represents something that is capabile of recommending actions, applying
/// actions to a given state, and learning based on the transition from one
/// state to another.
pub trait Agenter<'a, S, A>
where
    S: Stater<'a, A>,
    A: Actioner<'a>,
{
    /// Recommends an action given a state and the model that the agent has
    /// learned thus far.
    fn recommend_action(&mut self, stater: &'a S) -> Result<&'a A, LearnerError>;

    /// Applies an action to a given state.
    /// Implementors should take care to ensure that this method returns an
    /// error if the supplied action is not applicable to the specified state.
    fn transition(&self, stater: &'a S, actioner: &'a A) -> Result<(), LearnerError>;

    // Updates the model for a given state and action using the provided reward.
    fn learn(
        &mut self,
        previous_state: Option<&'a S>,
        action_taken: &'a A,
        current_state: &'a S,
        reward: f64,
    );
}

/// Represents the stats that can be associated with an action.
pub trait ActionStatter: Clone + Default {
    fn calls(&self) -> i64;
    fn set_calls(&mut self, n: i64);
    fn q_value_raw(&self) -> f64;
    fn set_q_value_raw(&mut self, q: f64);
    fn q_value_weighted(&self) -> f64;
    fn set_q_value_weighted(&mut self, q: f64);
}
