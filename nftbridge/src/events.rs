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
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::ToBytes;

use casper_contract::contract_api::storage;
use casper_types::{ContractPackageHash, Key, URef, U256};

use crate::constants::ARG_CONTRACT_PACKAGE_HASH;
use crate::helpers::*;

pub enum NftBridgeEvent {
    RequestBridgeNft {
        nft_contract: Key,
        token_ids: Vec<String>,
        from: String,
        to: String,
        request_id: String,
        request_index: U256,
        to_chainid: U256,
    },
    UnlockNft {
        nft_contract: Key,
        token_ids: Vec<String>,
        from: String,
        to: String,
        unlock_id: String,
        from_chainid: U256,
    },
}

impl NftBridgeEvent {
    pub fn type_name(&self) -> String {
        match self {
            NftBridgeEvent::RequestBridgeNft {
                nft_contract: _,
                token_ids: _,
                from: _,
                to: _,
                request_id: _,
                request_index: _,
                to_chainid: _,
            } => "request_bridge_nft",
            NftBridgeEvent::UnlockNft {
                nft_contract: _,
                token_ids: _,
                from: _,
                to: _,
                unlock_id: _,
                from_chainid: _,
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
            token_ids,
            from,
            to,
            request_id,
            request_index,
            to_chainid,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("nft_contract", nft_contract.to_string());
            event.insert(
                "token_ids",
                hex::encode(token_ids.to_bytes().unwrap_or_revert()),
            );
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("request_id", request_id.to_string());
            event.insert("request_index", request_index.to_string());
            event.insert("to_chainid", to_chainid.to_string());
            events.push(event);
        }

        NftBridgeEvent::UnlockNft {
            nft_contract,
            token_ids,
            from,
            to,
            unlock_id,
            from_chainid,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("nft_contract", nft_contract.to_string());
            event.insert(
                "token_ids",
                hex::encode(token_ids.to_bytes().unwrap_or_revert()),
            );
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("unlock_id", unlock_id.to_string());
            event.insert("from_chainid", from_chainid.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
