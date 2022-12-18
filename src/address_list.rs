use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

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
pub async fn address_list() -> String {
    let address_json = reqwest::get("https://metadata.perp.exchange/v2/optimism.json").await.unwrap().text().await.unwrap();
    address_json
}

/// Find an Address
async fn init_data() -> AddressList {
    let json_original = address_list().await;
    let address_data: AddressList = serde_json::from_str(&json_original).expect("Not Valid JSON");
    address_data 
}

/// List of Liquidity Pools
pub async fn get_pools() -> Vec<Pools> {
    let init_data = init_data().await;
    init_data.pools
}

/// Contracts
pub async fn get_contracts() -> HashMap<String, ContractData> {
    let init_data = init_data().await;
    init_data.contracts
}

/// Vault Address as a String
pub async fn get_vault() -> String {
    let contracts = get_contracts().await;
    let contract_data = contracts["Vault"].clone();
    return contract_data.address;
}

/// Account Balance Address as a String
pub async fn get_account_balance() -> String {
    let contracts = get_contracts().await;
    let contract_data = contracts["AccountBalance"].clone();
    return contract_data.address;
}

/// Clearing House Address as a String
pub async fn get_clearing_house() -> String {
    let contracts = get_contracts().await;
    let contract_data = contracts["ClearingHouse"].clone();
    return contract_data.address;
}

/// Perp Portal Address
pub fn get_perp_portal() -> Address {
    let contract_data = String::from("0xa18fa074a2A5B01E69a35771E709553af4676558").parse::<Address>().unwrap();
    contract_data
}

/// Contract Addresses
pub async fn get_contract_addresses() -> HashMap<String, Address> {
    let contracts = get_contracts().await;
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
pub async fn get_token_addresses() -> HashMap<String, Address> {
    let contracts = get_contracts().await;
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

/// Collateral Tokens
pub fn get_collateral_tokens() -> HashMap<String, Address> {
    let collaterals = HashMap::from(
        [
            ("USDC".to_string(), "0x7F5c764cBc14f9669B88837ca1490cCa17c31607".parse::<Address>().unwrap()),
            ("USDT".to_string(), "0x94b008aA00579c1307B0EF2c499aD98a8ce58e58".parse::<Address>().unwrap()),
            ("WETH".to_string(), "0x4200000000000000000000000000000000000006".parse::<Address>().unwrap()),
            ("FRAX".to_string(), "0x2E3D870790dC77A83DD1d18184Acc7439A53f475".parse::<Address>().unwrap()),
            ("OP".to_string(), "0x4200000000000000000000000000000000000042".parse::<Address>().unwrap()),
        ]
    );
    collaterals
}