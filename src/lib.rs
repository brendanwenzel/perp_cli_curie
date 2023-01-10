#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// Common Utilities
pub mod utils;

/// Contract Instances
pub mod contracts;

/// Quit
pub mod quit;

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

/// Tokens Command
pub mod tokens;

/// Deposit Command
pub mod deposit;

/// Withdraw Command
pub mod withdraw;

/// This module processes the "open" sub-command
pub mod open;

/// This module processes the "close" sub-command
pub mod close;

/// Metadata JSON
pub mod metadata;

/// Swap ERC-20 tokens on Velodrome
pub mod swap;

/// Re-export a prelude
pub mod prelude {
    pub use super::{amm::*, metadata::*, deposit::*, withdraw::*, swap::*, open::*, close::*, args::*, position::*, portfolio::*, quit::*, address_list::*, contracts::*, utils::*};
}