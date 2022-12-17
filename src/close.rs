use crate::{args::CloseCommand, address_list, utils};
use serde::Serialize;
use ethers::prelude::*;

#[tokio::main]
/// The function to process the Close command
pub async fn process(args: CloseCommand) {
    
    abigen!(ClearingHouseContract, "src/abis/IClearingHouse.json");

    #[derive(Clone, Debug, EthEvent, Serialize)]
    struct PositionChanged {
        #[ethevent(indexed)]
        trader: Address,
        #[ethevent(indexed)]
        #[serde(rename="baseToken")]
        base_token: Address,
        #[serde(rename="exchangedPositionSize")]
        exchanged_position_size: I256,
        #[serde(rename="exchangedPositionNotional")]
        exchanged_position_notional: I256,
        fee: U256,
        #[serde(rename="openNotional")]
        open_notional: I256,
        #[serde(rename="realizedPnl")]
        realized_pnl: I256,
        #[serde(rename="sqrtPriceAfterX96")]
        sqrt_price_after_x96: U256,
    }

    let close_position_params = ClosePositionParams {
        base_token: args.token.parse::<Address>().unwrap(),
        sqrt_price_limit_x96: U256::zero(),
        opposite_amount_bound: U256::zero(),
        deadline: U256::max_value(),
        referral_code: H256::zero().to_fixed_bytes(),
    };

    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider).expect("Failed");
    let clearing_house_address = address_list::get_clearing_house().parse::<Address>().unwrap();
    let contract = ClearingHouseContract::new(clearing_house_address, client);
    let mut base_symbol: String = String::new();
    let token_addresses = address_list::get_token_addresses();
    let mut _direction = String::new();
    for (key, val) in token_addresses {
        if val != close_position_params.base_token {continue;}
        base_symbol = key.parse::<String>().unwrap();
        break;
        }

    let tx = contract.close_position(close_position_params).send().await.expect("Failed").await.expect("Failed");
    let tx_receipt = tx.unwrap();
    let logs: Vec<PositionChanged> = contract
        .event()
        .from_block(tx_receipt.block_number.unwrap())
        .query()
        .await
        .expect("Failed");

    let position_size = ethers::utils::format_units(logs[0].exchanged_position_size, "ether").unwrap();
    let position_size_float = position_size.parse::<f64>().unwrap();

    let position_notional = ethers::utils::format_units(logs[0].exchanged_position_notional, "ether").unwrap();
    let position_notional_float = position_notional.parse::<f64>().unwrap();

    let avg_price = position_notional_float / position_size_float;

    println!("");
    println!("========================");
    println!("== CLOSING {} ==", base_symbol);
    println!("========================");
    println!("");
    println!("Transaction: {:#?}", tx_receipt.transaction_hash);
    println!("Position Size: {} {}", position_size_float, base_symbol);
    println!("Avg Price: {} USD", avg_price.abs());
    println!("Fee Paid: {} USD", ethers::utils::format_units(logs[0].fee, "ether").unwrap().parse::<f64>().unwrap());
    println!("Profit: {} USD", ethers::utils::format_units(logs[0].realized_pnl, "ether").unwrap().parse::<f64>().unwrap());
    println!("");
    println!("");
    
}