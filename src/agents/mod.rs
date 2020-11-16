pub mod bayesian;

use crate::actions::Actioner;
use crate::errors::LearnerError;
use crate::states::Stater;

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

    /// Updates the model for a given state and action using the provided reward.
    fn learn(
        &mut self,
        previous_state: Option<&'a S>,
        action_taken: &'a A,
        current_state: &'a S,
        reward: f64,
    );
}
