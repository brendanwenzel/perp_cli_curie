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
    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider.clone(), 10).expect("Failed");
    let clearing_house_contract = contracts::get_clearing_house(&client);
    let trader_address = utils::get_wallet().unwrap().address();
    let mut token_address =  String::from("0x0000000000000000000000000000000000000000").parse::<Address>().unwrap();
    match token.base_token {
        Some(token) => token_address = token.parse::<Address>().unwrap(),
        None => {eprintln!("Base Token Required");},
    }

    let mut base_symbol = String::new();
    let token_addresses = address_list::get_token_addresses();
    for (key, val) in token_addresses {
        if val != token_address {continue;}
        base_symbol = key;
    }

    let quit_market = clearing_house_contract
        .method::<_, QuitMarket>("quitMarket", (trader_address, token_address))
        .expect("Failed to call method")
        .call()
        .await
        .expect("Failed");

    println!("");
    println!("Closed all {} positions for {} USD", base_symbol, ethers::utils::format_units(quit_market.quote, "ether").unwrap());

}