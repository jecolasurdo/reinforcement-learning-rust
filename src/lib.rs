pub mod actionstats;
pub mod bayesianagent;
pub mod errors;
pub mod iface;
pub(crate) mod internal;


/// Using manually constructed mocks because (at least at this time), none of
/// the mocking frameworks seem to cope well with generic traits with
/// non-static lifetime requirements and functions with generic lifetime
/// return values.
pub(crate) mod mocks;
