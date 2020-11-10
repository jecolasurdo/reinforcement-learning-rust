pub mod actionstats;
pub mod bayesianagent;
pub mod errors;
pub mod iface;
pub(crate) mod internal;

/// Using manually constructed mocks because (at least at this time), none of
/// the mocking frameworks seem to cope well with generic traits that also have
/// non-static lifetime requirements as well as functions with explicit
/// lifetimes in return values.
#[cfg(test)]
pub(crate) mod mocks;
