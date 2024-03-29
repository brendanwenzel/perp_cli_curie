use crate::prelude::OpenPositionParams;
use crate::{address_list, args::OpenCommand, contracts};
use ethers::prelude::*;
use eyre::Result;
use rust_decimal::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, EthEvent, Serialize)]
struct PositionChanged {
    #[ethevent(indexed)]
    trader: Address,
    #[ethevent(indexed)]
    #[serde(rename = "baseToken")]
    base_token: Address,
    #[serde(rename = "exchangedPositionSize")]
    exchanged_position_size: I256,
    #[serde(rename = "exchangedPositionNotional")]
    exchanged_position_notional: I256,
    fee: U256,
    #[serde(rename = "openNotional")]
    open_notional: I256,
    #[serde(rename = "realizedPnl")]
    realized_pnl: I256,
    #[serde(rename = "sqrtPriceAfterX96")]
    sqrt_price_after_x96: U256,
}

/// The function to process the Open command
pub async fn process(args: OpenCommand) -> Result<()> {
    if args.long == Some(false) && args.short == Some(false) {
        panic!("Please specify either --long or --short. Use --help to see more information.");
    }
    if args.long == Some(true) && args.short == Some(true) {
        panic!("Please specify only one: --long or --short. Use --help to see more information.");
    }

    if args.input == Some(false) && args.output == Some(false) {
        panic!("Please specify either --input or --output. Use --help to see more information.");
    }
    if args.input == Some(true) && args.output == Some(true) {
        panic!("Please specify only one: --input or --output. Use --help to see more information.");
    }

    let contract = contracts::get_clearing_house().await?;
    let mut base_symbol: String = String::new();
    let token_addresses = address_list::get_token_addresses().await?;

    let mut base_token_address = if args.token.len() == 42 {
        args.token.parse::<Address>()?
    } else {
        Address::zero()
    };

    if args.token.len() < 41 {
        for (key, val) in token_addresses.clone() {
            let mut chars = key.chars();
            chars.next();
            let key_without_v = chars.as_str();
            if key_without_v == args.token {
                base_token_address = val;
                break;
            }
            if key != args.token {
                continue;
            }
            base_token_address = val;
            break;
        }
    }

    let mut limit_sqrt = U256::zero();

    if let Some(limit) = args.limit {
        limit_sqrt = price_to_sqrt(limit)?;
    }

    let open_position_params = OpenPositionParams {
        base_token: base_token_address,
        is_base_to_quote: args.short == Some(true),
        is_exact_input: args.input == Some(true),
        amount: ethers::utils::parse_units(args.order_amount, "ether")?.into(),
        opposite_amount_bound: U256::zero(),
        deadline: U256::max_value(),
        sqrt_price_limit_x96: limit_sqrt,
        referral_code: H256::zero().to_fixed_bytes(),
    };

    for (key, val) in token_addresses {
        if val != open_position_params.base_token {
            continue;
        }
        base_symbol = key.parse::<String>()?;
        break;
    }

    let tx = contract
        .open_position(open_position_params)
        .send()
        .await?
        .await?;
    let tx_receipt = tx.expect("Transaction receipt for opening a new position");
    let logs: Vec<PositionChanged> = contract
        .event()
        .from_block(tx_receipt.block_number.expect("Block Number from receipt"))
        .query()
        .await?;

    let position_size =
        ethers::utils::format_units(logs[0].exchanged_position_size, "ether")?.parse::<f64>()?;
    let position_notional =
        ethers::utils::format_units(logs[0].exchanged_position_notional, "ether")?
            .parse::<f64>()?;
    let avg_price = position_notional / position_size;

    println!();
    println!("========================");
    println!(
        "== New {} on {} ==",
        if position_size > 0.0 { "LONG" } else { "SHORT" },
        base_symbol
    );
    println!("========================");
    println!();
    println!("Transaction: {:#?}", tx_receipt.transaction_hash);
    println!("Position Size: {} {}", position_size, base_symbol);
    println!("Avg Price: {} USD", avg_price.abs());
    println!(
        "Fee Paid: {} USD",
        ethers::utils::format_units(logs[0].fee, "ether")?.parse::<f64>()?
    );
    println!();
    Ok(())
}

fn price_to_sqrt(price: f64) -> Result<U256> {
    let dec_price = Decimal::from_f64_retain(price).unwrap();
    let sqrt_x96: Decimal = dec_price.sqrt().unwrap();
    let q96 = ethers::utils::format_units(U256::from(2).pow(U256::from(96)), 18)?.parse::<f64>()?;
    let dec_q96 = Decimal::from_f64_retain(q96).unwrap();
    let limit_sqrt = ethers::utils::parse_ether(sqrt_x96 * dec_q96)?;
    Ok(limit_sqrt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_to_sqrt() -> Result<()> {
        let price = 13.357711719432539;
        let limit_sqrt = price_to_sqrt(price)?;
        assert_eq!(
            limit_sqrt,
            U256::from(289564699876979139832120809390 as u128)
        );
        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn test_input_and_output_true() {
        let arg = OpenCommand {
            long: Some(false),
            short: Some(true),
            token: String::from("BNB"),
            input: Some(true),
            output: Some(true),
            order_amount: 5.12423,
            limit: None,
        };
        process(arg).await.expect("Failed process");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_long_and_short_true() {
        let arg = OpenCommand {
            long: Some(true),
            short: Some(true),
            token: String::from("BNB"),
            input: Some(false),
            output: Some(true),
            order_amount: 5.12423,
            limit: None,
        };
        process(arg).await.expect("Failed process");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_long_and_short_false() {
        let arg = OpenCommand {
            long: Some(false),
            short: Some(false),
            token: String::from("BNB"),
            input: Some(false),
            output: Some(true),
            order_amount: 5.12423,
            limit: None,
        };
        process(arg).await.expect("Failed process");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_input_and_output_false() {
        let arg = OpenCommand {
            long: Some(false),
            short: Some(true),
            token: String::from("BNB"),
            input: Some(false),
            output: Some(false),
            order_amount: 5.12423,
            limit: None,
        };
        process(arg).await.expect("Failed process");
    }
}
