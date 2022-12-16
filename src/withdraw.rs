use crate::prelude::{WithdrawCommand};
use ethers::types::Address;
use crate::{address_list, contracts, utils};
use ethers::prelude::*;

#[tokio::main]
/// Process withdraw requests
pub async fn process(args: WithdrawCommand) {
    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider.clone()).expect("Failed");
    let vault_contract = contracts::get_vault(&client);
    let zero_address = String::from("0x0000000000000000000000000000000000000000").parse::<Address>().unwrap();

    let collaterals = address_list::get_collateral_tokens();

    if args.token == None && args.amount == None && args.eth == None {
        for (key, val) in &collaterals { println!("{}: {:?}", key, val); }
        eprintln!("");
    }

    let mut token_address = zero_address;
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

    let base_contract = contracts::get_base_contract(&client, token_address);

    match args.amount {
        Some(amount) => { 
            let decimals = base_contract
               .method::<_, u8> ("decimals", ())
               .expect("Failed")
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
               .method::<_, ()> ("withdrawEther", amount)
               .expect("Couldn't Send tx")
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

    if token_address != zero_address && args.amount != None && args.eth == None {
        let withdraw: TransactionReceipt = vault_contract
            .method::<_, ()> ("withdraw", (token_address, withdraw_amount))
            .expect("Couldn't send transaction")
            .send()
            .await
            .expect("Failed")
            .await
            .expect("Failed")
            .unwrap();

            println!("Withdrew {:?} {}\nTransaction: {:#?}", args.amount, token_symbol, withdraw.transaction_hash);
        }
}   
