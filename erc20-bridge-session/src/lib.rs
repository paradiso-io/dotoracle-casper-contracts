#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use contract::contract_api::runtime;
use types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};

#[no_mangle]
pub extern "C" fn call() {
    let erc20_contract_package_hash: Key = runtime::get_named_arg("erc20_contract_package_hash");
    let amount: U256 = runtime::get_named_arg("amount");
    let bridge_package_hash: Key = runtime::get_named_arg("bridge_package_hash");
    let receiver_address: String = runtime::get_named_arg("receiver_address");
    // first approve
    let _: () = runtime::call_versioned_contract(
        ContractPackageHash::new(erc20_contract_package_hash.into_hash().unwrap()),
        None,
        "approve",
        runtime_args! {
            "spender" => bridge_package_hash,
            "amount" => amount
        },
    );

    let _: () = runtime::call_versioned_contract(
        bridge_package_hash.into_hash().unwrap().into(),
        None,
        "request_bridge_erc20",
        runtime_args! {
            "erc20_contract_package_hash" => erc20_contract_package_hash,
            "amount" => amount,
            "receiver_address" => receiver_address
        },
    );
}
