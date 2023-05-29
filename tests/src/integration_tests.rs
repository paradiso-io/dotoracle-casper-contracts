use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};

use casper_types::{
    runtime_args, ContractPackageHash, Key, RuntimeArgs, account::AccountHash, ContractHash, U256
};

#[derive(Copy, Clone)]
struct TestContext {
    nft_package_hash: Key,
    nft_bridge_contract_hash: Key,
    nft_bridge: Key
}
fn exec_call(
    builder: &mut InMemoryWasmTestBuilder,
    account_hash: AccountHash,
    contract_package_hash: Key,
    fun_name: &str,
    args: RuntimeArgs,
    expect_success: bool,
) {
    let request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        account_hash,
        contract_package_hash.into_hash().unwrap().into(),
        None,
        fun_name,
        args,
    )
    .build();
    if expect_success {
        builder.exec(request).expect_success().commit();
    } else {
        builder.exec(request).expect_failure();
    }
}

fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

    let deploy_nft = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        "cep78.wasm",
        runtime_args! {
            "collection_name" => "cn".to_string(),
            "collection_symbol" => "cb".to_string(),
            "total_token_supply" => 10000u64,
            "allow_minting" => true,
            "ownership_mode" => 2u8,
            "nft_kind" => 1u8,
            "holder_mode" => 2u8,
            "whitelist_mode" => 0u8,
            "json_schema" => "",
            "receipt_name" => "",
            "identifier_mode" => 0u8,
            "nft_metadata_kind" => 0u8,
            "metadata_mutability" => 0u8,
            "owner_reverse_lookup_mode" =>1u8,
            "events_mode" => 0u8,
            "the_contract_owner" => Key::from(*DEFAULT_ACCOUNT_ADDR),
            "the_contract_minter" => Key::from(*DEFAULT_ACCOUNT_ADDR)
        },
    )
    .build();
    
    builder.exec(deploy_nft).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let nft_package_hash = account
        .named_keys()
        .get("cep78_contract_package_cn")
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract hash");

    let nft_package_hash = Key::from(nft_package_hash);

    // mint nft 
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, nft_package_hash, "mint", runtime_args! {
        "count" => 10u64,
        "token_owner" => Key::from(*DEFAULT_ACCOUNT_ADDR)
    }, true);
    
    let deploy_bridge = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        "contract.wasm",
        runtime_args! {
            "contract_name" => "nft_bridge".to_string(),
            "contract_owner" => Key::from(*DEFAULT_ACCOUNT_ADDR),
            "dev" => Key::from(*DEFAULT_ACCOUNT_ADDR)
        },
    )
    .build();
    
    builder.exec(deploy_bridge).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let bridge_package_hash = account
        .named_keys()
        .get("nft_bridge_package_hash")
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract hash");

    let bridge_contract_hash = account
        .named_keys()
        .get("nft_bridge_contract_hash")
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let bridge_package_hash = Key::from(bridge_package_hash);
    let bridge_contract_hash = Key::from(bridge_contract_hash);

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, bridge_package_hash, "set_supported_token", runtime_args! {
        "supported_token" => nft_package_hash,
        "is_supported_token" => true
    }, true);

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, nft_package_hash, "set_approval_for_all", runtime_args! {
        "operator" => bridge_contract_hash,
        "approve_all" => true
    }, true);

    let tc = TestContext {
        nft_package_hash,
        nft_bridge: bridge_package_hash,
        nft_bridge_contract_hash: bridge_contract_hash
    };

    (builder, tc)
}

#[test]
fn test_unlock() {
    let (mut builder, tc) = setup();

    // request bridge
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.nft_bridge, "request_bridge_nft", runtime_args! {
        "nft_package_hash" => tc.nft_package_hash,
        "to_chainid" => U256::from(43113u64),
        "identifier_mode" => 0u8,
        "request_id" => "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
        "token_ids" => vec![1u64],
        "receiver_address" => "0xbf26a30547a7dda6e86fc3C33396F28FFf6902c3".to_string()
    }, true);

    let mut unlock_id = "0x7788d03de297137446ae4d66a5630d40064e8ec398305c7189f717e4b41914e2-43113-96945816564243-93-".to_string() + &hex::encode(tc.nft_package_hash.into_hash().unwrap());
    unlock_id = unlock_id + "-96945816564243";
    // approve for unlock
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.nft_bridge, "approve_to_unlock_nft", runtime_args! {
        "target_key" => Key::from(*DEFAULT_ACCOUNT_ADDR),
        "unlock_id" => unlock_id,
        "token_ids" => vec![1u64],
        "from_chainid" => U256::from(43113u64),
        "identifier_mode" => 0u8,
        "nft_package_hash" => tc.nft_package_hash
    }, true);

    // claim
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.nft_bridge, "claim_unlock_nft", runtime_args! {
    }, true);

}
