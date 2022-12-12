use ethers::middleware::SignerMiddleware;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::Address,
};

#[path = "./abis/mod.rs"]
mod abis;
use crate::address_list;

/// Account Balance Contains Mostly Functions to Obtain Information Regarding Position Sizes
pub fn get_account_balance(provider: &SignerMiddleware<Provider<Http>, LocalWallet>) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::account_balance();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address = address_list::get_contract_addresses().get("AccountBalance").unwrap().to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// The contract responsible for processing orders
pub fn get_clearing_house(provider: &SignerMiddleware<Provider<Http>, LocalWallet>) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::clearing_house();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address = address_list::get_contract_addresses().get("ClearingHouse").unwrap().to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// The contract responsible for holding collateral assets
pub fn get_vault(provider: &SignerMiddleware<Provider<Http>, LocalWallet>) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::vault();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address: Address = address_list::get_contract_addresses().get("Vault").unwrap().to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// This is the base token contract that can work for any base token passed to it
pub fn get_base_contract(provider: &SignerMiddleware<Provider<Http>, LocalWallet>, token: Address) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::base_token();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address: Address = token.to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// OrderBook contract contains more in-depth information about positions
pub fn get_order_book(provider: &SignerMiddleware<Provider<Http>, LocalWallet>) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::order_book();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address: Address = address_list::get_contract_addresses().get("OrderBook").unwrap().to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// Swiss-Army knife of contracts with almost every possible function needed
pub fn get_perp_portal(provider: &SignerMiddleware<Provider<Http>, LocalWallet>) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::perp_portal();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address = String::from("0xa18fa074a2A5B01E69a35771E709553af4676558").parse::<Address>().expect("Failed to get Perp Portal Address");
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// This is the quote token contract that can work for any quote token passed to it
pub fn get_quote_contract(provider: &SignerMiddleware<Provider<Http>, LocalWallet>, token: Address) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::quote_token();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address: Address = token.to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}

/// Contract for the liquidity pools
pub fn get_pool_contract(provider: &SignerMiddleware<Provider<Http>, LocalWallet>, token: Address) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::pool();
    let abi: Abi = serde_json::from_str(&abi_original).expect("Failed");
    let address: Address = token.to_owned();
    let contract: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> = Contract::new(address, abi, provider);
    contract
}