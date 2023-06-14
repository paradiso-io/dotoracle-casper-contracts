#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use contract::contract_api::runtime;
use types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};

#[no_mangle]
pub extern "C" fn call() {
    let nft_package_hash: Key = runtime::get_named_arg("nft_package_hash");
    let token_ids: Vec<u64> = runtime::get_named_arg("token_ids");
    let bridge_contract_hash: Key = runtime::get_named_arg("bridge_contract_hash");
    let to_chainid: U256 = runtime::get_named_arg("to_chainid");
    let identifier_mode: u8 = runtime::get_named_arg("identifier_mode");
    let receiver_address: String = runtime::get_named_arg("receiver_address");
    // first approve
    let _: () = runtime::call_versioned_contract(
        ContractPackageHash::new(nft_package_hash.into_hash().unwrap()),
        None,
        "set_approval_for_all",
        runtime_args! {
            "approve_all" => true,
            "operator" => bridge_contract_hash
        },
    );

    let _: () = runtime::call_contract(
        bridge_contract_hash.into_hash().unwrap().into(),
        "request_bridge_nft",
        runtime_args! {
            "nft_package_hash" => nft_package_hash,
            "token_ids" => token_ids,
            "to_chainid" => to_chainid,
            "identifier_mode" => identifier_mode,
            "receiver_address" => receiver_address
        },
    );
}
