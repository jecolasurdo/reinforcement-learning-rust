//! Statistics about the relationship between an action and a state.

use crate::stats::ActionStatter;

/// Contains statistics about an action that has been applied to some state.
#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub struct Stats {
    pub(crate) call_count: i32,

    /// This is the raw q-value associated with this action.
    pub(crate) q_raw: f64,

    /// This is the q-value for this action that has been weighted acroding to
    /// the agent's weighting rules.
    pub(crate) q_weighted: f64,
}

impl ActionStatter for Stats {
    /// Returns the number of times this action has been called.
    fn calls(&self) -> i32 {
        self.call_count
    }

    /// Sets the number of times this action has been called.
    fn set_calls(&mut self, n: i32) {
        self.call_count = n
    }

    /// Returns the raw q-value for this action.
    fn q_value_raw(&self) -> f64 {
        self.q_raw
    }

    /// Sets the raw q-value for this action.
    fn set_q_value_raw(&mut self, q: f64) {
        self.q_raw = q
    }

    /// Returns the weighted q-value for this action.
    fn q_value_weighted(&self) -> f64 {
        self.q_weighted
    }

    /// Sets the weighted q-value for this action.
    fn set_q_value_weighted(&mut self, q: f64) {
        self.q_weighted = q
    }
}
