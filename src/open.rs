use ethers::prelude::*;
use crate::{address_list, contracts, args::OpenCommand};
use crate::prelude::{OpenPositionParams};
use serde::Serialize;

#[tokio::main]
/// The function to process the Open command
pub async fn process(args: OpenCommand) {

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

    if args.long == Some(false) && args.short == Some(false) {eprintln!("Please specify either --long or --short. Use --help to see more information.");}
    if args.long == Some(true) && args.short == Some(true) {eprintln!("Please specify only one: --long or --short. Use --help to see more information.");}

    if args.input == Some(false) && args.output == Some(false) {eprintln!("Please specify either --input or --output. Use --help to see more information.");}
    if args.input == Some(true) && args.output == Some(true) {eprintln!("Please specify only one: --input or --output. Use --help to see more information.");}

    let contract = contracts::get_clearing_house().await;
    let mut base_symbol: String = String::new();
    let token_addresses = address_list::get_token_addresses().await;
    let mut _direction = String::new();

    let open_position_params = OpenPositionParams {
        base_token: args.token.parse::<Address>().unwrap(),
        is_base_to_quote: if args.short == Some(true) {true} else {false},
        is_exact_input: if args.input == Some(true) {true} else {false},
        amount: ethers::utils::parse_units(args.amount, "ether").unwrap().into(),
        opposite_amount_bound: U256::zero(),
        deadline: U256::max_value(),
        sqrt_price_limit_x96: U256::zero(),
        referral_code: H256::zero().to_fixed_bytes(),
    };

    if open_position_params.is_base_to_quote == false {_direction = "LONG".to_string();} else {_direction = "SHORT".to_string();}
    for (key, val) in token_addresses {
        if val != open_position_params.base_token {continue;}
        base_symbol = key.parse::<String>().unwrap();
        break;
        }

    let tx = contract.open_position(open_position_params).send().await.expect("Failed").await.expect("Failed");
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
    println!("== New {} on {} ==", _direction, base_symbol);
    println!("========================");
    println!("");
    println!("Transaction: {:#?}", tx_receipt.transaction_hash);
    println!("Position Size: {} {}", position_size_float, base_symbol);
    println!("Avg Price: {} USD", avg_price.abs());
    println!("Fee Paid: {} USD", ethers::utils::format_units(logs[0].fee, "ether").unwrap().parse::<f64>().unwrap());
    println!("");
    
}