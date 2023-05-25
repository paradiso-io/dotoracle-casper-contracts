use alloc::{format, string::String};
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::NamedKeys, ContractHash, ContractPackageHash, EntryPoints, Key};
pub fn install_contract(
    contract_name: String,
    entry_points: EntryPoints,
    named_keys: NamedKeys,
) -> (ContractHash, ContractPackageHash) {
    let (contract_package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _version) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        contract_package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(contract_package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
    (contract_hash, contract_package_hash)
}
pub fn upgrade_contract(
    contract_name: String,
    entry_points: EntryPoints,
    named_keys: NamedKeys,
    disable_older_version_or_not: bool,
) {
    let package_hash: ContractPackageHash =
        runtime::get_key(&format!("{}_package_hash", contract_name))
            .unwrap_or_revert()
            .into_hash()
            .unwrap()
            .into();
    // get old_version_contract_hash that should be disable after upgrade
    let should_be_disable: Key =
        runtime::get_key(&format!("{}_contract_hash", contract_name)).unwrap();

    let (contract_hash, _) = storage::add_contract_version(package_hash, entry_points, named_keys);
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    if disable_older_version_or_not {
        storage::disable_contract_version(
            package_hash,
            should_be_disable.into_hash().unwrap_or_revert().into(),
        )
        .unwrap_or_revert();
    }
}
