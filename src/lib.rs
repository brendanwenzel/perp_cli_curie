#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// Common Utilities
pub mod utils;

/// Contracts, Pools and Tokens
pub mod addresses;

/// Contract Instances
pub mod contracts;

/// ABIs
pub mod abis;

/// Address List
pub mod address_list;

/// Portfolio
pub mod portfolio;

/// Arguments
pub mod args;

/// AMM Command
pub mod amm;

/// Position Command
pub mod position;

/// Re-export a prelude
pub mod prelude {
    pub use super::{abis::*, amm::*, args::*, address_list::*, position::*, portfolio::*, address_list::*, contracts::*, utils::*};
}