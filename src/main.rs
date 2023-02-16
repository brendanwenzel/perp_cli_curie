use clap::Parser;
use eyre::Result;
use perpcli_rs::{
    amm,
    args::{PerpArgs, SubCommand::*},
    close, config, deposit, open, portfolio, position, quit, swap, tokens, withdraw,
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = PerpArgs::parse();
    match_args(args).await?;
    Ok(())
}

async fn match_args(args: PerpArgs) -> Result<()> {
    match args.cmd {
        Position(position_args) => position::process(position_args).await?,
        Portfolio(portfolio_args) => portfolio::process(portfolio_args).await?,
        Amm(amm_args) => amm::process(amm_args).await?,
        Quit(quit_args) => quit::process(quit_args).await?,
        Tokens(tokens_args) => tokens::process(tokens_args).await?,
        Deposit(deposit_args) => deposit::process(deposit_args).await?,
        Withdraw(withdraw_args) => withdraw::process(withdraw_args).await?,
        Open(open_args) => open::process(open_args).await?,
        Close(close_args) => close::process(close_args).await?,
        Swap(swap_args) => swap::process(swap_args).await?,
        Config(config_args) => {
            if config_args.rpc.expect("true/false") {
                config::change_rpc()?;
            }
            if config_args.chain.expect("true/fase") {
                config::change_chain_id()?;
            }
            if config_args.pk.expect("true/false") {
                config::change_pk()?;
            }
            if !config_args.pk.expect("true/false")
                && !config_args.chain.expect("true/fase")
                && !config_args.chain.expect("true/fase")
            {
                let config = config::get_config()?;
                println!("RPC URL: {}", config.rpc_url);
                println!("CHAIN ID: {}", config.chain_id);
                println!("KEY: {}", config.pk);
                println!("Config Path: {}", config::config_path()?.to_str().unwrap());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::prelude::*;
    use perpcli_rs::{
        address_list,
        args::{DepositCommand, OpenCommand, SwapCommand, WithdrawCommand},
        contracts, utils,
    };

    #[tokio::test]
    async fn test_a_1_eth_to_usdc() -> Result<()> {
        let args = PerpArgs {
            cmd: Swap(SwapCommand {
                token_in: String::from("0x4200000000000000000000000000000000000006")
                    .parse::<Address>()?,
                amount_in: 1.75271,
                token_out: String::from("0x7f5c764cbc14f9669b88837ca1490cca17c31607")
                    .parse::<Address>()?,
                slippage: 0.5,
                eth: Some(true),
            }),
        };
        match_args(args).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_a_2_usdc_to_weth() -> Result<()> {
        let args = PerpArgs {
            cmd: Swap(SwapCommand {
                token_in: String::from("0x7f5c764cbc14f9669b88837ca1490cca17c31607")
                    .parse::<Address>()?,
                amount_in: 750.39121,
                token_out: String::from("0x4200000000000000000000000000000000000006")
                    .parse::<Address>()?,
                slippage: 0.5,
                eth: Some(false),
            }),
        };
        match_args(args).await?;
        Ok(())
    }

    // #[tokio::test]
    // async fn test_a_2_usdc_to_frax() -> Result<()> {
    //     let args = PerpArgs {
    //         cmd: Swap( SwapCommand {
    //             token_in: String::from("0x7f5c764cbc14f9669b88837ca1490cca17c31607").parse::<Address>()?,
    //             amount_in: 100.01,
    //             token_out: String::from("0x2E3D870790dC77A83DD1d18184Acc7439A53f475").parse::<Address>()?,
    //             slippage: 0.5,
    //             eth: Some(false),
    //         }),
    //     };
    //     match_args(args).await?;
    //     Ok(())
    // }

    #[tokio::test]
    async fn test_a_3_weth_to_usdt() -> Result<()> {
        let args = PerpArgs {
            cmd: Swap(SwapCommand {
                token_in: String::from("0x4200000000000000000000000000000000000006")
                    .parse::<Address>()?,
                amount_in: 0.132156,
                token_out: String::from("0x94b008aA00579c1307B0EF2c499aD98a8ce58e58")
                    .parse::<Address>()?,
                slippage: 0.5,
                eth: Some(false),
            }),
        };
        match_args(args).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_a_4_weth_to_op() -> Result<()> {
        let args = PerpArgs {
            cmd: Swap(SwapCommand {
                token_in: String::from("0x4200000000000000000000000000000000000006")
                    .parse::<Address>()?,
                amount_in: 0.132156,
                token_out: String::from("0x4200000000000000000000000000000000000042")
                    .parse::<Address>()?,
                slippage: 0.5,
                eth: Some(false),
            }),
        };
        match_args(args).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_b_deposit_no_arguments() -> Result<()> {
        let args = PerpArgs {
            cmd: Deposit(DepositCommand {
                token: None,
                amount: None,
                eth: None,
            }),
        };
        match_args(args).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_c_1_deposit_eth() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("WETH")
            .expect("Weth Address")
            .to_owned();

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let eth_in = 1.24938272;
        let args = PerpArgs {
            cmd: Deposit(DepositCommand {
                token: None,
                amount: None,
                eth: Some(eth_in),
            }),
        };
        match_args(args).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let token_balance =
            ethers::utils::format_units(post_balance - pre_balance, 18)?.parse::<f64>()?;
        assert_eq!(eth_in, token_balance);
        Ok(())
    }

    #[tokio::test]
    async fn test_c_2_deposit_usdt() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("USDT")
            .expect("USDT Address")
            .to_owned();
        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;
        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;
        let amount_in = 10.245;
        let args = PerpArgs {
            cmd: Deposit(DepositCommand {
                token: Some(token),
                amount: Some(amount_in),
                eth: None,
            }),
        };
        match_args(args).await?;
        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        assert_eq!(
            I256::from(ethers::utils::parse_units(amount_in, decimals as u32)?),
            post_balance
                .checked_sub(pre_balance)
                .expect("I256 of balance difference")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_c_3_deposit_weth() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("WETH")
            .expect("WETH Address")
            .to_owned();
        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;
        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;
        let amount_in = 0.000002892888188187;
        let args = PerpArgs {
            cmd: Deposit(DepositCommand {
                token: Some(token),
                amount: Some(amount_in),
                eth: None,
            }),
        };
        match_args(args).await?;
        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        assert_eq!(
            I256::from(ethers::utils::parse_units(amount_in, decimals as u32)?),
            post_balance
                .checked_sub(pre_balance)
                .expect("I256 of balance difference")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_c_4_deposit_usdc() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("USDC")
            .expect("USDC Address")
            .to_owned();
        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;
        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;
        let amount_in = 129.124658;
        let args = PerpArgs {
            cmd: Deposit(DepositCommand {
                token: Some(token),
                amount: Some(amount_in),
                eth: None,
            }),
        };
        match_args(args).await?;
        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        assert_eq!(
            I256::from(ethers::utils::parse_units(amount_in, decimals as u32)?),
            post_balance
                .checked_sub(pre_balance)
                .expect("I256 of balance difference")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_c_5_deposit_op() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("OP")
            .expect("OP Address")
            .to_owned();
        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;
        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;
        let amount_in = 12.157;
        let args = PerpArgs {
            cmd: Deposit(DepositCommand {
                token: Some(token),
                amount: Some(amount_in),
                eth: None,
            }),
        };
        match_args(args).await?;
        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        assert_eq!(
            I256::from(ethers::utils::parse_units(amount_in, decimals as u32)?),
            post_balance
                .checked_sub(pre_balance)
                .expect("I256 of balance difference")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_d_1_withdraw_eth() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("WETH")
            .expect("Weth Address")
            .to_owned();
        let eth_out = 0.432165;

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Withdraw(WithdrawCommand {
                token: None,
                amount: None,
                eth: Some(eth_out),
            }),
        };
        match_args(arg).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;

        let token_balance =
            ethers::utils::format_units(pre_balance - post_balance, decimals as u32)?
                .parse::<f64>()?;
        assert_eq!(eth_out, token_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_d_2_withdraw_weth() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("WETH")
            .expect("Weth Address")
            .to_owned();
        let amount = 0.132165;

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Withdraw(WithdrawCommand {
                token: Some(String::from("0x4200000000000000000000000000000000000006")),
                amount: Some(amount),
                eth: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;

        let token_balance =
            ethers::utils::format_units(pre_balance - post_balance, decimals as u32)?
                .parse::<f64>()?;
        assert_eq!(amount, token_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_d_3_withdraw_usdc() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("USDC")
            .expect("USDC Address")
            .to_owned();
        let amount = 3.133165;

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Withdraw(WithdrawCommand {
                token: Some(String::from("0x7F5c764cBc14f9669B88837ca1490cCa17c31607")),
                amount: Some(amount),
                eth: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;

        let token_balance =
            ethers::utils::format_units(pre_balance - post_balance, decimals as u32)?
                .parse::<f64>()?;
        assert_eq!(amount, token_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_d_4_withdraw_usdt() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("USDT")
            .expect("USDT Address")
            .to_owned();
        let amount = 0.132165;

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Withdraw(WithdrawCommand {
                token: Some(String::from("0x94b008aA00579c1307B0EF2c499aD98a8ce58e58")),
                amount: Some(amount),
                eth: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;

        let token_balance =
            ethers::utils::format_units(pre_balance - post_balance, decimals as u32)?
                .parse::<f64>()?;
        assert_eq!(amount, token_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_d_5_withdraw_op() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?
            .get("OP")
            .expect("OP Address")
            .to_owned();
        let amount = 1.132165;

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Withdraw(WithdrawCommand {
                token: Some(String::from("0x4200000000000000000000000000000000000042")),
                amount: Some(amount),
                eth: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let token_contract = contracts::get_token_contract(token)?;
        let decimals = token_contract.decimals().call().await?;

        let token_balance =
            ethers::utils::format_units(pre_balance - post_balance, decimals as u32)?
                .parse::<f64>()?;
        assert_eq!(amount, token_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_e_shorting() -> Result<()> {
        let account_balance_contract = contracts::get_account_balance().await?;
        let wallet = utils::get_wallet()?;
        let trader = wallet.address();
        let amount_out = 14.3;
        let token = "0x8711ac690984BAdC42e7AbE71f351b5Ac2a2Ad0d".parse::<H160>()?;
        let pre_balance = account_balance_contract
            .get_total_position_size(trader, token)
            .call()
            .await?;
        let arg = PerpArgs {
            cmd: Open(OpenCommand {
                long: Some(false),
                short: Some(true),
                token: String::from("DOGE"),
                input: Some(true),
                output: Some(false),
                order_amount: amount_out,
                limit: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = account_balance_contract
            .get_total_position_size(trader, token)
            .call()
            .await?;

        assert!(post_balance < pre_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_f_longing() -> Result<()> {
        let account_balance_contract = contracts::get_account_balance().await?;
        let wallet = utils::get_wallet()?;
        let trader = wallet.address();
        let amount_out = 14.3;
        let token = "0x8711ac690984BAdC42e7AbE71f351b5Ac2a2Ad0d".parse::<H160>()?;
        let pre_balance = account_balance_contract
            .get_total_position_size(trader, token)
            .call()
            .await?;
        let arg = PerpArgs {
            cmd: Open(OpenCommand {
                long: Some(true),
                short: Some(false),
                token: String::from("vDOGE"),
                input: Some(true),
                output: Some(false),
                order_amount: amount_out,
                limit: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = account_balance_contract
            .get_total_position_size(trader, token)
            .call()
            .await?;

        assert!(post_balance > pre_balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_g_output() -> Result<()> {
        let account_balance_contract = contracts::get_account_balance().await?;
        let wallet = utils::get_wallet()?;
        let trader = wallet.address();
        let amount_out = 14.3;
        let token = "0xBe5de48197fc974600929196239E264EcB703eE8".parse::<H160>()?;
        let pre_balance = account_balance_contract
            .get_total_position_size(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Open(OpenCommand {
                long: Some(true),
                short: Some(false),
                token: String::from("MATIC"),
                input: Some(false),
                output: Some(true),
                order_amount: amount_out,
                limit: None,
            }),
        };
        match_args(arg).await?;

        let post_balance = account_balance_contract
            .get_total_position_size(trader, token)
            .call()
            .await?;

        assert_eq!(
            ethers::utils::format_units(post_balance - pre_balance, 18)?.parse::<f64>()?,
            amount_out
        );

        Ok(())
    }
}
