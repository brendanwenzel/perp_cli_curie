use crate::prelude::{DepositCommand};
use ethers::types::Address;
use crate::{address_list, contracts, utils};
use ethers::prelude::*;
use eyre::Result;

#[tokio::main]
/// Process deposit requests
pub async fn process(args: DepositCommand) -> Result<()> {
    let client = utils::create_http_client()?;
    let vault_contract = contracts::get_vault().await?;
    let collaterals = address_list::get_collateral_tokens()?;

    if args.token == None && args.amount == None && args.eth == None {
        for (key, val) in &collaterals { println!("{}: {:?}", key, val); }
        eprintln!("");
    }

    let mut token_address = Address::zero();
    let mut deposit_amount = U256::zero();
    let mut token_symbol = String::new();

    match args.token {
        Some(token) => {
            for (key, val) in collaterals {
                if token.parse::<Address>()? != val { continue; }
                if token.parse::<Address>()? == val { 
                    token_symbol = key;
                    break; 
                }
                eprintln!("Token address given doesn't match accepted list of collaterals. Use 'perp deposit' to see available tokens.");
            }
            token_address = token.parse::<Address>()?;
        },
        None => {}
    }

    let base_contract = contracts::get_base_contract(token_address)?;

    match args.amount {
        Some(amount) => { 
            let decimals = base_contract
               .decimals()
               .call()
               .await?;
            deposit_amount = ethers::utils::parse_units(amount, decimals as u32)?.into();
         }
        None => {}
    }

    match args.eth {
        Some(eth) => {
            let amount: U256 = ethers::utils::parse_units(eth, "ether").unwrap().into();
            let data = vault_contract
               .deposit_ether()
               .calldata()
               .unwrap();

            let tx = TransactionRequest::new().to(vault_contract.address()).data(data).value(amount);
            let pending_tx = client.send_transaction(tx, None).await?;
            let receipt = pending_tx.await?.unwrap();
            // let tx = client.get_transaction(receipt.transaction_hash).await?.unwrap();

            eprintln!("Deposited {} ETH\nTransaction: {:#?}", eth, receipt.transaction_hash);
        },
        None => {}
    }

    if token_address != Address::zero() && args.amount != None && args.eth == None {
        let deposit: TransactionReceipt = vault_contract
            .deposit(token_address, deposit_amount)
            .send()
            .await?
            .await?
            .unwrap();

        println!("Deposited {:?} {}\nTransaction: {:#?}", args.amount, token_symbol, deposit.transaction_hash);
    }
    Ok(())
}   
