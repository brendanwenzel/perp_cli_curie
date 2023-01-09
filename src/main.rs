use clap::Parser;
use perpcli_rs::{amm, args::{PerpArgs, SubCommand::*}, open, close, position, portfolio, quit, tokens, withdraw, deposit};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = PerpArgs::parse();
    match_args(args).await?;
    Ok(())
}

async fn match_args(args: PerpArgs) -> Result<()> {
    match args.cmd {
        Position(position) => position::process(position).await?,
        Portfolio(portfolio) => portfolio::process(portfolio).await?,
        Amm(amm) => amm::process(amm).await?,
        Quit(token) => quit::process(token).await?,
        Tokens(symbol) => tokens::process(symbol).await?,
        Deposit(args) => deposit::process(args).await?,
        Withdraw(args) => withdraw::process(args).await?,
        Open(args) => open::process(args).await?,
        Close(args) => close::process(args).await?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use perpcli_rs::{args::{DepositCommand, OpenCommand, WithdrawCommand}, utils, contracts, address_list};
    use ethers::prelude::*;

    #[tokio::test]
    async fn test_a_deposit_with_no_args() {
        let args = PerpArgs {
            cmd: Deposit(
                DepositCommand {
                    token: None,
                    amount: None,
                    eth: None,
                }
            )
        };
        let process = match_args(args).await.unwrap();
        process
    }

    #[tokio::test]
    async fn test_b_deposit_eth() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?.get("WETH").expect("Weth Address").to_owned();

        let pre_balance = vault_contract
        .get_balance_by_token(trader, token)
        .call()
        .await?;

        let eth_in = 1.24938272;
        let args = PerpArgs {
            cmd: Deposit(
                DepositCommand {
                    token: None,
                    amount: None,
                    eth: Some(eth_in),
                }
            )
        };
        match_args(args).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;
        
        let token_balance = ethers::utils::format_units(post_balance - pre_balance, 18)?.parse::<f64>()?;
        assert_eq!(eth_in, token_balance);
        Ok(())
    }

    #[tokio::test]
    async fn test_c_withdraw_eth() -> Result<()> {
        let vault_contract = contracts::get_vault().await?;
        let trader = utils::get_wallet()?.address();
        let token = address_list::get_collateral_tokens()?.get("WETH").expect("Weth Address").to_owned();
        let eth_out = 0.432165;

        let pre_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;

        let arg = PerpArgs {
            cmd: Withdraw(
                    WithdrawCommand {
                        token: None,
                        amount: None,
                        eth: Some(eth_out),
                    }
                )
        };
        match_args(arg).await?;

        let post_balance = vault_contract
            .get_balance_by_token(trader, token)
            .call()
            .await?;
        
        let token_balance = ethers::utils::format_units(pre_balance - post_balance, 18)?.parse::<f64>()?;
        assert_eq!(eth_out, token_balance);

        Ok(())
    }



    #[tokio::test]
    async fn test_d_shorting() -> Result<()> {
        let arg = PerpArgs {
            cmd: Open(
                OpenCommand {
                    long: Some(false),
                    short: Some(true),
                    token: String::from("BNB"),
                    input: Some(true),
                    output: Some(false),
                    order_amount: 5.12423,
                    limit: None,
                }
            )
        };
        match_args(arg).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_e_longing() -> Result<()> {
        let arg = PerpArgs {
            cmd: Open(
                OpenCommand {
                    long: Some(true),
                    short: Some(false),
                    token: String::from("BNB"),
                    input: Some(true),
                    output: Some(false),
                    order_amount: 5.12423,
                    limit: None,
                }
            )
        };
        match_args(arg).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_f_output() -> Result<()> {
        let arg = PerpArgs {
            cmd: Open(
                OpenCommand {
                    long: Some(true),
                    short: Some(false),
                    token: String::from("BNB"),
                    input: Some(false),
                    output: Some(true),
                    order_amount: 5.12423,
                    limit: None,
                }
            )
        };
        match_args(arg).await?;
        Ok(())
    }

}