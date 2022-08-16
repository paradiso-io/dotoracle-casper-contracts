use alloc::string::{String, ToString};
use casper_contract::contract_api::storage;
use casper_types::{contracts::NamedKeys, Key, U256};

use crate::constants::*;
pub fn default(
    nft_bridge_contract_name: String,
    contract_owner: Key,
    fee_token: Option<Key>,
) -> NamedKeys {
    let mut named_keys = NamedKeys::new();

    // Contract 'Named keys'
    named_keys.insert(
        NFT_BRIDGE_CONTRACT_KEY_NAME.to_string(),
        Key::from(storage::new_uref(nft_bridge_contract_name.to_string()).into_read()),
    );
    named_keys.insert(
        CONTRACT_OWNER_KEY_NAME.to_string(),
        Key::from(storage::new_uref(contract_owner)),
    );
    named_keys.insert(
        REQUEST_INDEX.to_string(),
        Key::from(storage::new_uref(U256::zero())),
    );
    if fee_token.is_some() {
        named_keys.insert(
            FEE_TOKEN_KEY_NAME.to_string(),
            Key::from(storage::new_uref(fee_token.unwrap())),
        );
    }

    named_keys
}
