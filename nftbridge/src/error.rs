use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Error {
    InvalidAccount = 1,
    MissingInstaller = 2,
    InvalidContext = 3,
    InvalidIdentifierMode = 4,
    MissingTokenID = 5,
    InvalidTokenIdentifier = 6,
    FailedToGetArgBytes = 7,
    InvalidContractOwner = 8,
    RequestIdIlledFormat = 9,
    FailedToCreateDictionary = 10,
    RequestIdRepeated = 11,
    MissingKey = 12,
    SerilizationError = 13,
    UnlockIdRepeated = 14,
    FailedToCreateDictionaryUnlockIds = 15,
    ContractAlreadyInitialized = 16,
    CallerMustBeAccountHash = 17,
    TooManyTokenIds = 18,
    UnlockIdIllFormatted = 19,
    TxHashUnlockIdIllFormatted = 20,
    InvalidDev = 100,
    InvalidWrappedToken = 101,
    MissingContractOwner = 102,
    MissingDev = 103,
    MissingBridgeContractHash = 104,
    InvalidBridgeContractHash = 105,
    InvalidNftSupportTypes = 106,
    InvalidTokenMode = 107,
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::User(e as u16)
    }
}
