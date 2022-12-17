#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// Common Utilities
pub mod utils;

/// Contract Instances
pub mod contracts;

/// ABIs
pub mod abis;

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

/// Open Position
pub mod open;

/// Close Position
pub mod close;

/// Re-export a prelude
pub mod prelude {
    pub use super::{abis::*, amm::*, deposit::*, withdraw::*, open::*, close::*, args::*, position::*, portfolio::*, quit::*, address_list::*, contracts::*, utils::*};
}