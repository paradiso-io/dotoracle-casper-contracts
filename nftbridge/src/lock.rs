use crate::constants::*;
use crate::error::Error;
use crate::helpers::{self, *};
use casper_contract::contract_api::{runtime, storage};

pub fn when_not_locked() {
    let locked: bool = helpers::get_key(IS_LOCKED).unwrap();
    require(!locked, Error::ContractLocked);
}

pub fn lock_contract() {
    helpers::set_key(IS_LOCKED, true);
}

pub fn unlock_contract() {
    helpers::set_key(IS_LOCKED, false);
}

pub fn init() {
    runtime::put_key(IS_LOCKED, storage::new_uref(false).into());
}
