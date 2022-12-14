use crate::args::TokensCommand;
use crate::address_list;
use eyre::Result;


/// Primary function to process tokens command
pub async fn process(symbol: TokensCommand) -> Result<()> {
    let token_addresses = address_list::get_token_addresses().await?;
    println!();
    for (key, val) in token_addresses {
        match symbol.symbol {
           Some(ref token) => {if token == &key {println!("{}: {:?}", key, val); break;}},
           None => {println!("{}: {:?}", key, val); continue;},
        }
    }
    println!();
    Ok(())
}