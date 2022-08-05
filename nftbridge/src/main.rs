#![no_main]
#![no_std]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

mod address;
pub mod constants;
mod entry_points;
mod erc20_error;
mod error;
mod helpers;
mod named_keys;
mod utils;

use crate::address::Address;
use crate::erc20_error::Error;
use crate::helpers::{get_immediate_caller_address, get_self_key, set_key};

use crate::constants::*;
use crate::utils::*;
use alloc::string::{String, ToString};
use casper_types::account::AccountHash;
use core::convert::TryFrom;
use helpers::{get_key_from_address, get_self_address};
// use casper_erc20::{ Error, Address,
//     constants::{
//         TRANSFER_ENTRY_POINT_NAME, TRANSFER_FROM_ENTRY_POINT_NAME, OWNER_RUNTIME_ARG_NAME,
//         RECIPIENT_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME}
//     };

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ContractHash, HashAddr, Key, RuntimeArgs, URef, U256,
};

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

    runtime::put_key(contract_hash_key_name.as_str(), Key::from(contract_hash));
}

#[no_mangle]
pub extern "C" fn request_bridge_nft() {
    // TODO Frontend should check 'allowance' of ERC20 'Stake token' contract for user
    // Let user to call 'approve' first, before staking

    let contract_hash: String = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    let identifier_mode_u8: u8 = runtime::get_named_arg(ARG_IDENTIFIER_MODE);
    let identifier_mode =
        NFTIdentifierMode::try_from(identifier_mode_u8).unwrap_or(NFTIdentifierMode::Hash);
    let token_identifier: TokenIdentifier =
        get_token_identifier_from_runtime_args(&identifier_mode);
    let user: Address = get_immediate_caller_address().unwrap_or_revert();
    let user_formated_string = user.as_account_hash().unwrap().to_formatted_string();
    // let balances_key: Key = runtime::get_key(BALANCES_KEY_NAME).unwrap_or_revert();
    // let rewards_key: Key = runtime::get_key(REWARDS_KEY_NAME).unwrap_or_revert();
    // let balances_uref: URef = balances_key.into_uref().unwrap_or_revert();
    // let rewards_uref: URef = rewards_key.into_uref().unwrap_or_revert();

    // update_reward(staker, balances_uref, rewards_uref);

    // // update total_supply
    // named_key_add(amount, TOTAL_SUPPLY_KEY_NAME);

    // // update balance of caller
    // dictionary_add(balances_uref, staker, amount);

    // Transfer `amount` of Stake Token from caller to the stake contract
    cep78_transfer_from(
        &contract_hash,
        Key::from_formatted_str(&user_formated_string).unwrap(),
        identifier_mode,
        token_identifier,
    );
    //U256::one()
}

#[no_mangle]
pub extern "C" fn unlock_nft() {

    // let amount: U256 = runtime::get_named_arg(AMOUNT_KEY_NAME);

    // let staker = get_immediate_caller_address().unwrap_or_revert();
    // let balances_key: Key = runtime::get_key(BALANCES_KEY_NAME).unwrap_or_revert();
    // let rewards_key: Key = runtime::get_key(REWARDS_KEY_NAME).unwrap_or_revert();
    // let balances_uref: URef = balances_key.into_uref().unwrap_or_revert();
    // let rewards_uref: URef = rewards_key.into_uref().unwrap_or_revert();

    // update_reward(staker, balances_uref, rewards_uref);

    // // update total_supply
    // named_key_sub(amount, TOTAL_SUPPLY_KEY_NAME);

    // // update balance of caller
    // dictionary_sub(balances_uref, staker, amount);

    // // Transfer `amount` of Stake Token from the stake contract to caller
    // erc20_transfer(
    //     STAKE_TOKEN_HASH_KEY_NAME,
    //     staker,
    //     amount
    // );

    // get_reward();
}

#[no_mangle]
fn update_reward(staker: Address, balances_uref: URef, rewards_uref: URef) {

    // let current_block_time: U256 = U256::from(u64::from(runtime::get_blocktime()));
    // let user_reward_per_token_paid_key: Key = runtime::get_key(USER_REWARD_PER_TOKEN_PAID_KEY_NAME).unwrap_or_revert();
    // let user_reward_per_token_paid_uref: URef = user_reward_per_token_paid_key.into_uref().unwrap_or_revert();
    // let user_reward_per_token_paid: U256 = dictionary_read(user_reward_per_token_paid_uref, staker);

    // // update reward_per_token_stored
    // let reward_per_token_stored: U256 = reward_per_token(current_block_time);

    // // update last_update_time
    // set_key(LAST_UPDATE_KEY_NAME, current_block_time);

    // // update reward amount of the staker
    // dictionary_add(
    //     rewards_uref,
    //     staker,
    //     earned(staker, balances_uref, user_reward_per_token_paid)
    // );

    // // update "user_reward_per_token_paid" dictionary
    // dictionary_write(user_reward_per_token_paid_uref, staker, reward_per_token_stored);
}

fn cep78_transfer_from(
    contract_hash_str: &str,
    requester: Key,
    identifier_mode: NFTIdentifierMode,
    token_identifier: TokenIdentifier,
) {
    let contract_hash_key = Key::from_formatted_str(contract_hash_str).unwrap();
    let contract_hash_addr: HashAddr = contract_hash_key.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);
    let self_address = Address::Account(AccountHash::from_formatted_str("account-hash-32b0eaaa6c0d024e2e7efc34a0a8aad7889cdbb87c71f07cb0eb4f515d5696de").unwrap());
    let self_key = get_key_from_address(&self_address);
    match identifier_mode {
        NFTIdentifierMode::Ordinal => {
            let _: () = runtime::call_contract(
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
            let _: () = runtime::call_contract(
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
