use crate::{args::OpenCommand, contracts, utils};
use serde::Serialize;
use ethers::prelude::*;

#[tokio::main]
/// The function to process the Open command
pub async fn process(args: OpenCommand) {
    #[derive(Clone, Debug, Serialize)]
    struct OpenPositionParams {
        #[serde(rename="baseToken")]
        base_token: Address,
        #[serde(rename="isBaseToQuote")]
        is_base_to_quote: bool,
        #[serde(rename="isExactInput")]
        is_exact_input: bool,
        amount: U256,
        #[serde(rename="oppositeAmountBound")]
        opposite_amount_bound: U256,
        deadline: U256,
        #[serde(rename="sqrtPriceLimitX96")]
        sqrt_price_limit_x96: U256,
        #[serde(rename="referralCode")]
        referral_code: Bytes,
    }

    #[derive(Clone, Debug, EthEvent, Serialize)]
    struct OpenPosition {
        base: U256,
        quote: U256,
    }

    let http_provider = utils::get_http_provider().expect("Failed");
    let client = utils::create_http_client(http_provider.clone()).expect("Failed");
    let clearing_house_contract = contracts::get_clearing_house(&client);
    let zero_address = String::from("0x0000000000000000000000000000000000000000").parse::<Address>().unwrap();

    let mut open_position_params = OpenPositionParams {
        base_token: zero_address,
        is_base_to_quote: false,
        is_exact_input: false,
        amount: U256::zero(),
        opposite_amount_bound: U256::zero(),
        deadline: U256::max_value(),
        sqrt_price_limit_x96: U256::zero(),
        referral_code: Bytes::from(H256::zero().to_fixed_bytes()),
    };

    let tx = clearing_house_contract
        .method::<_, OpenPosition>("openPosition", (
            open_position_params.base_token, 
            open_position_params.is_base_to_quote, 
            open_position_params.is_exact_input,
            open_position_params.amount, 
            open_position_params.opposite_amount_bound, 
            open_position_params.deadline, 
            open_position_params.sqrt_price_limit_x96, 
            open_position_params.referral_code
        )
    )
        .expect("Failed to send method")
        .send()
        .await
        .expect("Failed")
        .await
        .expect("Failed")
        .unwrap();
        

    println!("Transaction Complete: {:?}", tx.transaction_hash);

}