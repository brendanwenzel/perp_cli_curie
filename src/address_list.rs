#[path = "./abis/IAddressList.rs"]
mod address_list;

use ethers::{types::Address};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contract Addresses
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ContractAddresses {
//     /// AccountBalance
//     pub account_balance_address: Address,
//     /// ClearingHouse
//     pub clearing_house_address: Address,
//     /// Base Token
//     pub vault_address: Address,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Contract Information
pub struct ContractData {
    /// Contract Address
    pub address: String,
    /// Block Created
    #[serde(rename="createdBlockNumber")]
    pub created_block_number: u64,
    /// Name of Contract
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Liquidity Pools
pub struct Pools {
    /// Liquidity Pool Address
    pub address: String,
    /// Base Token Address
    #[serde(rename="baseAddress")]
    pub base_address: String,
    /// Base Token Symbol
    #[serde(rename="baseSymbol")]
    pub base_symbol: String,
    /// Quote token Address
    #[serde(rename="quoteAddress")]
    pub quote_address: String,
    /// Quote token Symbol
    #[serde(rename="quoteSymbol")]
    pub quote_symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// List of Collateral Accepted
pub struct Collateral {
    /// Contract Address for Collateral
    pub address: String,
    /// Decimals for Token
    pub decimals: u8,
    /// Name of Collateral Tokens
    pub name: String,
    /// Address for the Price Feed
    #[serde(rename="priceFeedAddress")]
    pub price_feed_address: String,
    /// Collateral Token Symbol
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Primary Structure
pub struct AddressList {
    /// Chain Id for Network
    #[serde(rename="chainId")]
    pub chain_id: u8,
    /// List of Accepted Collateral
    pub collaterals: Vec<Collateral>,
    /// List of Contracts
    pub contracts: HashMap<String, ContractData>,
    /// list of External Contracts
    #[serde(rename="externalContracts")]
    pub external_contracts: HashMap<String, String>,
    /// Network Name
    pub network: String,
    /// List of Liquidity Pools
    pub pools: Vec<Pools>,
}

/// List of all Addresses for Optimism
pub fn address_list() -> String {
    return address_list::i_addresses_json();
}

/// Find an Address
fn init_data() -> AddressList {
    let json_original: String = address_list();
    let address_data: AddressList = serde_json::from_str(&json_original).expect("Not Valid JSON");
    address_data 
}

/// List of Liquidity Pools
pub fn get_pools() -> Vec<Pools> {
    return init_data().pools;
}

/// Contracts
pub fn get_contracts() -> HashMap<String, ContractData> {
    return init_data().contracts;
}

/// Vault Address as a String
pub fn get_vault() -> String {
    let contracts = get_contracts();
    let contract_data = contracts["Vault"].clone();
    return contract_data.address;
}

/// Account Balance Address as a String
pub fn get_account_balance() -> String {
    let contracts = get_contracts();
    let contract_data = contracts["AccountBalance"].clone();
    return contract_data.address;
}

/// Clearing House Address as a String
pub fn get_clearing_house() -> String {
    let contracts = get_contracts();
    let contract_data = contracts["ClearingHouse"].clone();
    return contract_data.address;
}

/// Contract Addresses
pub fn get_contract_addresses() -> HashMap<String, Address> {
    let contracts = get_contracts();
    let mut contract_addresses : HashMap<String, Address> = HashMap::new();
    for (key, value) in contracts {
        // if value.name == String::from("contracts/BaseToken.sol:BaseToken") || value.name == String::from("contracts/ChainlinkPriceFeedV2.sol:ChainlinkPriceFeedV2") {continue;}
        let address = value.address
            .parse::<Address>()
            .expect("Failed to make Address");
        contract_addresses.insert(key, address);
    }
    contract_addresses
}

/// Token Addresses
pub fn get_token_addresses() -> HashMap<String, Address> {
    let contracts = get_contracts();
    let mut token_addresses: HashMap<String, Address> = HashMap::new();
    for (key, val) in contracts {
    if val.name == String::from("contracts/BaseToken.sol:BaseToken") {
        let address = val.address
            .parse::<Address>()
            .expect("Failed to make Address");
        token_addresses.insert(key, address);
        }
    };
    token_addresses
}

// /// Contract Addresses
// pub fn contracts() -> ContractAddresses {
//     ContractAddresses {
//         account_balance_address: get_account_balance()
//             .parse::<Address>()
//             .expect("fail"),
//         clearing_house_address: "0x82ac2CE43e33683c58BE4cDc40975E73aA50f459"
//             .parse::<Address>()
//             .expect("fail"),
//         vault_address: get_vault()
//             .parse::<Address>()
//             .expect("failed making address"),
//     }
// }