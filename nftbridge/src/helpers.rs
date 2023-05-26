use crate::address::Address;
use crate::error::Error;
use alloc::string::{String, ToString};

use alloc::vec::Vec;
use casper_contract::{
    contract_api::{self},
    ext_ffi,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::bytesrepr::FromBytes;
use casper_types::CLTyped;
use casper_types::{
    api_error,
    bytesrepr::{self},
    ApiError,
};
use serde::{Deserialize, Serialize};

use casper_types::{bytesrepr::ToBytes, Key};
use casper_types::{system::CallStackElement, ContractHash, URef, U256};
use core::convert::TryFrom;
use core::convert::TryInto;
use core::mem::MaybeUninit;
use core::u64;

use crate::constants::*;

use crate::error;

// Helper functions

pub(crate) fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let result = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(result)
        }
    }
}

pub(crate) fn get_key_from_address(addr: &Address) -> Key {
    let self_key = match *addr {
        Address::Account(acc) => Key::from(acc),
        Address::Contract(contract_package_hash) => Key::from(contract_package_hash),
    };
    self_key
}

pub(crate) fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

/// Gets the immediate call stack element of the current execution.
fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

pub(crate) fn get_verified_caller() -> Result<Key, Error> {
    match *runtime::get_call_stack()
        .iter()
        .nth_back(1)
        .unwrap_or_revert()
    {
        CallStackElement::Session {
            account_hash: calling_account_hash,
        } => Ok(Key::Account(calling_account_hash)),
        CallStackElement::StoredSession { contract_hash, .. }
        | CallStackElement::StoredContract { contract_hash, .. } => Ok(contract_hash.into()),
    }
}

pub(crate) fn get_stored_value_with_user_errors<T: CLTyped + FromBytes>(
    name: &str,
    missing: Error,
    invalid: Error,
) -> T {
    let uref = get_uref(name);
    read_with_user_errors(uref, missing, invalid)
}
pub(crate) fn read_with_user_errors<T: CLTyped + FromBytes>(
    uref: URef,
    missing: Error,
    invalid: Error,
) -> T {
    let key: Key = uref.into();
    let (key_ptr, key_size, _bytes) = to_ptr(key);

    let value_size = {
        let mut value_size = MaybeUninit::uninit();
        let ret = unsafe { ext_ffi::casper_read_value(key_ptr, key_size, value_size.as_mut_ptr()) };
        match api_error::result_from(ret) {
            Ok(_) => unsafe { value_size.assume_init() },
            Err(ApiError::ValueNotFound) => runtime::revert(missing),
            Err(e) => runtime::revert(e),
        }
    };

    let value_bytes = read_host_buffer(value_size).unwrap_or_revert();

    bytesrepr::deserialize(value_bytes).unwrap_or_revert_with(invalid)
}

pub(crate) fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
    let bytes = t.into_bytes().unwrap_or_revert();
    let ptr = bytes.as_ptr();
    let size = bytes.len();
    (ptr, size, bytes)
}
pub(crate) fn read_host_buffer(size: usize) -> Result<Vec<u8>, ApiError> {
    let mut dest: Vec<u8> = if size == 0 {
        Vec::new()
    } else {
        let bytes_non_null_ptr = contract_api::alloc_bytes(size);
        unsafe { Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), size, size) }
    };
    read_host_buffer_into(&mut dest)?;
    Ok(dest)
}
pub(crate) fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
    let mut bytes_written = MaybeUninit::uninit();
    let ret = unsafe {
        ext_ffi::casper_read_host_buffer(dest.as_mut_ptr(), dest.len(), bytes_written.as_mut_ptr())
    };
    // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
    // caller ignores the return value, execution of the contract becomes unstable and ultimately
    // leads to `Unreachable` error.
    api_error::result_from(ret)?;
    Ok(unsafe { bytes_written.assume_init() })
}

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that only session code can execute this function, and disallows stored
/// session/stored contracts.
pub(crate) fn get_immediate_caller_address() -> Result<Address, Error> {
    get_immediate_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

pub(crate) fn get_immediate_caller_key() -> Key {
    let addr = get_immediate_caller_address().unwrap_or_revert();
    get_key_from_address(&addr)
}

#[no_mangle]
pub(crate) fn dictionary_write(dictionary_uref: URef, address: Address, amount: U256) {
    let dictionary_item_key = make_dictionary_item_key(address);
    storage::dictionary_put(dictionary_uref, &dictionary_item_key, amount);
}

/// Creates a dictionary item key for a dictionary item.
#[no_mangle]
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64::encode(&preimage)
}

pub(crate) fn make_dictionary_item_key_for_contract(contract_hash: Key) -> String {
    let pre_contract = contract_hash.into_hash().unwrap_or_revert();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    hex::encode(&pre_contract)
}

#[no_mangle]
pub(crate) fn dictionary_read(dictionary_uref: URef, address: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(address);

    storage::dictionary_get(dictionary_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Copy, Clone)]
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

pub(crate) fn get_token_identifiers_from_runtime_args(
    identifier_mode: &NFTIdentifierMode,
) -> Vec<TokenIdentifier> {
    match identifier_mode {
        NFTIdentifierMode::Ordinal => get_named_arg_with_user_errors::<Vec<u64>>(
            ARG_TOKEN_IDS,
            Error::MissingTokenID,
            Error::InvalidTokenIdentifier,
        )
        .unwrap_or_revert()
        .iter()
        .map(|identifier| TokenIdentifier::new_index(*identifier))
        .collect::<Vec<_>>(),
        NFTIdentifierMode::Hash => get_named_arg_with_user_errors::<Vec<String>>(
            ARG_TOKEN_HASHES,
            Error::MissingTokenID,
            Error::InvalidTokenIdentifier,
        )
        .unwrap_or_revert()
        .iter()
        .map(|identier| TokenIdentifier::new_hash(identier.clone()))
        .collect::<Vec<_>>(),
    }
}

pub(crate) fn get_identifier_mode_from_runtime_args() -> NFTIdentifierMode {
    let identifier_mode_u8: u8 = runtime::get_named_arg(ARG_IDENTIFIER_MODE);
    let identifier_mode =
        NFTIdentifierMode::try_from(identifier_mode_u8).unwrap_or(NFTIdentifierMode::Ordinal);
    identifier_mode
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
}

pub(crate) fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name).unwrap_or_revert();
    key.into_uref().unwrap_or_revert()
}

pub(crate) fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    dictionary_name: &str,
    key: &str,
) -> Option<T> {
    let seed_uref = get_uref(dictionary_name);

    match storage::dictionary_get::<T>(seed_uref, key) {
        Ok(maybe_value) => maybe_value,
        Err(_) => None,
    }
}

pub(crate) fn write_dictionary_value_from_key<T: CLTyped + FromBytes + ToBytes>(
    dictionary_name: &str,
    key: &str,
    value: T,
) {
    let seed_uref = get_uref(dictionary_name);

    match storage::dictionary_get::<T>(seed_uref, key) {
        Ok(None | Some(_)) => storage::dictionary_put(seed_uref, key, value),
        Err(error) => runtime::revert(error),
    }
}

pub(crate) fn get_unlock_id_key(unlock_id: &str) -> String {
    let unlock_id_bytes = unlock_id.as_bytes();
    let key_bytes = runtime::blake2b(unlock_id_bytes);
    hex::encode(&key_bytes)
}

pub fn log_msg(msg: &str) {
    // runtime::print(msg);
}
pub fn encode_dictionary_item_key(key: Key) -> String {
    match key {
        Key::Account(account_hash) => account_hash.to_string(),
        Key::Hash(hash_addr) => ContractHash::new(hash_addr).to_string(),
        _ => runtime::revert(Error::InvalidKey),
    }
}
