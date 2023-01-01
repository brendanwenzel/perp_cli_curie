use ethers::prelude::*;
use crate::{address_list, contracts, args::OpenCommand};
use crate::prelude::OpenPositionParams;
use serde::Serialize;
use eyre::Result;

#[tokio::main]
/// The function to process the Open command
pub async fn process(args: OpenCommand) -> Result<()> {

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

    let mut base_token_address = if args.token.len() == 42 { args.token.parse::<Address>()? } else { Address::zero() };

    if args.token.len() < 41 {
        for (key, val) in token_addresses.clone() {
            let mut chars = key.chars();
            chars.next();
            let key_without_v = chars.as_str();
            if key_without_v == args.token { base_token_address = val; break; }
            if key != args.token { continue; }
            base_token_address = val;
            break;
        }
    }

    let mut limit_sqrt = U256::zero();

    match args.limit {
        Some(limit) => {
            let sqrt_x96 = limit.sqrt();
            let q96 = ethers::utils::format_units(U256::from(2).pow(U256::from(96)), 18)?.parse::<f64>()?;
            limit_sqrt = ethers::utils::parse_ether(sqrt_x96 * q96)?;
        },
        None => { }
    }

    let open_position_params = OpenPositionParams {
        base_token: base_token_address,
        is_base_to_quote: if args.short == Some(true) {true} else {false},
        is_exact_input: if args.input == Some(true) {true} else {false},
        amount: ethers::utils::parse_units(args.order_amount, "ether").unwrap().into(),
        opposite_amount_bound: U256::zero(),
        deadline: U256::max_value(),
        sqrt_price_limit_x96: limit_sqrt,
        referral_code: H256::zero().to_fixed_bytes(),
    };

    for (key, val) in token_addresses {
        if val != open_position_params.base_token {continue;}
        base_symbol = key.parse::<String>()?;
        break;
        }

    let tx = contract.open_position(open_position_params).send().await?.await?;
    let tx_receipt = tx.unwrap();
    let logs: Vec<PositionChanged> = contract
        .event()
        .from_block(tx_receipt.block_number.unwrap())
        .query()
        .await?;

    let position_size = ethers::utils::format_units(logs[0].exchanged_position_size, "ether")?.parse::<f64>()?;
    let position_notional = ethers::utils::format_units(logs[0].exchanged_position_notional, "ether")?.parse::<f64>()?;
    let avg_price = position_notional / position_size;

    println!("");
    println!("========================");
    println!("== New {} on {} ==", if position_size > 0.0 {"LONG"} else {"SHORT"}, base_symbol);
    println!("========================");
    println!("");
    println!("Transaction: {:#?}", tx_receipt.transaction_hash);
    println!("Position Size: {} {}", position_size, base_symbol);
    println!("Avg Price: {} USD", avg_price.abs());
    println!("Fee Paid: {} USD", ethers::utils::format_units(logs[0].fee, "ether")?.parse::<f64>()?);
    println!("");
    Ok(())
}