use crate::Builder;
use mev_builders_macros::include_builders;

/// List of known builders with their details, ordered by landed blocks.
pub static BUILDERS: &[Builder] = include_builders!();
