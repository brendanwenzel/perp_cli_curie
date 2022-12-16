use ethers::prelude::*;
use crate::args::PortfolioCommand;
use crate::{contracts, address_list, utils};
use std::ops::Div;
use serde::Serialize;

#[tokio::main]
/// Primary function to process portfolio command
pub async fn process(args: PortfolioCommand) {

    #[derive(Clone, EthEvent, Serialize, Debug)]
    struct PositionChanged {
        trader: Address,
        #[serde(rename="baseToken")]
        base_token: Address,
        #[serde(rename="exchangedPositionSize")]
        exchanged_position_size: I256,
        #[serde(rename="exchangedPositionNotional")]
        exchanged_position_notional: bool,
        fee: U256,
        #[serde(rename="openNotional")]
        open_notional: I256,
        #[serde(rename="realizedPnl")]
        realized_pnl: I256,
        #[serde(rename="sqrtPriceAfterX96")]
        sqrt_price_after_x96: U256,
    }

    #[derive(Clone, EthEvent, Serialize, Debug)]
    struct PnlAndPendingFee {
        #[serde(rename="owedRealizedPnl")]
        owed_realized_pnl: I256,
        #[serde(rename="unrealizedPnl")]
        unrealized_pnl: I256,
        #[serde(rename="pendingFee")]
        pending_fee: U256,
    }

    #[derive(Clone, EthEvent, Serialize, Debug)]
    struct TokenAmountPendingFee {
        #[serde(rename="tokenAmount")]
        token_amount: U256,
        #[serde(rename="totalPendingFee")]
        total_pending_fee: U256,
    }

    // Connect to Provider and Create Client
    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider.clone()).expect("Failed");

    // Connecting Contracts
    let perp_portal_contract = contracts::get_perp_portal(&client);
    let account_balance_contract = contracts::get_account_balance(&client);
    let vault_contract = contracts::get_vault(&client);

    let trader = args.trader_address;
    let trader_address = trader.clone().parse::<Address>().expect("failed to make address");
    let trader_balance = http_provider.get_balance(trader_address, None).await.expect("Failed to get balance");

    let total_account_value: I256 = vault_contract
        .method::<_, I256>("getSettlementTokenValue", trader_address)
        .expect("Invalid Address")
        .call()
        .await
        .expect("Failed to Pull Account Value");
    let total_value = ethers::utils::format_units(total_account_value, 6).expect("Couldn't Format");

    let free_collateral_value: U256 = vault_contract
       .method::<_, _>("getFreeCollateral", trader_address)
       .expect("Invalid Address")
       .call()
       .await
       .expect("Failed to Pull Free Collateral Value");
    let free_collateral = ethers::utils::format_units(free_collateral_value, 6).expect("Couldn't Format");

    let pnl_and_pending_fee: PnlAndPendingFee = account_balance_contract
    .method::<_, PnlAndPendingFee>("getPnlAndPendingFee", trader_address)
    .expect("Failed to get PNL")
    .call()
    .await
    .expect("Failed to get PNL");

    println!("");
    println!("Trader Address: {}", trader);
    println!("Account Value: {} USD", total_value);
    println!("Unrealized PnL: {} USD", ethers::utils::format_units(pnl_and_pending_fee.unrealized_pnl, "ether").expect("Couldn't Format"));
    println!("");
    println!("Available Balances");
    println!("==================");
    println!("- OP ETH: {}", ethers::utils::format_units(trader_balance,"ether").expect("format failed"));
    println!("- Free Collateral: {} USD", free_collateral);
    println!("");

    let token_addresses = address_list::get_token_addresses();
    for (key, val) in token_addresses {
        let base_contract = contracts::get_base_contract(&client, val);
        let total_position_size = account_balance_contract
            .method::<_, I256>("getTotalPositionSize", (trader_address, val))
            .expect("Failed to Get Total Position Size")
            .call()
            .await
            .expect("Failed to Get Total Position Size");
        if total_position_size == I256::zero() {continue;}
        let format_total_position_size = ethers::utils::format_units(total_position_size, "ether").unwrap();
        
        let taker_position_size = account_balance_contract
            .method::<_, I256>("getTakerPositionSize", (trader_address, val))
            .expect("Failed to Get Position Size")
            .call()
            .await
            .expect("Failed to Get Position Size");
        let format_taker_position_size = ethers::utils::format_units(taker_position_size, "ether").unwrap();
        let tps_float = format_taker_position_size.parse::<f64>().unwrap();
        
        let maker_position_size = total_position_size - taker_position_size;
        let format_maker_position_size = ethers::utils::format_units(maker_position_size, "ether").unwrap();
        let mps_float = format_maker_position_size.parse::<f64>().unwrap();

        let taker_open_notional = account_balance_contract
            .method::<_, I256>("getTakerOpenNotional", (trader_address, val))
            .expect("Invalid Address")
            .call()
            .await
            .expect("Failed to get taker open notional value");

        let taker_open_notional_value = ethers::utils::format_units(taker_open_notional, "ether").unwrap();
        let ton_float = taker_open_notional_value.parse::<f64>().unwrap();

        let total_open_notional = account_balance_contract
            .method::<_, I256>("getTotalOpenNotional", (trader_address, val))
            .expect("Invalid Address")
            .call()
            .await
            .expect("Failed to get total open notional value");
        let format_total_open_notional = ethers::utils::format_units(total_open_notional, "ether").unwrap();
        let total_ton_float = format_total_open_notional.parse::<f64>().unwrap();

        let maker_open_notional = total_ton_float - ton_float;

        // pnl: -0.312628163013417874
        let index_price = base_contract
            .method::<_, U256>("getIndexPrice", U256::zero())
            .expect("Invalid Address")
            .call()
            .await
            .expect("Failed to Pull Account Value");
        let format_index_price = ethers::utils::format_units(index_price, "ether").unwrap();
        let index_float = format_index_price.parse::<f64>().unwrap();

        let taker_unrealized_pnl = 
            if tps_float < 0.0 {
                tps_float * index_float + ton_float
            } else {
                tps_float * index_float - ton_float
            };

        let maker_unrealized_profit = 
            if mps_float < 0.0 {
                mps_float * index_float + maker_open_notional
            } else {
                mps_float * index_float - maker_open_notional
            };

        // liq. price: 0.042020407273170428
        let liquidation_price: U256 = perp_portal_contract
           .method::<_, U256>("getLiquidationPrice", (trader_address, val))
           .expect("Failed to send getLiquidationPrice")
           .call()
           .await
           .expect("Failed to receieve Liquidation Price");
        let format_liquidation_price = ethers::utils::format_units(liquidation_price, "ether").unwrap();

        // entry price
        let mut entry_price = 0.0;
        if taker_open_notional != I256::zero() && taker_position_size != I256::zero() {
        entry_price = ton_float.div(tps_float).abs();
        }

        let quote = account_balance_contract
           .method::<_, I256>("getQuote", (trader_address, val))
           .expect("Failed to get quote")
           .call()
           .await
           .expect("Failed to call method");

        let quote_value = ethers::utils::format_units(quote, "ether").unwrap();
        let quote_float = quote_value.parse::<f64>().unwrap();

        let leverage = ton_float.div(quote_float).abs();

        let position_value = ton_float + maker_unrealized_profit + taker_unrealized_pnl;

        let pending_fee = perp_portal_contract
            .method::<_, TokenAmountPendingFee>("getTotalTokenAmountInPoolAndPendingFee", (trader_address, val, false))
            .expect("Failed to receieve pending_fee")
            .call()
            .await
            .expect("Failed to get pending fee");

        let pending_fee_value = ethers::utils::format_units(pending_fee.total_pending_fee, "ether").unwrap();
        let token_amount_in_pool = ethers::utils::format_units(pending_fee.token_amount, "ether").unwrap();

        println!("========================");
        println!("========  {}  ========", key);
        println!("========================");
        println!("Index Price: {}", format_index_price);
        println!("");
        if tps_float != 0.0 || taker_unrealized_pnl != 0.0 {
        println!("*** Taker ***");
        println!("- Position Size: {}", format_taker_position_size);
        println!("- Avg Entry Price: {} USD", entry_price);
        println!("- Open Notional: {}", taker_open_notional_value);
        println!("- Unrealized PnL: {}", taker_unrealized_pnl);
        println!("- Leverage: {}", leverage);
        println!("- Liquidation Price: {}", format_liquidation_price);
        println!("");
        }
        if mps_float != 0.0 || maker_unrealized_profit != 0.0 {
        println!("*** Maker ***");
        println!("- Position Size: {}", format_maker_position_size);
        println!("- Position Value: {} USD", token_amount_in_pool);
        println!("- Unrealized PnL: {}", maker_unrealized_profit);
        println!("- Pending Fees: {}", pending_fee_value);
        println!("- Open Notional: {}", maker_open_notional);
        println!("");
        }
        if tps_float != 0.0 && mps_float != 0.0 {
        println!("*** Total ***");
        println!("- Position: {}", format_total_position_size);
        println!("- Open Notional: {}", format_total_open_notional);
        println!("- Position Value (USD): {}", position_value);
        println!("");
        }
    }
}