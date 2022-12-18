use crate::args::AmmCommand;
use crate::{contracts, address_list};
use ethers::prelude::*;

/// Processing theh AMM Command
#[tokio::main]
pub async fn process(args: AmmCommand) {
    let pools = address_list::get_pools().await;
    let pools_iter = pools.iter();

    if args.search_parameter == None && args.short == Some(false) {
        println!("");
        for pool in pools_iter.clone() {
            let contract = contracts::get_base_contract(pool.base_address.parse::<Address>().expect("Failed to make Address"));
            let quote_contract = contracts::get_quote_contract(pool.quote_address.parse::<Address>().expect("Failed"));

            let index_price = contract
                .get_index_price(U256::zero())
                .call()
                .await
                .expect("Failed to Pull Account Value");
            let format_index_price = ethers::utils::format_units(index_price, 18).expect("Failed to Format Index Price");

            let price_feed = contract
               .get_price_feed()
               .call()
               .await
               .expect("Failed to get price feed");

            let base_asset_reserve = contract
               .balance_of(pool.address.parse::<Address>().unwrap())
               .call()
               .await
               .expect("Failed");

            let quote_asset_reserve = quote_contract
               .balance_of(pool.address.parse::<Address>().unwrap())
               .call()
               .await
               .expect("Failed");

            // let market_price = pool_contract
            //     .method::<_, Slot0>("slot0", ())
            //     .expect("Failed to send method")
            //     .call()
            //     .await
            //     .expect("Failed");

            println!("========================");
            println!("=====  {}/{}  =====", pool.base_symbol, pool.quote_symbol);
            println!("========================");
            println!("- Pool Address: {}", pool.address);
            println!("- Index Price: {}", format_index_price);
            // println!("- Market Price: {}", market_price);
            // println!("- OpenInterestNotionalCap: {}", open_interest_notional_cap);
            // println!("- OpenInterestNotional: {}", open_interest_notional);
            // println!("- MaxHoldingBaseAsset: {}", max_holding_base_asset);
            println!("- {} Reserves: {}", pool.base_symbol, ethers::utils::format_units(base_asset_reserve, "ether").unwrap());
            println!("- {} Reserves: {}", pool.quote_symbol, ethers::utils::format_units(quote_asset_reserve, "ether").unwrap());
            println!("- Price Feed: {:?}", price_feed);
        }
        println!("");
    }

    match args.search_parameter {
        Some(value) => {
            println!("");
        for pool in pools_iter.clone() {
            if pool.address != value && pool.base_address != value && pool.base_symbol != value {continue;}
            let contract = contracts::get_base_contract(pool.base_address.parse::<Address>().expect("Failed to make Address"));
            let quote_contract = contracts::get_base_contract(pool.quote_address.parse::<Address>().expect("Failed"));
            let index_price = contract
                .get_index_price(U256::zero())
                .call()
                .await
                .expect("Failed to Pull Account Value");
            let format_index_price = ethers::utils::format_units(index_price, 18).expect("Failed to Format Index Price");

            let price_feed = contract
               .get_price_feed()
               .call()
               .await
               .expect("Failed to get price feed");

            let base_asset_reserve = contract
               .balance_of(pool.address.parse::<Address>().unwrap())
               .call()
               .await
               .expect("Failed");

            let quote_asset_reserve = quote_contract
               .balance_of(pool.address.parse::<Address>().unwrap())
               .call()
               .await
               .expect("Failed");

            println!("========================");
            println!("=====  {}/{}  =====", pool.base_symbol, pool.quote_symbol);
            println!("========================");
            println!("- Pool Address: {}", pool.address);
            println!("- Index Price: {}", format_index_price);
            // println!("- Market Price: {}", market_price);
            // println!("- OpenInterestNotionalCap: {}", open_interest_notional_cap);
            // println!("- OpenInterestNotional: {}", open_interest_notional);
            // println!("- MaxHoldingBaseAsset: {}", max_holding_base_asset);
            println!("- {} Reserves: {}", pool.base_symbol, ethers::utils::format_units(base_asset_reserve, "ether").unwrap());
            println!("- {} Reserves: {}", pool.quote_symbol, ethers::utils::format_units(quote_asset_reserve, "ether").unwrap());
            println!("- Price Feed: {:?}", price_feed);
            break;
        }
        println!("");
    },
        None => {},
    }

    match args.short {
        Some(short) => if short {
            println!("");
            for pool in pools_iter {
                println!("- {}/{}: {}", pool.base_symbol, pool.quote_symbol, pool.address);
            }
            println!("");
        },
        None => {},
    }
}