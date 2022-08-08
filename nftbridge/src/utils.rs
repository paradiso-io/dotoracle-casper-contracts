use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::convert::TryFrom;
use serde::{Deserialize, Serialize};

use casper_contract::{
    contract_api::{self, runtime},
    ext_ffi,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error,
    bytesrepr::{self, FromBytes},
    ApiError,
};

use crate::error::Error;

use crate::{
    ARG_TOKEN_HASH, ARG_TOKEN_ID, error,
};

#[repr(u8)]
pub enum NFTIdentifierMode {
    Ordinal = 0,
    Hash = 1,
}

impl TryFrom<u8> for NFTIdentifierMode {
    type Error = error::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NFTIdentifierMode::Ordinal),
            1 => Ok(NFTIdentifierMode::Hash),
            _ => Err(Error::InvalidIdentifierMode),
        }
    }
}

pub(crate) fn get_named_arg_size(name: &str) -> Option<usize> {
    let mut arg_size: usize = 0;
    let ret = unsafe {
        ext_ffi::casper_get_named_arg_size(
            name.as_bytes().as_ptr(),
            name.len(),
            &mut arg_size as *mut usize,
        )
    };
    match api_error::result_from(ret) {
        Ok(_) => Some(arg_size),
        Err(ApiError::MissingArgument) => None,
        Err(e) => runtime::revert(e),
    }
}

pub(crate) fn get_token_identifier_from_runtime_args(
    identifier_mode: &NFTIdentifierMode,
) -> TokenIdentifier {
    match identifier_mode {
        NFTIdentifierMode::Ordinal => get_named_arg_with_user_errors::<u64>(
            ARG_TOKEN_ID,
            Error::MissingTokenID,
            Error::InvalidTokenIdentifier,
        )
        .map(TokenIdentifier::new_index)
        .unwrap_or_revert(),
        NFTIdentifierMode::Hash => get_named_arg_with_user_errors::<String>(
            ARG_TOKEN_HASH,
            Error::MissingTokenID,
            Error::InvalidTokenIdentifier,
        )
        .map(TokenIdentifier::new_hash)
        .unwrap_or_revert(),
    }
}

pub(crate) fn get_named_arg_with_user_errors<T: FromBytes>(
    name: &str,
    missing: Error,
    invalid: Error,
) -> Result<T, Error> {
    let arg_size = get_named_arg_size(name).ok_or(missing)?;
    let arg_bytes = if arg_size > 0 {
        let res = {
            let data_non_null_ptr = contract_api::alloc_bytes(arg_size);
            let ret = unsafe {
                ext_ffi::casper_get_named_arg(
                    name.as_bytes().as_ptr(),
                    name.len(),
                    data_non_null_ptr.as_ptr(),
                    arg_size,
                )
            };
            let data =
                unsafe { Vec::from_raw_parts(data_non_null_ptr.as_ptr(), arg_size, arg_size) };
            api_error::result_from(ret).map(|_| data)
        };
        // Assumed to be safe as `get_named_arg_size` checks the argument already
        res.unwrap_or_revert_with(Error::FailedToGetArgBytes)
    } else {
        // Avoids allocation with 0 bytes and a call to get_named_arg
        Vec::new()
    };

    bytesrepr::deserialize(arg_bytes).map_err(|_| invalid)
}

#[derive(PartialEq, Clone)]
pub(crate) enum TokenIdentifier {
    Index(u64),
    Hash(String),
}

impl TokenIdentifier {
    pub(crate) fn new_index(index: u64) -> Self {
        TokenIdentifier::Index(index)
    }

    pub(crate) fn new_hash(hash: String) -> Self {
        TokenIdentifier::Hash(hash)
    }

    pub(crate) fn get_index(&self) -> Option<u64> {
        if let Self::Index(index) = self {
            return Some(*index);
        }
        None
    }

    pub(crate) fn get_hash(self) -> Option<String> {
        if let Self::Hash(hash) = self {
            return Some(hash);
        }
        None
    }

    pub(crate) fn get_dictionary_item_key(&self) -> String {
        match self {
            TokenIdentifier::Index(token_index) => token_index.to_string(),
            TokenIdentifier::Hash(hash) => hash.clone(),
        }
    }
}

// Metadata mutability is different from schema mutability.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MetadataSchemaProperty {
    name: String,
    description: String,
    required: bool,
}

// Using a structure for the purposes of serialization formatting.
#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataNFT721 {
    name: String,
    symbol: String,
    token_uri: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataCEP78 {
    name: String,
    token_uri: String,
    checksum: String,
}

// Using a structure for the purposes of serialization formatting.
#[derive(Serialize, Deserialize)]
pub(crate) struct CustomMetadata {
    attributes: BTreeMap<String, String>,
}