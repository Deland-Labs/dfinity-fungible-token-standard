use candid::candid_method;
use dft_standard::auto_scaling_storage::exec_auto_scaling_strategy;
use dft_standard::state::TOKEN;
use dft_types::*;
use ic_cdk::{api, export::candid::Nat};
use ic_cdk_macros::*;
use std::string::String;

use crate::token::BurnableExtension;

#[update(name = "burnFrom")]
#[candid_method(update, rename = "burnFrom")]
async fn burn_from(
    from_sub_account: Option<Subaccount>,
    owner: String,
    value: Nat,
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let spender = TokenHolder::new(caller, from_sub_account);
    let owner_parse_res = owner.parse::<TokenHolder>();
    match owner_parse_res {
        Ok(owner_holder) => {
            match TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.burn_from(
                    &caller,
                    &owner_holder,
                    &spender,
                    value.clone(),
                    created_at,
                    api::time(),
                )
            }) {
                Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                    tx_id: hex::encode(tx_hash.as_ref()),
                    block_height,
                    error: match exec_auto_scaling_strategy().await {
                        Ok(_) => None,
                        Err(e) => Some(e.into()),
                    },
                },
                Err(e) => OperationResult::Err(e.into()),
            }
        }

        Err(_) => OperationResult::Err(DFTError::InvalidSpender.into()),
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(
    from_sub_account: Option<Subaccount>,
    value: Nat,
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    match TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.burn(
            &caller,
            &transfer_from,
            value.clone(),
            created_at,
            api::time(),
        )
    }) {
        Ok((block_height, _, tx_hash)) => OperationResult::Ok {
            tx_id: hex::encode(tx_hash.as_ref()),
            block_height,
            error: match exec_auto_scaling_strategy().await {
                Ok(_) => None,
                Err(e) => Some(e.into()),
            },
        },
        Err(e) => OperationResult::Err(e.into()),
    }
}
