extern crate alloc;

use alloc::{string::String, vec::Vec};
use casper_types::Key;
use casper_types_derive::{CLTyped, FromBytes, ToBytes};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes)]
pub(crate) struct ApproveUnlock {
    pub identifier_mode: u8,
    pub nft_contract_hash: Key,
    pub token_ids: Vec<u64>,
    pub token_hashes: Vec<String>,
    pub unlock_id: String,
}
