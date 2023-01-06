use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::metadata;
use eyre::Result;

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

/// Find an Address
async fn init_data() -> Result<AddressList> {
    let json_original = reqwest::get("https://metadata.perp.exchange/v2/optimism.json").await;
    let json = match json_original {
            Ok(response) => response.text().await?,
            Err(_e) => {String::new()},
        };
    let address_data: Result<AddressList, serde_json::Error> = serde_json::from_str(&json);
        match address_data {
            Ok(data) => Ok(data),
            Err(_e) => {
                let json = metadata::get_metadata_json();
                let data = serde_json::from_str(&json)?;
                Ok(data)
            }
        }
}

/// List of Liquidity Pools
pub async fn get_pools() -> Result<Vec<Pools>> {
    let init_data = init_data().await?;
    Ok(init_data.pools)
}

/// Contracts
pub async fn get_contracts() -> Result<HashMap<String, ContractData>> {
    let init_data = init_data().await?;
    Ok(init_data.contracts)
}

/// Vault Address as a String
pub async fn get_vault() -> Result<String> {
    let contracts = get_contracts().await?;
    let contract_data = contracts["Vault"].clone();
    Ok(contract_data.address)
}

/// Account Balance Address as a String
pub async fn get_account_balance() -> Result<String> {
    let contracts = get_contracts().await?;
    let contract_data = contracts["AccountBalance"].clone();
    Ok(contract_data.address)
}

/// Clearing House Address as a String
pub async fn get_clearing_house() -> Result<String> {
    let contracts = get_contracts().await?;
    let contract_data = contracts["ClearingHouse"].clone();
    Ok(contract_data.address)
}

/// Perp Portal Address
pub fn get_perp_portal() -> Result<Address> {
    let contract_data = String::from("0xa18fa074a2A5B01E69a35771E709553af4676558").parse::<Address>()?;
    Ok(contract_data)
}

/// Contract Addresses
pub async fn get_contract_addresses() -> Result<HashMap<String, Address>> {
    let contracts = get_contracts().await?;
    let mut contract_addresses : HashMap<String, Address> = HashMap::new();
    for (key, value) in contracts {
        // if value.name == String::from("contracts/BaseToken.sol:BaseToken") || value.name == String::from("contracts/ChainlinkPriceFeedV2.sol:ChainlinkPriceFeedV2") {continue;}
        let address = value.address
            .parse::<Address>()
            .expect("Failed to make Address");
        contract_addresses.insert(key, address);
    }
    Ok(contract_addresses)
}

/// Token Addresses
pub async fn get_token_addresses() -> Result<HashMap<String, Address>> {
    let contracts = get_contracts().await?;
    let mut token_addresses: HashMap<String, Address> = HashMap::new();
    for (key, val) in contracts {
    if val.name == String::from("contracts/BaseToken.sol:BaseToken") {
        let address = val.address
            .parse::<Address>()
            .expect("Failed to make Address");
        token_addresses.insert(key, address);
        }
    };
    Ok(token_addresses)
}

/// Collateral Tokens
pub fn get_collateral_tokens() -> Result<HashMap<String, Address>> {
    let collaterals = HashMap::from(
        [
            ("USDC".to_string(), "0x7F5c764cBc14f9669B88837ca1490cCa17c31607".parse::<Address>()?),
            ("USDT".to_string(), "0x94b008aA00579c1307B0EF2c499aD98a8ce58e58".parse::<Address>()?),
            ("WETH".to_string(), "0x4200000000000000000000000000000000000006".parse::<Address>()?),
            ("FRAX".to_string(), "0x2E3D870790dC77A83DD1d18184Acc7439A53f475".parse::<Address>()?),
            ("OP".to_string(), "0x4200000000000000000000000000000000000042".parse::<Address>()?),
        ]
    );
    Ok(collaterals)
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[tokio::test]
    async fn test_tokens() -> Result<()> {
        let token_addresses = get_token_addresses().await?;
        assert_eq!(token_addresses.len(), 19);
        Ok(())
    }

    #[test]
    fn test_collateral() -> Result<()> {
        let token_addresses = get_collateral_tokens();
        assert_eq!(token_addresses?.len(), 5);
        Ok(())
    }

    #[tokio::test]
    async fn test_vault() -> Result<()> {
        assert_eq!(get_vault().await?, "0xAD7b4C162707E0B2b5f6fdDbD3f8538A5fbA0d60");
        Ok(())
    }

    #[tokio::test]
    async fn test_contracts() -> Result<()> {
        assert_eq!(get_contract_addresses().await?.len(), 55);
        Ok(())
    }

}