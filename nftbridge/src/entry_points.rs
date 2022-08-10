use alloc::{string::String, vec, boxed::Box};

use crate::constants::*;

use casper_types::{
    U256, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter
    };

fn request_bridge_nft() -> EntryPoint {
    EntryPoint::new(
        String::from(REQUEST_BRIDGE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_TOKEN_IDS, CLType::List(Box::new(CLType::U64))),
            Parameter::new(ARG_TOKEN_HASHES, CLType::List(Box::new(CLType::String))),
            Parameter::new(ARG_TO_CHAINID, U256::cl_type()),
            Parameter::new(ARG_IDENTIFIER_MODE, u8::cl_type()),
            Parameter::new(ARG_NFT_CONTRACT_HASH, String::cl_type()),
            Parameter::new(ARG_RECEIVER_ADDRESS, String::cl_type()),
            Parameter::new(ARG_REQUEST_ID, String::cl_type()),
            ],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn unlock_nft() -> EntryPoint {
    EntryPoint::new(
        String::from(UNLOCK_NFT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_TOKEN_IDS, CLType::List(Box::new(CLType::U64))),
            Parameter::new(ARG_TOKEN_HASHES, CLType::List(Box::new(CLType::String))),
            Parameter::new(ARG_FROM_CHAINID, U256::cl_type()),
            Parameter::new(ARG_IDENTIFIER_MODE, u8::cl_type()),
            Parameter::new(ARG_NFT_CONTRACT_HASH, String::cl_type()),
            Parameter::new(ARG_TARGET_KEY, CLType::Key),
            Parameter::new(ARG_UNLOCK_ID, String::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn transfer_owner() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_OWNER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ARG_CONTRACT_OWNER, CLType::Key)
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub(crate) fn default() -> EntryPoints {
    
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(request_bridge_nft());
    entry_points.add_entry_point(unlock_nft());
    entry_points.add_entry_point(transfer_owner());
    entry_points
    
}
