use crate::args::QuitCommand;
use crate::{address_list, utils};
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
    abigen!(ClearingHouseContract, "src/abis/IClearingHouse.json");

    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider.clone()).expect("Failed");
    let signer = utils::get_wallet().unwrap();
    let trader_address = signer.address();
    let token_address = token.base_token.parse::<Address>().unwrap();
    let clearing_house_address = address_list::get_clearing_house().parse::<Address>().unwrap();
    let contract = ClearingHouseContract::new(clearing_house_address, client);

    let mut base_symbol = String::new();
    let token_addresses = address_list::get_token_addresses();
    for (key, val) in token_addresses {
        if val != token_address {continue;}
        base_symbol = key;
    }

    let quit_market = contract.quit_market(trader_address, token_address).send().await.expect("Failed").await.expect("Failed");

    println!("");
    println!("Closed all {} positions: {:?}", base_symbol, quit_market);

}