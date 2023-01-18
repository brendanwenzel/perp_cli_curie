use clap::{ArgAction, Args, Parser, Subcommand};
use ethers::types::Address;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// Pulls in Primary Commands
pub struct PerpArgs {
    #[clap(subcommand)]
    /// The command to run
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
/// Which Command should we parse?
pub enum SubCommand {
    /// Information on most recent position changes
    Position(PositionCommand),
    /// List of all current positions and assets for a wallet
    Portfolio(PortfolioCommand),
    /// Find information about liquidity pools
    Amm(AmmCommand),
    /// List all base token symbols and addresses
    Tokens(TokensCommand),
    /// Deposit new collateral into Perp account
    Deposit(DepositCommand),
    /// Withdraw collateral from Perp account
    Withdraw(WithdrawCommand),
    /// Close position for specified market
    Close(CloseCommand),
    /// Open New Position
    Open(OpenCommand),
    /// Close all positions in a CLOSED market.
    Quit(QuitCommand),
    /// Swap ETH or ERC-20 tokens on Velodrome for collateral tokens
    Swap(SwapCommand),
    /// Setup or edit the configuration settings of the app
    Config(ConfigCommand),
}

#[derive(Debug, Args)]
/// Setting up configuration settings for the app
pub struct ConfigCommand {
    #[clap(long, action(ArgAction::SetTrue))]
    /// Provide an RPC URL from a node provider or your local node
    pub rpc: Option<bool>,
    #[clap(long, action(ArgAction::SetTrue))]
    /// Provide a different Chain ID for testing purposes with forked networks
    pub chain: Option<bool>,
    #[clap(long, action(ArgAction::SetTrue))]
    /// Provide the key for the wallet doing the trading
    pub pk: Option<bool>,
}

#[derive(Debug, Args)]
/// Arguments in order to make a swap
pub struct SwapCommand {
    /// Token Address of token in
    pub token_in: Address,
    /// The amount of tokens to swap
    pub amount_in: f64,
    /// Token Address of token out
    pub token_out: Address,
    /// The percentage of acceptable slippage for the swap.
    /// ie 1.5 would mean 1.5% slippage is acceptable.
    pub slippage: f64,
    #[clap(long, action(ArgAction::SetTrue))]
    /// If you want to swap to or from unwrapped ETH
    /// use the --ETH flag at the end of the command
    pub eth: Option<bool>,
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
/// Portfolio Command
pub struct PortfolioCommand {
    /// The address of the trader
    pub trader_address: Option<String>,
}

#[derive(Debug, Args)]
/// Amm Command
pub struct AmmCommand {
    #[clap(short, long, action(ArgAction::SetTrue))]
    /// --short flag prints list of available pools
    pub short: Option<bool>,
    /// Search with pool address, base token address or base token symbol (ie vBTC, vPERP, vSOL)
    pub search_parameter: Option<String>,
}

#[derive(Debug, Args)]
/// Deposit Command
pub struct DepositCommand {
    /// Token address to deposit as collateral. Use "perp deposit" for list of accepted collateral tokens.
    pub token: Option<Address>,
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

#[derive(Debug, Args)]
/// Open Position Command
pub struct OpenCommand {
    #[clap(long, action(ArgAction::SetTrue))]
    /// Use this flag to open a long position
    pub long: Option<bool>,
    #[clap(long, action(ArgAction::SetTrue))]
    /// Use this flag to open a short position
    pub short: Option<bool>,
    /// Base token address or symbol
    pub token: String,
    #[clap(long, action(ArgAction::SetTrue))]
    /// Use this flag to specify amount IN
    pub input: Option<bool>,
    #[clap(long, action(ArgAction::SetTrue))]
    /// Use this flag to specify amount OUT
    pub output: Option<bool>,
    /// Amount to buy in decimals. Will convert to Wei in backend.
    pub order_amount: f64,
    #[clap(long)]
    /// Will open the order up to the point of hitting this limit
    pub limit: Option<f64>,
}

#[derive(Debug, Args)]
/// Close Position Command
pub struct CloseCommand {
    /// Base token address or symbol to trade
    pub token: String,
}

#[derive(Debug, Args)]
/// Close all positions in a CLOSED market. Can only be used when a market is no longer active.
pub struct QuitCommand {
    /// Base token address to close positions for
    pub token: String,
}
