//! Statistics that aid in the learning process.

pub mod actionstats;

/// Represents the stats that can be associated with an action.
pub trait ActionStatter: Clone + Default {
    /// The number of times this action has been executed.
    fn calls(&self) -> i32;

    /// Set the number of times this action has been executed.
    fn set_calls(&mut self, n: i32);

    /// The raw Q value for this action.
    fn q_value_raw(&self) -> f64;

    /// Set the raw Q value for this action.
    fn set_q_value_raw(&mut self, q: f64);

    /// The weighted Q value for this action.
    fn q_value_weighted(&self) -> f64;

    /// Set the weighted Q value for this action.
    fn set_q_value_weighted(&mut self, q: f64);
}
