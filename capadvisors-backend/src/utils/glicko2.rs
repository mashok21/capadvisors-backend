// Exposes the spec's required function signature without duplicating the
// core algorithm, which lives in rating.rs alongside its test suite.
pub use crate::utils::rating::compute_glicko2_update;
