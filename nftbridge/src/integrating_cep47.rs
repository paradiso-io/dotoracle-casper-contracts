use crate::constants::*;
use crate::error::Error;
use crate::helpers::*;
use alloc::{
    string::{String, ToString},
    vec,
    vec::*,
};
use serde::{Deserialize, Serialize};

use casper_contract::{
    contract_api::{
        runtime,
        // runtime::print,
        storage,
        system::{transfer_from_purse_to_account, transfer_from_purse_to_purse},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr, bytesrepr::FromBytes, bytesrepr::ToBytes, contracts::NamedKeys, runtime_args,
    CLType, CLTyped, ContractHash, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, HashAddr, Key, RuntimeArgs, URef, U256,
};
use core::convert::TryFrom;

//use events::MarketPlaceEvent;
pub const NFT_SUPPORT_TYPES: &str = "nft_support_types";
#[repr(u8)]
pub enum NftSupportTypes {
    Cep47IdIsString = 0,
    Cep47IdIsU256 = 1,
    Cep78 = 2,
}
impl TryFrom<u8> for NftSupportTypes {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NftSupportTypes::Cep47IdIsString),
            1 => Ok(NftSupportTypes::Cep47IdIsU256),
            2 => Ok(NftSupportTypes::Cep78),
            _ => Err(Error::InvalidNftSupportTypes),
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct GenericTokenId {
    pub mode: u8,
    pub token_identifier: String,
}

impl ToBytes for GenericTokenId {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = bytesrepr::allocate_buffer(self)?;
        result.extend(self.mode.to_bytes()?);
        result.extend(self.token_identifier.to_bytes()?);
        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.mode.serialized_length() + self.token_identifier.serialized_length()
    }
}

impl FromBytes for GenericTokenId {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (mode, remainder) = u8::from_bytes(bytes)?;
        let (token_identifier, remainder) = String::from_bytes(remainder)?;
        Ok((
            GenericTokenId {
                mode,
                token_identifier,
            },
            remainder,
        ))
    }
}

impl CLTyped for GenericTokenId {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

pub fn init() {
    storage::new_dictionary(NFT_SUPPORT_TYPES)
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
}

pub fn only_dev() {
    require(
        dev_internal() == get_verified_caller().unwrap_or_revert(),
        Error::InvalidDev,
    );
}
pub fn dev_internal() -> Key {
    let dev_key: Key =
        get_stored_value_with_user_errors::<Key>(DEV, Error::MissingDev, Error::InvalidDev);
    dev_key
}
#[no_mangle]
pub extern "C" fn set_nft_support_types() {
    only_dev();
    let nft_contract: Key = runtime::get_named_arg(ARG_NFT_CONTRACT_HASH);
    let nft_type: u8 = runtime::get_named_arg(ARG_NFT_TYPE);
    let nft_contract_hash_key: String = make_dictionary_item_key_for_contract(&nft_contract);
    write_dictionary_value_from_key(NFT_SUPPORT_TYPES, &nft_contract_hash_key, nft_type);
}

pub fn get_nft_type(nft_contract: &Key) -> u8 {
    let contract_hash_key: String = make_dictionary_item_key_for_contract(nft_contract);
    let nft_type: u8 =
        get_dictionary_value_from_key(NFT_SUPPORT_TYPES, &contract_hash_key).unwrap();
    nft_type
}
pub(crate) fn transfer_from_nft(
    nft_contract_hash: ContractHash,
    from: Key,
    to: Key,
    token_id: &GenericTokenId,
) {
    if token_id.mode == 0 || token_id.mode == 1 {
        // CEP78
        let rt = if token_id.mode == 0 {
            let t: u64 = token_id.token_identifier.parse().unwrap();
            runtime_args! {
                "source_key" => from,
                "target_key" => to,
                "token_id" => t
            }
        } else {
            runtime_args! {
                "source_key" => from,
                "target_key" => to,
                "token_hash" => token_id.token_identifier.clone()
            }
        };

        let _: () = runtime::call_contract(nft_contract_hash, "transfer", rt);
    } else if token_id.mode == 2 {
        // CEP47 with token id as U256
        let token_ids: Vec<U256> = vec![U256::from_dec_str(&token_id.token_identifier).unwrap()];
        let _: () = runtime::call_contract(
            nft_contract_hash,
            "transfer_from",
            runtime_args! {
                "sender" => from,
                "recipient" => to,
                "token_ids" => token_ids
            },
        );
    } else if token_id.mode == 3 {
        // CEP47 with token id as String
        let token_ids: Vec<String> = vec![token_id.token_identifier.clone()];
        let _: () = runtime::call_contract(
            nft_contract_hash,
            "transfer_from",
            runtime_args! {
                "sender" => from,
                "recipient" => to,
                "token_ids" => token_ids
            },
        );
    } else {
        runtime::revert(Error::InvalidTokenMode);
    }
}

pub(crate) fn transfer_from_nfts(
    nft_contract_hash: ContractHash,
    from: Key,
    to: Key,
    token_ids: Vec<GenericTokenId>,
) {
    for token_id in token_ids {
        transfer_from_nft(nft_contract_hash, from, to, &token_id);
    }
}

pub fn entry_points() -> Vec<EntryPoint> {
    vec![EntryPoint::new(
        String::from("set_nft_support_types"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )]
}
