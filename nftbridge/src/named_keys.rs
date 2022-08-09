use casper_types::{contracts::NamedKeys, Key};
use alloc::string::{String, ToString};
use casper_contract::{
    contract_api::{storage}
};

use crate::constants::*;

pub fn default(
    nft_bridge_contract_name: String,
    contract_owner: Key) -> NamedKeys {
    
    let mut named_keys = NamedKeys::new();

    // Contract 'Named keys'

    // 0. Name of the Stake contract
    let nft_bridge_contract_name_key = {
        let nft_bridge_contract_name_uref = storage::new_uref(nft_bridge_contract_name).into_read();
        Key::from(nft_bridge_contract_name_uref)
    };
    named_keys.insert(NFT_BRIDGE_CONTRACT_KEY_NAME.to_string(), nft_bridge_contract_name_key);
    named_keys.insert(CONTRACT_OWNER_KEY_NAME.to_string(), contract_owner);

    named_keys
}