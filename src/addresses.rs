use ethers::types::Address;

/// Contract Addresses
pub struct Contracts {
    /// AccountBalance
    pub i_account_balance: Address,
    /// ClearingHouse
    pub i_clearing_house: Address,
}

/// Contract Addresses
pub fn contracts() -> Contracts {
    Contracts {
        i_account_balance: "0xA7f3FC32043757039d5e13d790EE43edBcBa8b7c"
            .parse::<Address>()
            .expect("fail"),
        i_clearing_house: "0x82ac2CE43e33683c58BE4cDc40975E73aA50f459"
            .parse::<Address>()
            .expect("fail"),
    }
}