use crate::prelude::{WithdrawCommand};
use ethers::types::Address;
use crate::{address_list, contracts};
use ethers::prelude::*;

#[tokio::main]
/// Process withdraw requests
pub async fn process(args: WithdrawCommand) {
    let vault_contract = contracts::get_vault().await;
    let collaterals = address_list::get_collateral_tokens();

    if args.token == None && args.amount == None && args.eth == None {
        for (key, val) in &collaterals { println!("{}: {:?}", key, val); }
        eprintln!("");
    }

    let mut token_address = Address::zero();
    let mut withdraw_amount = U256::zero();
    let mut token_symbol = String::new();

    match args.token {
        Some(token) => {
            for (key, val) in collaterals {
                if token.parse::<Address>().unwrap() != val { continue; }
                if token.parse::<Address>().unwrap() == val { 
                    token_symbol = key;
                    break; 
                }
                eprintln!("Token address given doesn't match accepted list of collaterals. Use 'perp withdraw' to see available tokens.");
            }
            token_address = token.parse::<Address>().unwrap();
        },
        None => {}
    }

    let base_contract = contracts::get_base_contract(token_address);

    match args.amount {
        Some(amount) => { 
            let decimals = base_contract
               .decimals()
               .call()
               .await
               .expect("Failed to get decimals");
            withdraw_amount = ethers::utils::parse_units(amount, decimals as u32).unwrap().into();
         }
        None => {}
    }

    match args.eth {
        Some(eth) => {
            let amount: U256 = ethers::utils::parse_units(eth, "ether").unwrap().into();
            let tx = vault_contract
               .withdraw_ether(amount)
               .send()
               .await
               .expect("Failed")
               .await
               .expect("Failed")
               .unwrap();

            eprintln!("Withdrew {} ETH\nTransaction: {:#?}", eth, tx.transaction_hash);
        },
        None => {}
    }

    if token_address != Address::zero() && args.amount != None && args.eth == None {
        let withdraw: TransactionReceipt = vault_contract
            .withdraw(token_address, withdraw_amount)
            .send()
            .await
            .expect("Failed")
            .await
            .expect("Failed")
            .unwrap();

            println!("Withdrew {:?} {}\nTransaction: {:#?}", args.amount, token_symbol, withdraw.transaction_hash);
        }
}   
