use crate::prelude::WithdrawCommand;
use ethers::types::Address;
use crate::{address_list, contracts};
use ethers::prelude::*;
use eyre::Result;

/// Process withdraw requests
pub async fn process(args: WithdrawCommand) -> Result<()> {
    let vault_contract = contracts::get_vault().await?;
    let collaterals = address_list::get_collateral_tokens()?;

    if args.token.is_none() && args.amount.is_none() && args.eth.is_none() {
        println!();
        for (key, val) in &collaterals { println!("{}: {:?}", key, val); }
        println!();
    }

    let mut token_address = Address::zero();
    let mut withdraw_amount = U256::zero();
    let mut token_symbol = String::new();

    if let Some(token) = args.token {
        for (key, val) in collaterals {
            if token.parse::<Address>()? != val { continue; }
            if token.parse::<Address>()? == val { 
                token_symbol = key;
                break; 
            }
            panic!("Token address given doesn't match accepted list of collaterals. Use 'perp withdraw' to see available tokens.");
        }
        token_address = token.parse::<Address>()?;        
    }

    let base_contract = contracts::get_base_contract(token_address)?;

    if let Some(amount) = args.amount {
        let decimals = base_contract
            .decimals()
            .call()
            .await?;
        withdraw_amount = ethers::utils::parse_units(amount, decimals as u32)?.into();
    }

    if let Some(eth) = args.eth {
        let amount: U256 = ethers::utils::parse_units(eth, "ether")?.into();
        let tx = vault_contract
           .withdraw_ether(amount)
           .send()
           .await?
           .await?
           .expect("Withdraw Ether from Vault Contract");

        println!("Withdrew {} ETH\nTransaction: {:#?}", eth, tx.transaction_hash);        
    }

    if token_address != Address::zero() && args.amount.is_some() && args.eth.is_none() {
        let withdraw: TransactionReceipt = vault_contract
            .withdraw(token_address, withdraw_amount)
            .send()
            .await?
            .await?
            .expect("Withdraw through the Vault Contract");

            println!("Withdrew {:?} {}\nTransaction: {:#?}", args.amount, token_symbol, withdraw.transaction_hash);
        }
        Ok(())
}   
