use clap::Parser;
use perpcli_rs::{amm, args::{PerpArgs, SubCommand}, open, close, position, portfolio, quit, tokens, withdraw, deposit};

fn main() {
    env_logger::init();
    let args = PerpArgs::parse();
    match args.cmd {
        SubCommand::Position(position) => position::process(position),
        SubCommand::Portfolio(portfolio) => portfolio::process(portfolio),
        SubCommand::Amm(amm) => amm::process(amm),
        SubCommand::Quit(token) => quit::process(token),
        SubCommand::Tokens(symbol) => tokens::process(symbol),
        SubCommand::Deposit(args) => deposit::process(args),
        SubCommand::Withdraw(args) => withdraw::process(args),
        SubCommand::Open(args) => open::process(args),
        SubCommand::Close(args) => close::process(args),
    }
}
