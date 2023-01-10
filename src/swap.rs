use ethers::prelude::*;
use crate::{prelude::SwapCommand, address_list, utils, contracts::{velodrome_contract::Route, get_token_contract}};
use eyre::Result;
use crate::contracts;

/// This is the main function to process the swaps through Velodrome
pub async fn process(args: SwapCommand) -> Result<()> {
    let provider = utils::get_http_provider()?;
    let client = utils::create_http_client()?;
    let velodrome = contracts::get_velodrome_contract()?;
    let token_out_contract = get_token_contract(args.token_out)?;
    let token_in_contract = get_token_contract(args.token_in)?;
    let path = Route { from: args.token_in, to: args.token_out, stable: false };
    let routes: Vec<Route> = vec![path];
    let to = utils::get_wallet()?.address();
    let block_number = provider.get_block_number().await?;
    let block = provider.get_block(block_number).await?;
    let mut deadline = U256::from(4000000000 as u64);

    let token_in_decimals = token_in_contract
       .decimals()
       .call()
       .await?;
    let amount_in = U256::from(ethers::utils::parse_units(args.amount_in, token_in_decimals as u32)?);

    if let Some(block_returned) = block {
        let block_timestamp = block_returned.timestamp;
        deadline = block_timestamp.checked_add(U256::from(120)).expect("Custom Deadline");
    }
    
    let expected_out = velodrome
       .get_amount_out(amount_in, args.token_in, args.token_out)
       .call()
       .await?;

    let output_decimals = token_out_contract
       .decimals()
       .call()
       .await?;

    let decimals = output_decimals as u32;

    let slippage = args.slippage / 100.0;
    let slippage_amount = ethers::utils::format_units(expected_out.0, decimals)?.parse::<f64>()? * slippage;
    let amount_out_min = expected_out.0.checked_sub(ethers::utils::parse_units(slippage_amount, decimals)?.into()).expect("Amount Out");

    if args.eth == Some(false) {
        
        let allowance = token_in_contract
           .allowance(to, velodrome.address())
           .call()
           .await?;
        if allowance < amount_in {
            println!("Not enough allowance to swap this token. Submitting approval now.");
            let approval = token_in_contract
                .approve(velodrome.address(), amount_in)
                .send()
                .await?
                .await?
                .expect("Approval Request");
                println!("Approval succeeded and now sending swap request");
                println!("Approval Transaction Hash: {:#?}", approval.transaction_hash);
        } else {
            println!("Router has been approved and sending swap request")
        }

        let data = velodrome
           .swap_exact_tokens_for_tokens(amount_in, amount_out_min, routes, to, deadline)
           .calldata()
           .expect("Swap Exact Tokens for Tokens");
        let tx = TransactionRequest::new().to(velodrome.address()).data(data);
        let pending_tx = client.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?.expect("Swap Tokens for Tokens");
        println!("Swap Successful! Transaction: {:#?}", receipt.transaction_hash);
        
        //    println!("Successfully swapped {} {} for {} {}\nTransaction: {:#?}", args.amount_in tx.transaction_hash);
    } else {
        let data = velodrome
            .swap_exact_eth_for_tokens(amount_out_min, routes, to, deadline)
            .calldata()
            .expect("Swap ETH for Tokens");
        let tx = TransactionRequest::new().to(address_list::get_velodrome()?).data(data).value(amount_in);
        let pending_tx = client.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?.expect("Swap ETH for Tokens");
        println!("{:#?}", receipt.transaction_hash);
    }

    Ok(())
}