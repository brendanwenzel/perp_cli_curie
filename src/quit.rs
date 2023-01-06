use crate::args::QuitCommand;
use crate::{address_list, contracts, utils};
use ethers::prelude::*;
use serde::Serialize;
use eyre::Result;

#[derive(Clone, EthEvent, Serialize, Debug)]
struct QuitMarket {
    base: U256,
    quote: U256,
}

#[tokio::main]
/// Process the request to quit market.
pub async fn process(args: QuitCommand) -> Result<()> {
    let signer = utils::get_wallet()?;
    let trader_address = signer.address();
    let mut token_address = if args.token.len() == 42 { args.token.parse::<Address>()? } else { Address::zero() };
    let contract = contracts::get_clearing_house().await?;
    let token_addresses = address_list::get_token_addresses().await?;

    if args.token.len() < 41 {
        for (key, val) in token_addresses.clone() {
            let mut chars = key.chars();
            chars.next();
            let key_without_v = chars.as_str();
            if key_without_v == args.token { token_address = val; break; }
            if key != args.token { continue; }
            token_address = val;
            break;
        }
    }

    let mut base_symbol = String::new();
    for (key, val) in token_addresses {
        if val != token_address {continue;}
        base_symbol = key;
    }

    let quit_market = contract.quit_market(trader_address, token_address).send().await?.await?;

    println!("");
    println!("Closed all {} positions: {:?}", base_symbol, quit_market);
Ok(())
}