use candid::candid_method;
use dft_standard::token_service::TokenService;
use dft_types::*;
use ic_cdk::{api, export::Principal};
use ic_cdk_macros::*;
use std::string::String;

#[update(name = "setOwner")]
#[candid_method(update, rename = "setOwner")]
fn set_owner(owner: Principal, created_at: Option<u64>) -> BooleanResult {
    TokenService::default()
        .set_owner(&api::caller(), owner, created_at, api::time())
        .into()
}

#[update(name = "setLogo")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: Option<Vec<u8>>) -> BooleanResult {
    TokenService::default()
        .set_logo(&api::caller(), logo)
        .into()
}

#[update(name = "setDesc")]
#[candid_method(update, rename = "setDesc")]
fn set_desc_info(desc_data: Vec<(String, String)>) -> BooleanResult {
    // convert desc data to hashmap
    let mut desc_info = std::collections::HashMap::new();
    for (key, value) in desc_data {
        desc_info.insert(key, value);
    }
    TokenService::default()
        .set_desc(&api::caller(), desc_info)
        .into()
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: CandidTokenFee, created_at: Option<u64>) -> BooleanResult {
    let caller = api::caller();
    TokenService::default()
        .set_fee(&caller, fee.into(), created_at, api::time())
        .into()
}

#[update(name = "setFeeTo")]
#[candid_method(update, rename = "setFeeTo")]
fn set_fee_to(fee_to: String, created_at: Option<u64>) -> BooleanResult {
    match fee_to.parse::<TokenHolder>() {
        Ok(holder) => TokenService::default()
            .set_fee_to(&api::caller(), holder, created_at, api::time())
            .into(),
        Err(_) => BooleanResult::Err(DFTError::InvalidArgFormatFeeTo.into()),
    }
}
