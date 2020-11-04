use mockall::automock;

#[derive(Debug, Copy, Clone)]
pub struct LearnerError;

/// Represents the current state of the model.
#[automock]
pub trait Stater<A: 'static>
where
    A: Actioner,
{
    /// Provides a slice of Actions that are applicable to this state.
    fn possible_actions(&mut self) -> Vec<A>;

    /// Checks whether or not the supplied action is compatible with this state.
    fn action_is_compatible(&mut self, actioner: A) -> bool;

    /// Returns an action of the specified name, or an error if no action exists
    /// of that name fot this state.
    fn get_action(&mut self, action_name: String) -> Result<A, LearnerError>;

    /// Returns a string representation of this state.
    /// Implementors should take care to ensure this is a consistent hash for a
    /// given state.
    fn id(&mut self) -> String;

    /// Executes the supplied action.
    fn apply(&mut self, actioner: A) -> Result<(), LearnerError>;
}

/// Represents an action that can be applied to the model's current state.
#[automock]
pub trait Actioner {
    /// Returns a string representation of the given action.
    /// Implementors shoud take care to ensure this is a consistent hash for a
    /// given state.
    fn id(&mut self) -> String;
}

/// Represents something that is capabile of recommending actions, applying
/// actions to a given state, and learning based on the transition from one
/// state to another.
pub trait Agenter<S, A: 'static>
where
    S: Stater<A>,
    A: Actioner,
{
    /// Recommends an action given a state and the model that the agent has
    /// learned thus far.
    fn recommend_action(&mut self, stater: S) -> Result<A, LearnerError>;

    /// Applies an action to a given state.
    /// Implementors should take care to ensure that this method returns an
    /// error if the supplied action is not applicable to the specified state.
    fn transition(&mut self, stater: S, actioner: A) -> Result<(), LearnerError>;

    // Updates the model for a given state and action using the provided reward.
    fn learn(&mut self, previous_state: S, action_taken: A, current_state: S, reward: f64);
}

/// Represents the stats that can be associated with an action.
#[automock]
pub trait ActionStatter {
    fn calls(&mut self) -> i64;
    fn set_calls(&mut self, n: i64);
    fn q_value_raw(&mut self) -> f64;
    fn set_q_value_raw(&mut self, q: f64);
    fn q_value_weighted(self) -> f64;
    fn set_q_value_weighted(&mut self, q: f64);
}
