use clap::{ Args, Parser, Subcommand, ArgAction };

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// Pulls in Primary Commands
pub struct PerpArgs{
    #[clap(subcommand)]
    /// The command to run
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
/// Which Command should we parse?
pub enum SubCommand {
    /// Use this command for information on specific positions
    Position(PositionCommand),
    /// Use this command for a list of all current positions and assets for a wallet
    Portfolio(PortfolioCommand),
    /// Use this command to find information about liquidity pools
    Amm(AmmCommand),
    // /// Verify Function Data
    // Verify(VerifyCommand)
}

#[derive(Debug, Args)]
/// Position Command
pub struct PositionCommand { 
    #[clap(short, long)]
    /// Use the trader address to filter only that trader's positions
    pub trader: Option<String>,
    #[clap(short, long)]
    /// Use the base token address to filter only positions for that token
    pub base_token: Option<String>,
    #[clap(short, long)]
    /// Limit the amount of blocks to look back for positions
    pub limit: Option<usize>,
}

#[derive(Debug, Args)]
/// Portfolio Command
pub struct PortfolioCommand {
    /// The address of the trader
    pub trader_address: String,
}

#[derive(Debug, Args)]
/// Amm Command
pub struct AmmCommand {
    #[clap(short, long, action(ArgAction::SetTrue),)]
    /// --short flag prints list of available pools
    pub short: Option<bool>,
    /// Search with pool address, base token address or base token symbol (ie vBTC, vPERP, vSOL)
    pub search_parameter: Option<String>,
}

// #[derive(Debug, Args)]
// /// Verify Command
// pub struct VerifyCommand {
//     /// Contract address to send the byte code
//     pub contract_address: String,
//     /// Byte code 
//     pub byte_code: String,
// }