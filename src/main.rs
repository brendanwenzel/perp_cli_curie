use clap::Parser;
use perpcli_rs::{amm, args::{PerpArgs, SubCommand}, position, portfolio};

fn main() {
    env_logger::init();
    let args = PerpArgs::parse();
    match args.cmd {
        SubCommand::Position(position) => position::process(position),
        SubCommand::Portfolio(portfolio) => portfolio::process(portfolio),
        SubCommand::Amm(amm) => amm::process(amm),
        // SubCommand::Verify(verify) => verify::process(verify),
    }
}
