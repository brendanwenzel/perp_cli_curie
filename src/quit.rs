use crate::args::QuitCommand;
use crate::{address_list, contracts, utils};
use ethers::prelude::*;
use serde::Serialize;

#[derive(Clone, EthEvent, Serialize, Debug)]
struct QuitMarket {
    base: U256,
    quote: U256,
}

#[tokio::main]
/// Process the request to quit market.
pub async fn process(token: QuitCommand) {
    let signer = utils::get_wallet().unwrap();
    let trader_address = signer.address();
    let token_address = token.base_token.parse::<Address>().unwrap();
    let contract = contracts::get_clearing_house().await;

    let mut base_symbol = String::new();
    let token_addresses = address_list::get_token_addresses().await;
    for (key, val) in token_addresses {
        if val != token_address {continue;}
        base_symbol = key;
    }

    let quit_market = contract.quit_market(trader_address, token_address).send().await.expect("Failed").await.expect("Failed");

    println!("");
    println!("Closed all {} positions: {:?}", base_symbol, quit_market);

}