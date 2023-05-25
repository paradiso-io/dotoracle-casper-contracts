use casper_contract::contract_api::{runtime, storage};
use casper_types::{Key, U256};

use crate::constants::*;
pub fn default(contract_owner: Key, dev: Key) {
    runtime::put_key(
        CONTRACT_OWNER_KEY_NAME,
        storage::new_uref(contract_owner).into(),
    );

    runtime::put_key(DEV, storage::new_uref(dev).into());

    runtime::put_key(REQUEST_INDEX, Key::from(storage::new_uref(U256::zero())));
}
