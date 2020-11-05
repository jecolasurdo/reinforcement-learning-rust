use crate::errors::LearnerError;
use crate::iface::{Actioner, Agenter, Stater};

pub struct BayesianAgent {}

impl<S, A: 'static> Agenter<S, A> for BayesianAgent
where
    S: Stater<A>,
    A: Actioner,
{
    fn recommend_action(&self, stater: S) -> Result<A, LearnerError> {
        unimplemented!();
    }

    fn transition(&self, stater: S, actioner: A) -> Result<(), LearnerError> {
        unimplemented!();
    }

    fn learn(&mut self, previous_state: S, action_taken: A, current_state: S, reward: f64) {
        unimplemented!();
    }
}
