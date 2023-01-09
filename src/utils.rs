use std::sync::Arc;

use ethers::prelude::*;
use eyre::Result;
use dotenv::dotenv;

/// Read environment variables
pub fn read_env_vars() -> Result<Vec<(String, String)>> {
    dotenv().ok();
    let mut env_vars = Vec::new();
    let keys = vec![
        "RPC_URL",
        "CHAIN_ID",
        "PRIVATE_KEY"
    ];
    for key in keys {
        // Read environment variable
        let value = std::env::var(key)
            .map_err(|_| eyre::eyre!("Required environment variable \"{}\" not set - READENVVARS", key))?;
        env_vars.push((key.to_string(), value));
    }
    Ok(env_vars)
}

/// Return a Provider for the given URL
pub fn get_http_provider() -> Result<Provider<Http>> {
    dotenv().ok();
    let url = std::env::var("RPC_URL")
        .map_err(|_| eyre::eyre!("Required environment variable \"RPC_URL\" not set - get_http_provider"))?;
    Provider::<Http>::try_from(url).map_err(|_| eyre::eyre!("Invalid RPC URL"))
}

/// Construct the searcher wallet
pub fn get_wallet() -> Result<LocalWallet> {
    dotenv().ok();
    let private_key = std::env::var("PRIVATE_KEY")
        .map_err(|_| eyre::eyre!("Required environment variable \"PRIVATE_KEY\" not set"))?;
    private_key
        .parse::<LocalWallet>()
        .map_err(|e| eyre::eyre!("Failed to parse private key: {:?}", e))
}

/// Creates a client from a provider
pub fn create_http_client() -> Result<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let wallet = get_wallet()?;
    let provider = get_http_provider()?;
    let chain_id: u64 = std::env::var("CHAIN_ID")
        .map_err(|_| eyre::eyre!("Required environment variable \"RPC_URL\" not set - get_http_provider"))?
        .parse::<u64>()?;
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    Ok(Arc::new(client))
}
