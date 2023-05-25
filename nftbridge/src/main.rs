#![no_main]
#![no_std]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

mod address;
pub mod constants;
mod entry_points;
mod error;
mod events;
mod helpers;
mod named_keys;
mod upgrade;

use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::error::Error;
use crate::helpers::*;
use alloc::{
    format,
    string::{String, ToString},
    vec::*,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ContractHash, HashAddr, Key, RuntimeArgs, URef, U256,
};
use casper_types_derive::{FromBytes, ToBytes};

use events::NftBridgeEvent;
use helpers::get_immediate_caller_key;

#[derive(Serialize, Deserialize, Clone, ToBytes, FromBytes)]
pub(crate) struct RequestBridge {
    nft_contract_hash: Key,
    identifier_mode: u8,
    request_id: String,
    to_chainid: U256,
    request_index: U256,
    from: Key,
    to: String,
    token_ids: Vec<u64>,
    token_hashes: Vec<String>,
}

#[no_mangle]
pub extern "C" fn init() {
    if get_key::<Key>(CONTRACT_HASH_KEY_NAME).is_some() {
        runtime::revert(Error::ContractAlreadyInitialized);
    }
    let contract_hash: Key = runtime::get_named_arg(ARG_CONTRACT_HASH);
    set_key(CONTRACT_HASH_KEY_NAME, contract_hash);
    let contract_package_hash: Key = runtime::get_named_arg(ARG_CONTRACT_PACKAGE_HASH);
    set_key(ARG_CONTRACT_PACKAGE_HASH, contract_package_hash);
    let dev: Key = runtime::get_named_arg(DEV);
    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);
    named_keys::default(contract_owner, dev);

    storage::new_dictionary(REQUEST_IDS).unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary(WRAPPED_TOKEN).unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary(UNLOCK_IDS)
        .unwrap_or_revert_with(Error::FailedToCreateDictionaryUnlockIds);
    storage::new_dictionary(SUPPORTED_TOKEN).unwrap_or_revert_with(Error::FailedToCreateDictionary);
}

#[no_mangle]
fn call() {
    let contract_name: String = runtime::get_named_arg("contract_name");
    let dev: Key = runtime::get_named_arg(DEV);
    let contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);
    let disable_older_version_or_not: bool = runtime::get_named_arg("disable_older_version_or_not");

    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        let (contract_hash, contract_package_hash) =
            upgrade::install_contract(contract_name, entry_points::default(), NamedKeys::new());

        runtime::call_contract::<()>(
            contract_hash,
            INIT_ENTRY_POINT_NAME,
            runtime_args! {
                "contract_hash" => Key::from(contract_hash),
                "contract_package_hash" => Key::from(contract_package_hash),
                "contract_owner" => Key::from(contract_owner),
                "dev" => Key::from(dev),
            },
        );
    } else {
        upgrade::upgrade_contract(
            contract_name,
            entry_points::default(),
            NamedKeys::new(),
            disable_older_version_or_not,
        );
    }
}

#[no_mangle]
pub extern "C" fn request_bridge_nft() {
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    let to_chainid: U256 = runtime::get_named_arg(ARG_TO_CHAINID);

    let contract_hash_dictionary_key: String = make_dictionary_item_key_for_contract(contract_hash);
    // check as if token is wrapped token => revert
    if get_dictionary_value_from_key::<bool>(WRAPPED_TOKEN, &contract_hash_dictionary_key).is_some()
    {
        let is_wrapped_token_value =
            get_dictionary_value_from_key::<bool>(WRAPPED_TOKEN, &contract_hash_dictionary_key)
                .unwrap_or_revert();

        if is_wrapped_token_value == true {
            runtime::revert(Error::InvalidWrappedToken);
        }
    }

    // check as if token is supported

    let supported_token_item =
        get_dictionary_value_from_key::<bool>(SUPPORTED_TOKEN, &contract_hash_dictionary_key);
    let supported = if supported_token_item.is_some() && supported_token_item.unwrap_or_revert() {
        true
    } else {
        false
    };
    if !supported {
        runtime::revert(Error::InvalidSupportedToken);
    }

    let identifier_mode = get_identifier_mode_from_runtime_args();
    let user = get_immediate_caller_key();
    if user.into_account().is_none() {
        runtime::revert(Error::CallerMustBeAccountHash);
    }

    // Transfer nft to this bridge contract hash - NOT the bridge contract package hash
    // let self_key = get_self_key();
    let this_bridge_contract_hash = helpers::get_stored_value_with_user_errors::<Key>(
        CONTRACT_HASH_KEY_NAME,
        Error::MissingBridgeContractHash,
        Error::InvalidBridgeContractHash,
    );

    let request_id: String = runtime::get_named_arg(ARG_REQUEST_ID);
    if request_id.chars().count() != 64 {
        runtime::revert(Error::RequestIdIlledFormat);
    }
    let decoded = hex::decode(&request_id);
    if decoded.is_err() {
        runtime::revert(Error::RequestIdIlledFormat);
    }
    if request_id.len() != 64 || decoded.unwrap().len() != 32 {
        runtime::revert(Error::RequestIdIlledFormat);
    }

    if get_dictionary_value_from_key::<String>(REQUEST_IDS, &request_id).is_some() {
        runtime::revert(Error::RequestIdRepeated);
    }

    let token_identifiers = get_token_identifiers_from_runtime_args(&identifier_mode);
    if token_identifiers.len() > 10 {
        runtime::revert(Error::TooManyTokenIds);
    }

    let mut current_request_index: U256 = get_key(REQUEST_INDEX).unwrap();
    current_request_index = current_request_index.checked_add(U256::one()).unwrap();

    let receiver_address = runtime::get_named_arg(ARG_RECEIVER_ADDRESS);

    let request_bridge_data = RequestBridge {
        nft_contract_hash: contract_hash,
        identifier_mode: identifier_mode as u8,
        request_id: request_id.clone(),
        to_chainid: to_chainid.clone(),
        request_index: current_request_index,
        from: user.clone(),
        to: receiver_address,
        token_ids: match identifier_mode {
            NFTIdentifierMode::Ordinal => get_named_arg_with_user_errors::<Vec<u64>>(
                ARG_TOKEN_IDS,
                Error::MissingTokenID,
                Error::InvalidTokenIdentifier,
            )
            .unwrap_or_revert(),
            NFTIdentifierMode::Hash => Vec::new(),
        },
        token_hashes: match identifier_mode {
            NFTIdentifierMode::Hash => get_named_arg_with_user_errors::<Vec<String>>(
                ARG_TOKEN_HASHES,
                Error::MissingTokenID,
                Error::InvalidTokenIdentifier,
            )
            .unwrap_or_revert(),
            NFTIdentifierMode::Ordinal => Vec::new(),
        },
    };

    let token_ids_to_tring: Vec<String> = request_bridge_data
        .token_ids
        .clone()
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let json_request_data = casper_serde_json_wasm::to_string(&request_bridge_data).unwrap();

    write_dictionary_value_from_key(REQUEST_IDS, &request_id, json_request_data);

    set_key(REQUEST_INDEX, current_request_index);

    // Transfer NFT to the bridge contract_hash (NOT the bridge contract_package_hash)
    cep78_transfer_from(
        &contract_hash,
        user,
        this_bridge_contract_hash,
        identifier_mode,
        token_identifiers,
    );
    events::emit(&NftBridgeEvent::RequestBridgeNft {
        nft_contract: contract_hash.clone(),
        token_ids: match identifier_mode {
            NFTIdentifierMode::Ordinal => token_ids_to_tring.clone(),
            NFTIdentifierMode::Hash => request_bridge_data.token_hashes.clone(),
        },
        from: user.clone().to_formatted_string(),
        to: request_bridge_data.to.clone().to_string(),
        request_id: request_id.clone(),
        request_index: current_request_index.clone(),
        to_chainid: to_chainid.clone(),
    });
}

#[no_mangle]
pub extern "C" fn set_wrapped_token() {
    let wrapped_token: Key = runtime::get_named_arg(ARG_WRAPPED_TOKEN);
    let is_wrapped_token: bool = runtime::get_named_arg(ARG_IS_WRAPPED_TOKEN);
    // let dev = runtime::get_key(DEV).unwrap_or_revert();
    // let caller = helpers::get_verified_caller().unwrap_or_revert();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    let current_dev = helpers::get_stored_value_with_user_errors::<Key>(
        DEV,
        Error::MissingDev,
        Error::InvalidDev,
    );

    if caller != current_dev {
        runtime::revert(Error::InvalidDev);
    }

    let wrapped_token_dictionary_key: String = make_dictionary_item_key_for_contract(wrapped_token);

    write_dictionary_value_from_key(
        WRAPPED_TOKEN,
        &wrapped_token_dictionary_key,
        is_wrapped_token,
    );
}

#[no_mangle]
pub extern "C" fn set_supported_token() {
    let supported_token: Key = runtime::get_named_arg(ARG_SUPPORTED_TOKEN);
    let is_supported_token: bool = runtime::get_named_arg(ARG_IS_SUPPORTED_TOKEN);
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    let current_dev = helpers::get_stored_value_with_user_errors::<Key>(
        DEV,
        Error::MissingDev,
        Error::InvalidDev,
    );

    if caller != current_dev {
        runtime::revert(Error::InvalidDev);
    }

    // register owner of NFT for this bridge contract
    if is_supported_token {
        let this_bridge_contract_hash = helpers::get_stored_value_with_user_errors::<Key>(
            CONTRACT_HASH_KEY_NAME,
            Error::MissingBridgeContractHash,
            Error::InvalidBridgeContractHash,
        );
        let _: (String, URef) = runtime::call_contract(
            ContractHash::new(supported_token.into_hash().unwrap_or_revert()),
            "register_owner",
            runtime_args! {
                "token_owner" => this_bridge_contract_hash,
            },
        );
    }
    let supported_token_dictionary_key: String =
        make_dictionary_item_key_for_contract(supported_token);

    write_dictionary_value_from_key(
        SUPPORTED_TOKEN,
        &supported_token_dictionary_key,
        is_supported_token,
    );
}

#[no_mangle]
pub extern "C" fn transfer_owner() {
    let new_contract_owner: Key = runtime::get_named_arg(ARG_CONTRACT_OWNER);
    // let current_contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap_or_revert();
    // let caller = helpers::get_verified_caller().unwrap_or_revert();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    let current_contract_owner = helpers::get_stored_value_with_user_errors::<Key>(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );

    if caller != current_contract_owner {
        runtime::revert(Error::InvalidContractOwner);
    }
    set_key(CONTRACT_OWNER_KEY_NAME, new_contract_owner);
}

#[no_mangle]
pub extern "C" fn transfer_dev() {
    let new_dev: Key = runtime::get_named_arg(ARG_NEW_DEV);
    //let current_dev = runtime::get_key(DEV).unwrap_or_revert();
    //let caller = get_immediate_caller_key();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    let current_dev = helpers::get_stored_value_with_user_errors::<Key>(
        DEV,
        Error::MissingDev,
        Error::InvalidDev,
    );

    if caller != current_dev {
        runtime::revert(Error::InvalidDev);
    }
    set_key(DEV, new_dev);
}

#[no_mangle]
pub extern "C" fn unlock_nft() {
    //let caller = get_immediate_caller_key();
    //let contract_owner = runtime::get_key(CONTRACT_OWNER_KEY_NAME).unwrap();
    let caller = helpers::get_verified_caller().unwrap_or_revert();
    let contract_owner = helpers::get_stored_value_with_user_errors::<Key>(
        CONTRACT_OWNER_KEY_NAME,
        Error::MissingContractOwner,
        Error::InvalidContractOwner,
    );
    if caller != contract_owner {
        runtime::revert(Error::InvalidAccount);
    }

    let from_chainid: U256 = runtime::get_named_arg(ARG_FROM_CHAINID);

    let unlock_id: String = runtime::get_named_arg(ARG_UNLOCK_ID);

    //verify unlock id
    let unlock_id_parts: Vec<&str> = unlock_id.split("-").collect();
    if unlock_id_parts.len() != 6 {
        runtime::revert(Error::UnlockIdIllFormatted);
    }

    {
        if unlock_id_parts[0].len() != 66 || !unlock_id_parts[0].starts_with("0x") {
            runtime::revert(Error::UnlockIdIllFormatted);
        }
        let tx_hash_without_prefix = unlock_id_parts[0].replace("0x", "");
        let decoded = hex::decode(&tx_hash_without_prefix);
        if decoded.is_err() {
            runtime::revert(Error::TxHashUnlockIdIllFormatted);
        }
    }

    let unlock_id_key = get_unlock_id_key(&unlock_id);
    if get_dictionary_value_from_key::<bool>(UNLOCK_IDS, &unlock_id_key).is_some() {
        runtime::revert(Error::UnlockIdRepeated);
    }
    write_dictionary_value_from_key(UNLOCK_IDS, &unlock_id_key, true);

    let mut origin_contract_address = "hash-".to_string();
    origin_contract_address.push_str(unlock_id_parts[4]);
    let contract_hash: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    if contract_hash.to_formatted_string() != origin_contract_address {
        runtime::revert(Error::UnlockIdIllFormatted);
    }

    let identifier_mode = get_identifier_mode_from_runtime_args();
    let token_identifiers = get_token_identifiers_from_runtime_args(&identifier_mode);
    if token_identifiers.len() > 10 {
        runtime::revert(Error::TooManyTokenIds);
    }
    let target: Key = runtime::get_named_arg(ARG_TARGET_KEY);

    // let self_key = get_self_key();
    let this_bridge_contract_hash = helpers::get_stored_value_with_user_errors::<Key>(
        CONTRACT_HASH_KEY_NAME,
        Error::MissingBridgeContractHash,
        Error::InvalidBridgeContractHash,
    );

    cep78_transfer_from(
        &contract_hash,
        this_bridge_contract_hash,
        target,
        identifier_mode,
        token_identifiers,
    );

    let token_ids = match identifier_mode {
        NFTIdentifierMode::Ordinal => get_named_arg_with_user_errors::<Vec<u64>>(
            ARG_TOKEN_IDS,
            Error::MissingTokenID,
            Error::InvalidTokenIdentifier,
        )
        .unwrap_or_revert(),
        NFTIdentifierMode::Hash => Vec::new(),
    };
    let token_hashes = match identifier_mode {
        NFTIdentifierMode::Hash => get_named_arg_with_user_errors::<Vec<String>>(
            ARG_TOKEN_HASHES,
            Error::MissingTokenID,
            Error::InvalidTokenIdentifier,
        )
        .unwrap_or_revert(),
        NFTIdentifierMode::Ordinal => Vec::new(),
    };
    let token_ids_to_tring: Vec<String> = token_ids.into_iter().map(|x| x.to_string()).collect();

    events::emit(&NftBridgeEvent::UnlockNft {
        nft_contract: contract_hash.clone(),
        token_ids: match identifier_mode {
            NFTIdentifierMode::Ordinal => token_ids_to_tring.clone(),
            NFTIdentifierMode::Hash => token_hashes,
        },
        from: caller.clone().to_formatted_string(),
        to: target.clone().to_formatted_string(),
        unlock_id: unlock_id.clone(),
        from_chainid: from_chainid.clone(),
    });
}

fn cep78_transfer_from(
    contract_hash: &Key,
    source: Key,
    target: Key,
    identifier_mode: NFTIdentifierMode,
    token_identifiers: Vec<TokenIdentifier>,
) {
    let contract_hash_addr: HashAddr = contract_hash.into_hash().unwrap_or_revert();
    let contract_hash: ContractHash = ContractHash::new(contract_hash_addr);
    // let self_address = Address::Account(AccountHash::from_formatted_str("account-hash-32b0eaaa6c0d024e2e7efc34a0a8aad7889cdbb87c71f07cb0eb4f515d5696de").unwrap());
    //let self_key = get_key_from_address(&self_address);
    match identifier_mode {
        NFTIdentifierMode::Ordinal => {
            for token_identifier in token_identifiers {
                let _: (String, Key) = runtime::call_contract(
                    contract_hash,
                    TRANSFER_ENTRY_POINT_NAME,
                    runtime_args! {
                        ARG_SOURCE_KEY => source,
                        ARG_TARGET_KEY => target,
                        ARG_TOKEN_ID => token_identifier.get_index().unwrap()
                    },
                );
            }
        }
        NFTIdentifierMode::Hash => {
            for token_identifier in token_identifiers {
                let _: (String, Key) = runtime::call_contract(
                    contract_hash,
                    TRANSFER_ENTRY_POINT_NAME,
                    runtime_args! {
                        ARG_SOURCE_KEY => source,
                        ARG_TARGET_KEY => target,
                        ARG_TOKEN_HASH => token_identifier.get_hash().unwrap()
                    },
                );
            }
        }
    }
}
