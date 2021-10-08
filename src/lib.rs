
// mod zcash_address::{ZcashAddress};
uniffi_macros::include_scaffolding!("zcash_address");
use std::convert::TryFrom;

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
    kind: AddressType,
}

#[derive(Debug)]
pub enum AddressType {
    Sprout,
    Transparent,
    Sapling,
    Unified,
}

fn parse(
    address_text: String
) -> Result<Address, AddressError> {
    ZcashAddress::try_from_encoded(&address_text.as_str())
        .map( |zcash_address| Address::try_from(&zcash_address).unwrap())
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

impl TryFrom<&ZcashAddress> for Address {
    type Error = AddressError;

    fn try_from(zcash_address:&ZcashAddress) ->  Result<Address, Self::Error> {
        let address_kind = AddressType::try_from(zcash_address).unwrap();
        let address = Address {
            string: zcash_address.to_string(),
            kind: address_kind,
        };
        return Ok(address)
    }
}

impl TryFrom<&ZcashAddress> for AddressType {
    type Error = AddressError;    

    fn try_from(_: &ZcashAddress) -> Result<AddressType, Self::Error> {
            Ok(AddressType::Sapling)
        }
}
