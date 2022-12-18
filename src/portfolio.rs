use ethers::prelude::*;
use crate::{args::PortfolioCommand, address_list, contracts, utils};
use std::ops::Div;

#[tokio::main]
/// Primary function to process portfolio command
pub async fn process(args: PortfolioCommand) {

    let http_provider = utils::get_http_provider().expect("Failed");
    let perp_portal_contract = contracts::get_perp_portal();
    let account_balance_contract = contracts::get_account_balance().await;
    let vault_contract = contracts::get_vault().await;
    let clearing_house = contracts::get_clearing_house().await;
    let mut trader = utils::get_wallet().unwrap().address();

    match args.trader_address {
        Some(trader_add) => {trader = trader_add.parse::<Address>().expect("failed to make address");}
        None => {}    
    }

    let total_account_value: I256 = clearing_house
       .get_account_value(trader)
       .call()
       .await
       .expect("Failed to Get Account Value");

    let free_collateral_value: U256 = vault_contract
       .get_free_collateral(trader)
       .call()
       .await
       .expect("Failed to Pull Free Collateral Value");

    let pnl_and_pending_fee: (I256, I256, U256) = account_balance_contract
        .get_pnl_and_pending_fee(trader)
        .call()
        .await
        .expect("Failed to get PNL");
    
    let total_value = ethers::utils::format_units(total_account_value, "ether").expect("Couldn't Format").parse::<f64>().unwrap();
    let trader_balance = http_provider.get_balance(trader, None).await.expect("Failed to get balance");
    let free_collateral = ethers::utils::format_units(free_collateral_value, 6).expect("Couldn't Format").parse::<f64>().unwrap();
    let token_addresses = address_list::get_token_addresses().await;

    println!("");
    println!("Trader Address: {:?}", trader);
    println!("Account Value: {} USD", total_value);
    println!("Owed Realized PnL: {} USD", ethers::utils::format_units(pnl_and_pending_fee.0, "ether").unwrap().parse::<f64>().unwrap());
    println!("Unrealized PnL: {} USD", ethers::utils::format_units(pnl_and_pending_fee.1, "ether").unwrap().parse::<f64>().unwrap());
    println!("");
    println!("Available Balances");
    println!("==================");
    println!("- OP ETH: {}", ethers::utils::format_units(trader_balance,"ether").expect("format failed").parse::<f64>().unwrap());
    println!("- Total Free Collateral: {} USD", free_collateral);
    println!("");

    for (key, val) in token_addresses {
        let base_contract = contracts::get_base_contract(val);

        let total_position_size = account_balance_contract
           .get_total_position_size(trader, val)
           .call()
           .await
           .unwrap();
        
           if total_position_size == I256::zero() {continue;}
        
        let taker_position_size = account_balance_contract
            .get_taker_position_size(trader, val)
            .call()
            .await
            .unwrap();

        let taker_open_notional = account_balance_contract
            .get_taker_open_notional(trader, val)
            .call()
            .await
            .expect("Failed to get taker open notional value");

        let total_open_notional = account_balance_contract
            .get_total_open_notional(trader, val)
            .call()
            .await
            .unwrap();

        let index_price = base_contract
            .get_index_price(U256::zero())
            .call()
            .await
            .unwrap();

        let liquidation_price: U256 = perp_portal_contract
            .get_liquidation_price(trader, val)
            .call()
            .await
            .unwrap();

        let pending_fee = perp_portal_contract
            .get_total_token_amount_in_pool_and_pending_fee(trader, val, false)
            .call()
            .await
            .expect("Failed to get pending fee");

        let format_total_position_size = ethers::utils::format_units(total_position_size, "ether").unwrap().parse::<f64>().unwrap();
        let tps_float = ethers::utils::format_units(taker_position_size, "ether").unwrap().parse::<f64>().unwrap();
        let maker_position_size = format_total_position_size - tps_float;
        let ton_float = ethers::utils::format_units(taker_open_notional, "ether").unwrap().parse::<f64>().unwrap();
        let total_open_notional_float = ethers::utils::format_units(total_open_notional, "ether").unwrap().parse::<f64>().unwrap();
        let maker_open_notional = total_open_notional_float - ton_float;    
        let index_float = ethers::utils::format_units(index_price, "ether").unwrap().parse::<f64>().unwrap();
        let format_liquidation_price = ethers::utils::format_units(liquidation_price, "ether").unwrap().parse::<f64>().unwrap();
        let pending_fee_value = ethers::utils::format_units(pending_fee.1, "ether").unwrap().parse::<f64>().unwrap();
        let token_amount_in_pool = ethers::utils::format_units(pending_fee.0, "ether").unwrap().parse::<f64>().unwrap();

        let taker_unrealized_pnl = 
            if tps_float < 0.0 {
                -tps_float * index_float - ton_float
            } else {
                tps_float * index_float + ton_float
            };

        let maker_unrealized_profit = 
            if maker_position_size < 0.0 {
                -maker_position_size * index_float - maker_open_notional
            } else {
                maker_position_size * index_float + maker_open_notional
            };

        let mut entry_price = 0.0;
            if taker_open_notional != I256::zero() && taker_position_size != I256::zero() {
                entry_price = ton_float.div(tps_float).abs();
            }

        let position_value = ton_float + maker_unrealized_profit + taker_unrealized_pnl;

        println!("========================");
        println!("========  {}  ========", key);
        println!("========================");
        println!("");
        println!("Index Price: {}", index_float);
        println!("");
        if tps_float != 0.0 || taker_unrealized_pnl != 0.0 {
        println!("*** Taker ***");
        println!("- Position Size: {}", tps_float);
        println!("- Avg Entry Price: {} USD", entry_price);
        println!("- Open Notional: {}", ton_float);
        println!("- Unrealized PnL: {}", taker_unrealized_pnl);
        println!("- Liquidation Price: {}", format_liquidation_price);
        println!("");
        }
        if maker_position_size != 0.0 || maker_unrealized_profit != 0.0 {
        println!("*** Maker ***");
        println!("- Position Size: {}", maker_position_size);
        println!("- Position Value: {} USD", token_amount_in_pool);
        println!("- Unrealized PnL: {}", maker_unrealized_profit);
        println!("- Pending Fees: {}", pending_fee_value);
        println!("- Open Notional: {}", maker_open_notional);
        println!("");
        }
        if tps_float != 0.0 && maker_position_size != 0.0 {
        println!("*** Total ***");
        println!("- Position: {}", format_total_position_size);
        println!("- Open Notional: {}", total_open_notional_float);
        println!("- Position Value (USD): {}", position_value);
        println!("");
        }
    }
}