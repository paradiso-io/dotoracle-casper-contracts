#![no_main]
#![no_std]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

mod address;
pub mod constants;
mod entry_points;
mod error;
mod helpers;
mod named_keys;
mod utils;

use crate::address::Address;
use crate::helpers::get_immediate_caller_address;

use crate::constants::*;
use crate::error::Error;
use crate::utils::*;
use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ContractHash, HashAddr, Key, RuntimeArgs, URef};
use core::convert::TryFrom;
use helpers::{get_self_address};

#[no_mangle]
fn call() {
    let contract_name: String = runtime::get_named_arg(NFT_BRIDGE_CONTRACT_KEY_NAME);
    let contract_hash_key_name = String::from(contract_name.clone());
    let contract_package_hash_key_name = String::from(contract_name.clone() + "_package_hash");

    let contract_owner: Address = runtime::get_named_arg(CONTRACT_OWNER_KEY_NAME);

    let named_keys: NamedKeys = named_keys::default(contract_name, contract_owner);

    // We store contract on-chain
    let (contract_hash, _version) = storage::new_locked_contract(
        entry_points::default(),
        Some(named_keys),
        Some(String::from(contract_package_hash_key_name)),
        None,
    );
    runtime::put_key(
        CONTRACT_OWNER_KEY_NAME,
        Key::from(*contract_owner.as_account_hash().unwrap()),
    );
    runtime::put_key(contract_hash_key_name.as_str(), Key::from(contract_hash));
}

#[no_mangle]
pub extern "C" fn request_bridge_nft() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    let identifier_mode_u8: u8 = runtime::get_named_arg(ARG_IDENTIFIER_MODE);
    let identifier_mode = NFTIdentifierMode::try_from(identifier_mode_u8).unwrap_or(NFTIdentifierMode::Ordinal);
    let user: Address = get_immediate_caller_address().unwrap_or_revert();
    let user_formated_string = user.as_account_hash().unwrap().to_formatted_string();
    let token_identifier: TokenIdentifier =
        get_token_identifier_from_runtime_args(&identifier_mode);
    cep78_transfer_from(
        &contract_hash,
        Key::from_formatted_str(&user_formated_string).unwrap(),
        identifier_mode,
        token_identifier,
    );
    //U256::one()
}

#[no_mangle]
pub extern "C" fn unlock_nft() -> Result<(), Error> {
    let caller = get_immediate_caller_address().unwrap_or_revert();
    let caller = Key::from(*caller.as_account_hash().unwrap());
    let contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap();
    if caller != contract_owner {
        runtime::revert(Error::InvalidAccount);
    }

    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    let identifier_mode_u8: u8 = runtime::get_named_arg(ARG_IDENTIFIER_MODE);
    let identifier_mode = NFTIdentifierMode::try_from(identifier_mode_u8).unwrap_or(NFTIdentifierMode::Ordinal);
    let token_identifier: TokenIdentifier =
        get_token_identifier_from_runtime_args(&identifier_mode);
    Ok(())
}

fn cep78_transfer_from(
    contract_hash: &Key,
    requester: Key,
    identifier_mode: NFTIdentifierMode,
    token_identifier: TokenIdentifier,
) {
    let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);
    let self_address = get_self_address();
    if self_address.is_err() {
        runtime::revert(error::Error::InvalidAccount);
    }
    let self_address = self_address.unwrap_or_revert();
    let self_key = match self_address {
        Address::Account(acc) => Key::from(acc),
        Address::Contract(contract_package_hash) => Key::from(contract_package_hash),
    };
    // let self_address = Address::Account(AccountHash::from_formatted_str("account-hash-32b0eaaa6c0d024e2e7efc34a0a8aad7889cdbb87c71f07cb0eb4f515d5696de").unwrap());
    //let self_key = get_key_from_address(&self_address);
    match identifier_mode {
        NFTIdentifierMode::Ordinal => {
            let _: (String, Key) = runtime::call_contract(
                contract_hash,
                TRANSFER_ENTRY_POINT_NAME,
                runtime_args! {
                    ARG_SOURCE_KEY => requester,
                    ARG_TARGET_KEY => self_key,
                    ARG_TOKEN_ID => token_identifier.get_index().unwrap()
                },
            );
        }
        NFTIdentifierMode::Hash => {
            let _: (String, Key) = runtime::call_contract(
                contract_hash,
                TRANSFER_ENTRY_POINT_NAME,
                runtime_args! {
                    ARG_SOURCE_KEY => requester,
                    ARG_TARGET_KEY => self_key,
                    ARG_TOKEN_HASH => token_identifier.get_hash().unwrap()
                },
            );
        }
    }
}
