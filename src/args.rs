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
    /// Close all positions in specific market 
    Quit(QuitCommand),
    /// List all base token symbols and addresses
    Tokens(TokensCommand),
    /// Deposit new collateral into Perp account
    Deposit(DepositCommand),
    /// Withdraw collateral from Perp account
    Withdraw(WithdrawCommand),
}

#[derive(Debug, Args)]
/// Tokens Command
pub struct TokensCommand {
    /// The address of the wallet to query
    #[clap(short, long)]
    pub symbol: Option<String>,
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
/// Close all positions in specific market
pub struct QuitCommand {
    /// Base token address to close positions for
    pub base_token: Option<String>,
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

#[derive(Debug, Args)]
/// Deposit Command
pub struct DepositCommand {
    /// Token address to deposit as collateral. Use "perp deposit" for list of accepted collateral tokens.
    pub token: Option<String>,
    /// Amount to deposit... for example 0.1 or 600.594... backend will convert to wei format.
    pub amount: Option<f64>,
    #[clap(long)]
    /// Deposit unwrapped Ether
    pub eth: Option<f64>,
}

#[derive(Debug, Args)]
/// Withdraw Command
pub struct WithdrawCommand {
    /// Token address to withdraw as collateral. Use "perp withdraw" for list of accepted collateral tokens.
    pub token: Option<String>,
    /// Amount to withdraw... for example 0.1 or 600.594... backend will convert to wei format.
    pub amount: Option<f64>,
    #[clap(long)]
    /// Withdraw unwrapped Ether
    pub eth: Option<f64>,
}

// #[derive(Debug, Args)]
// /// Verify Command
// pub struct VerifyCommand {
//     /// Contract address to send the byte code
//     pub contract_address: String,
//     /// Byte code 
//     pub byte_code: String,
// }