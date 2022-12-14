use crate::prelude::DepositCommand;
use ethers::types::Address;
use crate::{address_list, contracts, utils};
use ethers::prelude::*;
use eyre::Result;

/// Process deposit requests
pub async fn process(args: DepositCommand) -> Result<()> {
    let client = utils::create_http_client()?;
    let vault_contract = contracts::get_vault().await?;
    let collaterals = address_list::get_collateral_tokens()?;
    let wallet = utils::get_wallet()?;
    let wallet_address = wallet.address();

    if args.token.is_none() && args.amount.is_none() && args.eth.is_none() {
        println!();
        for (key, val) in &collaterals { println!("{}: {:?}", key, val); }
        println!();
    }

    if let Some(eth) = args.eth { 
        let amount: U256 = ethers::utils::parse_units(eth, "ether")?.into();
        let data = vault_contract
           .deposit_ether()
           .calldata()
           .expect("Deposit Ether with Vault Contract");

        let tx = TransactionRequest::new().to(vault_contract.address()).data(data).value(amount);
        let pending_tx = client.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?.expect("Transaction Reciept with hash number");
        println!("Deposited {} ETH\nTransaction: {:#?}", eth, receipt.transaction_hash);
        return Ok(());
    }

    let mut token_address = Address::zero();
    let mut deposit_amount = U256::zero();
    let mut token_symbol = String::new();

    if let Some(token) = args.token {
        for (key, val) in collaterals {
            if token != val { continue; }
            if token == val { 
                token_symbol = key;
                break; 
            }
            panic!("Token address given doesn't match accepted list of collaterals. Use 'perp deposit' to see available tokens.");
        }
        token_address = token;
    }

    let base_contract = contracts::get_base_contract(token_address)?;

    if let Some(amount) = args.amount {
        let decimals = base_contract
            .decimals()
            .call()
            .await?;
        deposit_amount = ethers::utils::parse_units(amount, decimals as u32)?.into();
    }

    if token_address != Address::zero() && args.amount.is_some() && args.eth.is_none() {
        println!("Checking for allowance to send token to contract");
        let allowance = base_contract
            .allowance(wallet_address, vault_contract.address())
            .call()
            .await?;
        if allowance < deposit_amount {
            println!("Not enough allowance to send token to contract");
            println!("Sending Approval for contract to take {} {}", args.amount.unwrap(), token_symbol);
            let approval = base_contract
                .approve(vault_contract.address(), deposit_amount)
                .send()
                .await?
                .await?
                .expect("Approval Request");
            println!("Approval succeeded and now sending deposit");
            println!("Approval Transaction Hash: {:#?}", approval.transaction_hash);
        }
        let deposit: TransactionReceipt = vault_contract
            .deposit(token_address, deposit_amount)
            .send()
            .await?
            .await?
            .expect("Transaction receipt for deposit");

        println!("Deposited {:?} {} with transaction hash:\n{:#?}", args.amount.unwrap(), token_symbol, deposit.transaction_hash);
        println!();
    }
    Ok(())
}