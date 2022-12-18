use crate::{args::CloseCommand, address_list, prelude::ClosePositionParams, contracts};
use serde::Serialize;
use ethers::prelude::*;
use eyre::Result;

#[tokio::main]
/// The function to process the Close command
pub async fn process(args: CloseCommand) -> Result<()> {

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

    let contract = contracts::get_clearing_house().await;
    let mut base_symbol: String = String::new();
    let token_addresses = address_list::get_token_addresses().await;
    let mut _direction = String::new();
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

    let close_position_params = ClosePositionParams {
        base_token: base_token_address,
        sqrt_price_limit_x96: U256::zero(),
        opposite_amount_bound: U256::zero(),
        deadline: U256::max_value(),
        referral_code: H256::zero().to_fixed_bytes(),
    };

    for (key, val) in token_addresses {
        if val != close_position_params.base_token {continue;}
        base_symbol = key.parse::<String>()?;
        break;
        }

    let tx = contract.close_position(close_position_params).send().await?.await?;
    let tx_receipt = tx.unwrap();
    let logs: Vec<PositionChanged> = contract
        .event()
        .from_block(tx_receipt.block_number.unwrap())
        .query()
        .await?;

    let position_size = ethers::utils::format_units(logs[0].exchanged_position_size, "ether")?;
    let position_size_float = position_size.parse::<f64>()?;

    let position_notional = ethers::utils::format_units(logs[0].exchanged_position_notional, "ether")?;
    let position_notional_float = position_notional.parse::<f64>()?;

    let avg_price = position_notional_float / position_size_float;

    println!("");
    println!("========================");
    println!("==== CLOSING {} ====", base_symbol);
    println!("========================");
    println!("");
    println!("Transaction: {:#?}", tx_receipt.transaction_hash);
    println!("Position Size: {} {}", position_size_float, base_symbol);
    println!("Avg Price: {} USD", avg_price.abs());
    println!("Fee Paid: {} USD", ethers::utils::format_units(logs[0].fee, "ether")?.parse::<f64>()?);
    println!("Profit: {} USD", ethers::utils::format_units(logs[0].realized_pnl, "ether")?.parse::<f64>()?);
    println!("");
    println!("");
    Ok(())
}