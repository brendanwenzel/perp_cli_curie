use crate::prelude::{DepositCommand};
use ethers::types::Address;
use crate::{address_list, contracts, utils};
use ethers::prelude::*;

#[tokio::main]
/// Process deposit requests
pub async fn process(args: DepositCommand) {
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
    let mut deposit_amount = U256::zero();
    let mut token_symbol = String::new();

    match args.token {
        Some(token) => {
            for (key, val) in collaterals {
                if token.parse::<Address>().unwrap() != val { continue; }
                if token.parse::<Address>().unwrap() == val { 
                    token_symbol = key;
                    break; 
                }
                eprintln!("Token address given doesn't match accepted list of collaterals. Use 'perp deposit' to see available tokens.");
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
            deposit_amount = ethers::utils::parse_units(amount, decimals as u32).unwrap().into();
         }
        None => {}
    }

    match args.eth {
        Some(eth) => {
            let amount: U256 = ethers::utils::parse_units(eth, "ether").unwrap().into();
            let data = vault_contract
               .method::<_, ()> ("depositEther", ())
               .unwrap()
               .calldata()
               .unwrap();

            let tx = TransactionRequest::new().to(vault_contract.address()).data(data).value(amount);
            let pending_tx = client.send_transaction(tx, None).await.expect("failed");
            let receipt = pending_tx.await.expect("failed to get transaction receipt").expect("failed");
            // let tx = client.get_transaction(receipt.transaction_hash).await.expect("failed").unwrap();

            eprintln!("Deposited {} ETH\nTransaction: {:#?}", eth, receipt.transaction_hash);
        },
        None => {}
    }

    if token_address != zero_address && args.amount != None && args.eth == None {
        let deposit: TransactionReceipt = vault_contract
            .method::<_, ()> ("deposit", (token_address, deposit_amount))
            .expect("Couldn't send transaction")
            .send()
            .await
            .expect("Failed")
            .await
            .expect("Failed")
            .unwrap();

        println!("Deposited {:?} {}\nTransaction: {:#?}", args.amount, token_symbol, deposit.transaction_hash);
    }
}   
