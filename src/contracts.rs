#![allow(missing_docs)]

use ethers::{
    prelude::*, 
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::Address,
};
use crate::{address_list, utils};
use eyre::Result;

abigen!(AccountBalanceContract, "src/abis/IAccountBalance.json");
abigen!(BaseTokenContract, "src/abis/IBaseToken.json");
abigen!(ClearingHouseContract, "src/abis/IClearingHouse.json");
abigen!(CollateralManagerContract, "src/abis/ICollateralManager.json");
abigen!(ExchangeContract, "src/abis/IExchange.json");
abigen!(MarketRegistryContract, "src/abis/IMarketRegistry.json");
abigen!(OrderBookContract, "src/abis/IOrderBook.json");
abigen!(PerpPortalContract, "src/abis/IPerpPortal.json");
abigen!(PoolContract, "src/abis/IPool.json");
abigen!(QuoteTokenContract, "src/abis/IQuoteToken.json");
abigen!(VaultContract, "src/abis/IVault.json");
abigen!(VelodromeContract, "src/abis/IVelodromeRouter.json");
abigen!(TokenContract, "src/abis/IErc20.json");

/// Account Balance Contains Mostly Functions to Obtain Information Regarding Position Sizes
pub async fn get_account_balance() -> Result<AccountBalanceContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = AccountBalanceContract::new(address_list::get_account_balance().await?.parse::<Address>()?, utils::create_http_client()?);
    Ok(contract)
}

pub fn get_velodrome_contract() -> Result<VelodromeContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = VelodromeContract::new(address_list::get_velodrome()?, utils::create_http_client()?);
    Ok(contract)
}

pub fn get_token_contract(token: Address) -> Result<TokenContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = TokenContract::new(token, utils::create_http_client()?);
    Ok(contract)
}

/// This is the base token contract that can work for any base token passed to it
pub fn get_base_contract(token: Address) -> Result<BaseTokenContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = BaseTokenContract::new(token, utils::create_http_client()?);
    Ok(contract)
}

/// The contract responsible for processing orders
pub async fn get_clearing_house() -> Result<ClearingHouseContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = ClearingHouseContract::new(address_list::get_clearing_house().await?.parse::<Address>()?, utils::create_http_client()?);
    Ok(contract)
}

/// The contract responsible for collateral management
pub async fn get_collateral_manager() -> Result<CollateralManagerContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = CollateralManagerContract::new(address_list::get_contract_addresses().await?.get("CollateralManager").expect("Collateral address from json").to_owned(), utils::create_http_client()?);
    Ok(contract)
}

/// The contract responsible for exchange
pub async fn get_exchange() -> Result<ExchangeContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = ExchangeContract::new(address_list::get_contract_addresses().await?.get("Exchange").expect("Exchange address from JSON").to_owned(), utils::create_http_client()?);
    Ok(contract)
}

/// The contract responsible for market registry
pub async fn get_market_registry() -> Result<MarketRegistryContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = MarketRegistryContract::new(address_list::get_contract_addresses().await?.get("MarketRegistry").expect("Market Registry address from JSON").to_owned(), utils::create_http_client()?);
    Ok(contract)
}

/// OrderBook contract contains more in-depth information about positions
pub async fn get_order_book() -> Result<OrderBookContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = OrderBookContract::new(address_list::get_contract_addresses().await?.get("OrderBook").expect("Order Book address from JSON").to_owned(), utils::create_http_client()?);
    Ok(contract)
}

/// Swiss-Army knife of contracts with almost every possible function needed
pub fn get_perp_portal() -> Result<PerpPortalContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = PerpPortalContract::new(address_list::get_perp_portal()?, utils::create_http_client()?);
    Ok(contract)
}

/// Contract for the liquidity pools
pub fn get_pool_contract(token: Address) -> Result<PoolContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = PoolContract::new(token, utils::create_http_client()?);
    Ok(contract)
}

/// This is the quote token contract that can work for any quote token passed to it
pub fn get_quote_contract(token: Address) -> Result<QuoteTokenContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = QuoteTokenContract::new(token, utils::create_http_client()?);
    Ok(contract)
}

/// The contract responsible for holding collateral assets
pub async fn get_vault() -> Result<VaultContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let contract = VaultContract::new(address_list::get_contract_addresses().await?.get("Vault").expect("Vault address from JSON").to_owned(), utils::create_http_client()?);
    Ok(contract)
}