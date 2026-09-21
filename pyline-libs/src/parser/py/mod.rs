//! Python parser implementation.
//!
//! Split into:
//! - `base` — shared constants and default values;
//! - `engine` — language-agnostic parsing core (crate-private);

pub(crate) mod base;
pub(crate) mod engine;
