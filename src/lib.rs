#![warn(
    missing_docs,
    missing_doc_code_examples,
    broken_intra_doc_links,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::as_conversions,
    clippy::todo,
    clippy::print_stdout,
    clippy::use_debug
)]
#![allow(
    clippy::must_use_candidate,
    clippy::float_cmp,
    clippy::similar_names,
    clippy::missing_errors_doc,
    clippy::missing_const_for_fn
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
