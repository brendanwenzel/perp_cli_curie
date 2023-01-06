use crate::args::AmmCommand;
use crate::{contracts, address_list};
use ethers::prelude::*;
use eyre::Result;
use crate::prelude::Pools;

/// Processing theh AMM Command
#[tokio::main]
pub async fn process(args: AmmCommand) -> Result<()> {
    let pools = address_list::get_pools().await?;
    let pools_iter = pools.iter();

    async fn print_amm(pool: &Pools) -> Result<()> {
        println!("========================");
        println!("=====  {}/{}  =====", pool.base_symbol, pool.quote_symbol);
        println!("========================");

        let contract = contracts::get_base_contract(pool.base_address.parse::<Address>()?)?;
        let quote_contract = contracts::get_base_contract(pool.quote_address.parse::<Address>()?)?;
        let pool_contract = contracts::get_pool_contract(pool.address.parse::<Address>()?)?;

        let index_price = contract
            .get_index_price(U256::zero())
            .call()
            .await?;
        let format_index_price = ethers::utils::format_units(index_price, 18)?.parse::<f64>()?;

        let price_feed = contract
            .get_price_feed()
            .call()
            .await?;

        let base_asset_reserve = contract
            .balance_of(pool.address.parse::<Address>()?)
            .call()
            .await?;

        let quote_asset_reserve = quote_contract
            .balance_of(pool.address.parse::<Address>()?)
            .call()
            .await?;

        let sqrt_price = (pool_contract
           .slot_0()
           .call()
           .await?).0;
           
        let sqrt_price_float = ethers::utils::format_units(sqrt_price.pow(U256::from(2)), 18)?.parse::<f64>()?;
        let q96 = ethers::utils::format_units(U256::from(2).pow(U256::from(192)), 18)?.parse::<f64>()?;
        let market_price = sqrt_price_float / q96;

        println!("- Pool Address: {}", pool.address);
        println!("- Index Price: {}", format_index_price);
        println!("- Market Price: {}", market_price);
        // println!("- OpenInterestNotionalCap: {}", open_interest_notional_cap);
        // println!("- OpenInterestNotional: {}", open_interest_notional);
        // println!("- MaxHoldingBaseAsset: {}", max_holding_base_asset);
        println!("- {} Reserves: {}", pool.base_symbol, ethers::utils::format_units(base_asset_reserve, "ether")?.parse::<f64>()?);
        println!("- {} Reserves: {}", pool.quote_symbol, ethers::utils::format_units(quote_asset_reserve, "ether")?.parse::<f64>()?);
        println!("- Price Feed: {:?}", price_feed);
        Ok(())
    }

    if args.search_parameter == None && args.short == Some(false) {
        println!("");
        for pool in pools_iter.clone() {
            let print_result = print_amm(pool).await;
            match print_result {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
        }
        println!("");
    }

    match args.search_parameter {
        Some(value) => {
            println!("");
        for pool in pools_iter.clone() {
            if pool.address != value && pool.base_address != value && pool.base_symbol != value {continue;}
            print_amm(pool).await?;
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
    Ok(())
}