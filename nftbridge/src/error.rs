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
    FailedToGetArgBytes = 7
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::User(e as u16)
    }
}
