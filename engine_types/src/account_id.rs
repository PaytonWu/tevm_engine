#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub enum ParseAccountError {
    TooLong,
    TooShort,
    Invalid,
}

const ACCOUNT_ADDRESS_LENGTH: usize = 46;
const ACCOUNT_ADDRESS_TABLE_ID_PREFIX: u8 = b'@';

#[derive(Eq, Ord, Hash, Clone, Debug, PartialEq, PartialOrd)]
pub struct AccountAddress([u8;46]);

impl AccountAddress {
    pub fn new(account_address: &str) -> Result<Self, ParseAccountError> {
        unimplemented!()
    }

    pub fn validate(account_address: &str) -> Result<(), ParseAccountError> {
        if account_address.as_bytes().contains(&ACCOUNT_ADDRESS_TABLE_ID_PREFIX) {
            Err(ParseAccountError::Invalid)
        }

        if account_address.len() != ACCOUNT_ADDRESS_LENGTH {
            Err(ParseAccountError::Invalid)
        }

        if account_address.as_bytes()[0] != b'T' {
            Err(ParseAccountError::Invalid)
        }

        let sub = account_address.as_bytes().

        OK()
    }
}