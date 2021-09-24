
// mod zcash_address::{ZcashAddress};
uniffi_macros::include_scaffolding!("zcash_address");

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("The received address string is invalid")]
    InvalidAddress,
}
#[derive(Debug, Clone)]
pub enum Kind {
    Sransparent,
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

    fn derive_transaparent_address(
        seed_bytes: &[u8], 
        account: i32, 
        index: i32
    ) -> Result<Self, AddressError> {
        Err(AddressError::InvalidAddress)
    }

    fn derive_sapling_address(
        seed_bytes: &[u8],
        account: i32,
        index: i32
    ) -> Result<Self, AddressError> {
        Err(AddressError::InvalidAddress)       
    }

    fn derive_unified_address(
        seed_bytes: &[u8],
        account: i32,
        index: i32
    ) -> Result<Self, AddressError> {
        Err(AddressError::InvalidAddress)       
    }
}