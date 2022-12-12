use crate::args::PositionCommand;
use crate::{address_list, contracts, utils};
use ethers::{prelude::*, abi::RawLog};
use serde::Serialize;

/// function to process the position command
#[tokio::main]
pub async fn process(args: PositionCommand) {

    /// struct to hold the necessary variables
    #[derive(Debug)]
    struct Variables {
        trader: Address,
        base_token: Address,
        block_limit: u64,
        hash: H256
    }

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

    // Connect to Provider and Create Client
    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider.clone(), 10).expect("Failed");
    let _clearing_house_contract = contracts::get_clearing_house(&client);
    let zero_address = String::from("0x0000000000000000000000000000000000000000").parse::<Address>().unwrap();
    let mut variables = Variables {
        trader: zero_address,
        base_token: zero_address,
        block_limit: 0 as u64,
        hash: H256::zero(),
    };

    match args.trader {
        Some(address) => variables.trader = String::from(address).parse::<Address>().unwrap(),
        None => {},
    }
    match args.base_token {
        Some(address) => variables.base_token = address.parse::<Address>().unwrap(),
        None => {},
    }
    match args.limit {
        Some(block_limit) => variables.block_limit = block_limit as u64,
        None => {variables.block_limit = 250},
    }

    let block_number = http_provider.get_block_number().await.expect("Failed to Get Block Number");
    let target_block = block_number - variables.block_limit;

    let filter = Filter::new().select(target_block..).address(address_list::get_clearing_house().parse::<Address>().unwrap()).topic0("0x968bc4f738eae0486dc6736c4b427dbafa4acfdf6eaf223337791ddeb3a56247".parse::<H256>().unwrap());
    let logs = client
        .get_logs(&filter)
        .await
        .expect("failted");

    for log in logs {
        let event = <PositionChanged as EthLogDecode>::decode_log(&RawLog { topics: log.topics, data: log.data.to_vec() }).unwrap();
        if variables.trader != zero_address && variables.trader != event.trader { continue; }
        if variables.base_token != zero_address && variables.base_token != event.base_token { continue; }
        let mut base_symbol: String = String::new();
        let token_addresses = address_list::get_token_addresses();
        for (key, val) in token_addresses {
        if val != event.base_token {continue;}
        base_symbol = key.parse::<String>().unwrap();
        break;
        }

        let position_size = ethers::utils::format_units(event.exchanged_position_size, "ether").unwrap();
        let postion_float = position_size.parse::<f64>().unwrap();
        let open_notional = ethers::utils::format_units(event.exchanged_position_notional, "ether").unwrap();
        let notional_float = open_notional.parse::<f64>().unwrap();
        if postion_float < 0.000000000000000002 && postion_float > -0.000000000000000002 {continue;}
        let price = notional_float / postion_float;

        match log.transaction_hash {
            Some(transaction_hash) => {
                variables.hash = transaction_hash;
            },
            None => {},
        }

        println!("");
        println!("=====================");
        println!("==== {}: {} ====", if event.exchanged_position_size < I256::zero() {String::from("SHORT")} else {" LONG".to_string()}, base_symbol);
        println!("=====================");
        println!("- Trader: {:?}", event.trader);
        println!("- Price: {}", price.abs()); // Need to fix this
        println!("- Size: {}", position_size);
        println!("- Tx: {:?}", variables.hash); // Need to fix this
    }  

}