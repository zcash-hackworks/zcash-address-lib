
// mod zcash_address::{ZcashAddress};
uniffi_macros::include_scaffolding!("zcash_address");

use zcash_address::{ZcashAddress, ParseError};

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("The received address string is invalid")]
    InvalidAddress,
    #[error("The received address string does not appear to be a Zcash address")]
    NotZcash,
    #[error("The received address was matched as a UA but resulted invalid")]
    InvalidUnifiedAddress,
}

#[derive(Debug)]
pub struct Address {
    string: String,
}

fn parse(
    address_text: String
) -> Result<Address, AddressError> {
    ZcashAddress::try_from_encoded(&address_text.as_str())
        .map(|zcash_address| zcash_address.to_address())
        .map_err(|e| e.to_address_error())
}

fn derive_transparent_address(
    seed_bytes: &[u8], 
    account: i32, 
    index: i32
) -> Result<Address, AddressError> {
    Err(AddressError::InvalidAddress)
}

fn derive_sapling_address(
    seed_bytes: &[u8],
    account: i32,
    index: i32
) -> Result<Address, AddressError> {
    Err(AddressError::InvalidAddress)       
}

fn derive_unified_address(
    seed_bytes: &[u8],
    account: i32,
    index: i32
) -> Result<Address, AddressError> {
    Err(AddressError::InvalidAddress)       
}
trait ToAddress {
    fn to_address(&self) -> Address;
}

trait ToAddressError {
    fn to_address_error(&self) -> AddressError;
}

impl ToAddressError for ParseError {
    fn to_address_error(&self) -> AddressError {
        match self {
            ParseError::InvalidEncoding => AddressError::InvalidAddress,
            ParseError::NotZcash => AddressError::InvalidAddress,
            ParseError::Unified(_) => AddressError::InvalidUnifiedAddress
        }
    }
}
impl ToAddress for ZcashAddress {
    fn to_address(&self) -> Address {
        let address = Address {
            string: self.to_string()
        };
        return address

    }
}