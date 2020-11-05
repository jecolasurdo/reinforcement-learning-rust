use crate::iface::ActionStatter;

/// Contains statistics about an action that has been applied to some state.
pub struct ActionStats {
    call_count: i64,

    /// This is the raw q-value associated with this action.
    q_raw: f64,

    /// This is the q-value for this action that has been weighted acroding to
    /// the agent's weighting rules.
    q_weighted: f64,
}

impl ActionStatter for ActionStats {
    /// Returns the number of times this action has been called.
    fn calls(&self) -> i64 {
        self.call_count
    }

    /// Sets the number of times this action has been called.
    fn set_calls(&mut self, n: i64) {
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
