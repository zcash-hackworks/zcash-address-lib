
// mod zcash_address::{ZcashAddress};

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("The received address string is invalid")]
    InvalidAddress,
}
#[derive(Debug, Clone)]
pub enum Kind {
    Sransparent,
    Sprout,
    Sapling,
    Unified,
}
#[derive(Debug, Clone)]
pub enum Network {
    Main,
    Test,
    Regtest
}

#[derive(Debug)]
pub struct Address {
    kind: Kind,
    network: Network,
    string: String,
}


impl Address {
    fn parse(address_text: &str) -> Result<Self, AddressError> {
        Err(AddressError::InvalidAddress)
    }
}