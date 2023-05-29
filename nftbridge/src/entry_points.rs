use alloc::{boxed::Box, string::String, vec};

use crate::constants::*;

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

fn request_bridge_nft() -> EntryPoint {
    EntryPoint::new(
        String::from(REQUEST_BRIDGE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_NFT_PACKAGE_HASH, CLType::Key),
            Parameter::new(ARG_TO_CHAINID, U256::cl_type()),
            Parameter::new(ARG_REQUEST_ID, String::cl_type()),
            Parameter::new(ARG_IDENTIFIER_MODE, u8::cl_type()),
            Parameter::new(ARG_TOKEN_IDS, CLType::List(Box::new(CLType::U64))),
            Parameter::new(ARG_TOKEN_HASHES, CLType::List(Box::new(CLType::String))),
            Parameter::new(ARG_RECEIVER_ADDRESS, String::cl_type()),
        ],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn approve_to_unlock_nft() -> EntryPoint {
    EntryPoint::new(
        String::from(APPROVE_TO_UNLOCK_NFT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_TOKEN_IDS, CLType::List(Box::new(CLType::U64))),
            Parameter::new(ARG_TOKEN_HASHES, CLType::List(Box::new(CLType::String))),
            Parameter::new(ARG_FROM_CHAINID, U256::cl_type()),
            Parameter::new(ARG_IDENTIFIER_MODE, u8::cl_type()),
            Parameter::new(ARG_NFT_PACKAGE_HASH, CLType::Key),
            Parameter::new(ARG_TARGET_KEY, CLType::Key),
            Parameter::new(ARG_UNLOCK_ID, String::cl_type()),
        ],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn claim_unlock_nft() -> EntryPoint {
    EntryPoint::new(
        String::from(CLAIM_UNLOCK_NFT_ENTRY_POINT_NAME),
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn transfer_owner() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_OWNER_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_CONTRACT_OWNER, CLType::Key)],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn transfer_dev() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_DEV_ENTRY_POINT_NAME),
        vec![Parameter::new(ARG_NEW_DEV, CLType::Key)],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(INIT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_CONTRACT_HASH, CLType::Key),
            Parameter::new(ARG_CONTRACT_PACKAGE_HASH, CLType::Key),
            Parameter::new(ARG_CONTRACT_OWNER, CLType::Key),
            Parameter::new(DEV, CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
fn set_supported_token() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_SUPPORTED_TOKEN_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_IS_SUPPORTED_TOKEN, CLType::Bool),
            Parameter::new(ARG_SUPPORTED_TOKEN, CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub(crate) fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(request_bridge_nft());
    entry_points.add_entry_point(transfer_dev());
    entry_points.add_entry_point(approve_to_unlock_nft());
    entry_points.add_entry_point(transfer_owner());
    entry_points.add_entry_point(set_supported_token());
    entry_points.add_entry_point(claim_unlock_nft());
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(EntryPoint::new(
        String::from("update_contract_hash_after_upgrade"),
        vec![
            Parameter::new(ARG_CONTRACT_HASH, CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
