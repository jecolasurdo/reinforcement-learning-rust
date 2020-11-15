#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    // clippy::nursery,
    // clippy::cargo
)]

pub mod actions;
pub mod agents;
pub mod errors;
pub(crate) mod internal;
pub mod states;
pub mod stats;

/// Using manually constructed mocks because (at least at this time), none of
/// the mocking frameworks seem to cope well with generic traits that also have
/// non-static lifetime requirements as well as functions with explicit
/// lifetimes in return values.
#[cfg(test)]
pub(crate) mod mocks;
