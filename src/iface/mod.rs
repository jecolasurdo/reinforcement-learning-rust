use std::error::Error;

/// Represents the current state of the model.
pub trait Stater {
    /// Provides a slice of Actions that are applicable to this state.
    fn possible_actions(self) -> Vec<Box<dyn Actioner>>;

    /// Checks whether or not the supplied action is compatible with this state.
    fn action_is_compatible(self, actioner: dyn Actioner) -> bool;

    /// Returns an action of the specified name, or an error if no action exists
    /// of that name fot this state.
    fn get_action(self, action_name: String) -> Result<Box<dyn Actioner>, Box<dyn Error>>;

    /// Returns a string representation of this state.
    /// Implementors should take care to ensure this is a consistent hash for a
    /// given state.
    fn id(self) -> String;

    /// Executes the supplied action.
    fn apply(self, actioner: dyn Actioner) -> Result<(), Box<dyn Error>>;
}

/// Represents an action that can be applied to the model's current state.
pub trait Actioner {
    /// Returns a string representation of the given action.
    /// Implementors shoud take care to ensure this is a consistent hash for a
    /// given state.
    fn id(self) -> String;
}

/// Represents something that is capabile of recommending actions, applying
/// actions to a given state, and learning based on the transition from one
/// state to another.
pub trait Agenter {
    /// Recommends an action given a state and the model that the agent has
    /// learned thus far.
    fn recommend_action(self, stater: dyn Stater) -> Result<Box<dyn Actioner>, Box<dyn Error>>;

    /// Applies an action to a given state.
    /// Implementors should take care to ensure that this method returns an
    /// error if the supplied action is not applicable to the specified state.
    fn transition(self, stater: dyn Stater, actioner: dyn Actioner) -> Result<(), Box<dyn Error>>;

    // Updates the model for a given state and action using the provided reward.
    fn learn(
        self,
        previous_state: dyn Stater,
        action_taken: dyn Actioner,
        current_state: dyn Stater,
        reward: f64,
    );
}

/// Represents the stats that can be associated with an action.
pub trait ActionStatter {
    fn calls(self) -> i64;
    fn set_calls(self, n: i64);
    fn q_value_raw(self) -> f64;
    fn set_q_value_raw(self, q: f64);
    fn q_value_weighted(self) -> f64;
    fn set_q_value_weighted(self, q: f64);
}
