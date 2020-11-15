pub mod actionstats;

/// Represents the stats that can be associated with an action.
pub trait ActionStatter: Clone + Default {
    fn calls(&self) -> i32;
    fn set_calls(&mut self, n: i32);
    fn q_value_raw(&self) -> f64;
    fn set_q_value_raw(&mut self, q: f64);
    fn q_value_weighted(&self) -> f64;
    fn set_q_value_weighted(&mut self, q: f64);
}
