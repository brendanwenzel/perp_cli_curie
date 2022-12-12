#[path = "./IAccountBalanceABI.rs"]
mod account_balance;

#[path = "./IClearingHouseABI.rs"]
mod clearing_house;

#[path = "./IBaseTokenABI.rs"]
mod base_token;

#[path = "./IQuoteTokenABI.rs"]
mod quote_token;

#[path = "./ICollateralManagerABI.rs"]
mod collateral_manager;

#[path = "./IExchangeABI.rs"]
mod exchange;

#[path = "./IMarketRegistryABI.rs"]
mod market_registry;

#[path = "./IVaultABI.rs"]
mod vault;

#[path = "./IPerpPortal.rs"]
mod perp_portal;

#[path = "./IOrderBookABI.rs"]
mod order_book;

#[path = "./IPoolABI.rs"]
mod pool;

/// ABI for AccountBalance Contract
pub fn account_balance() -> String {
    return account_balance::i_account_balance_abi();
}

/// ABI for ClearingHouse Contract
pub fn clearing_house() -> String {
    return clearing_house::i_clearing_house_abi();
}

/// ABI for BaseToken Contract
pub fn base_token() -> String {
    return base_token::i_base_token_abi();
}
#[allow(dead_code)]
/// ABI for CollateralManager Contract
pub fn collateral_manager() -> String {
    return collateral_manager::i_collateral_manager_abi();
}
#[allow(dead_code)]
/// ABI for Exchange Contract
pub fn exchange() -> String {
    return exchange::i_exchange_abi();
}
#[allow(dead_code)]
/// ABI for MarketRegistry Contract
pub fn market_registry() -> String {
    return market_registry::i_market_registry_abi();
}

/// ABI for PerpPortal Contract
pub fn perp_portal() -> String {
    return perp_portal::i_perp_portal();
}

/// ABI for OrderBook Contract
pub fn order_book() -> String {
    return order_book::i_order_book_abi();
}

/// ABI for Vault Contract
pub fn vault() -> String {
    return vault::i_vault_abi();
}

/// ABI for QuoteToken Contract
pub fn quote_token() -> String {
    return quote_token::i_quote_token_abi();
}

/// ABI for QuoteToken Contract
pub fn pool() -> String {
    return pool::i_pool_abi();
}