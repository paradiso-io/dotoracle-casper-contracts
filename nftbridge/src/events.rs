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
    ApproveUnlockNft {
        unlock_id: String,
    },
    ClaimUnlockNft {
        token_owner: Key,
        unlock_ids: Vec<String>,
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
            NftBridgeEvent::ApproveUnlockNft { unlock_id: _ } => "approve_unlock_nft",
            NftBridgeEvent::ClaimUnlockNft {
                token_owner: _,
                unlock_ids: _,
            } => "claim_unlock_nft",
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

        NftBridgeEvent::ApproveUnlockNft { unlock_id } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert("unlock_id", unlock_id.to_string());
            events.push(event);
        }

        NftBridgeEvent::ClaimUnlockNft {
            token_owner,
            unlock_ids,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", pair_event.type_name());
            event.insert(
                "token_owner",
                encode_dictionary_item_key(token_owner.clone()),
            );
            event.insert(
                "unlock_ids",
                hex::encode(unlock_ids.to_bytes().unwrap_or_revert()),
            );

            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
