#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// use std::collections::BTreeMap;

extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::*,
};

use casper_contract::contract_api::storage;
use casper_types::{ContractPackageHash, Key, URef, U256};

use crate::constants::ARG_CONTRACT_PACKAGE_HASH;
use crate::helpers::*;

pub enum NftBridgeEvent {
    RequestBridgeNft {
        nft_contract: Key,
        token_id: String,
        from: String,
        to: String,
        request_id: String,
        request_index: U256,
    },
    UnlockNft {
        nft_contract: Key,
        token_id: String,
        from: String,
        to: String,
        unlock_id: String,
    },
}

impl NftBridgeEvent {
    pub fn type_name(&self) -> String {
        match self {
            NftBridgeEvent::RequestBridgeNft {
                nft_contract: _,
                token_id: _,
                from: _,
                to: _,
                request_id: _,
                request_index: _,
            } => "request_bridge_nft",
            NftBridgeEvent::UnlockNft {
                nft_contract: _,
                token_id: _,
                from: _,
                to: _,
                unlock_id: _,
            } => "unlock_nft",
        }
        .to_string()
    }
}

pub fn contract_package_hash() -> ContractPackageHash {
    log_msg("get key");
    let pk: Key = get_key(ARG_CONTRACT_PACKAGE_HASH).unwrap();
    ContractPackageHash::new(pk.into_hash().unwrap())
}

pub(crate) fn emit(pair_event: &NftBridgeEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    log_msg("log");
    match pair_event {
        NftBridgeEvent::RequestBridgeNft {
            nft_contract,
            token_id,
            from,
            to,
            request_id,
            request_index,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("nft_contract", nft_contract.to_string());
            event.insert("token_id", token_id.to_string());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("request_id", request_id.to_string());
            event.insert("request_index", request_id.to_string());
            events.push(event);
        }

        NftBridgeEvent::UnlockNft {
            nft_contract,
            token_id,
            from,
            to,
            unlock_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("nft_contract", nft_contract.to_string());
            event.insert("token_id", token_id.to_string());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("unlock_id", unlock_id.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
