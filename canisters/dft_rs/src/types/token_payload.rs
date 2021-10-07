use super::{MetaData, TokenHolder, Txs};
use candid::Principal;
use ic_cdk::export::{
    candid::{CandidType, Deserialize, Nat}
};
use std::string::String;

#[derive(CandidType, Debug, Deserialize)]
pub struct TokenPayload {
    pub owner: Principal,
    pub fee_to: TokenHolder,
    pub meta: MetaData,
    pub extend: Vec<(String, String)>,
    pub logo: Vec<u8>,
    pub balances: Vec<(TokenHolder, Nat)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, Nat)>)>,
    pub tx_id_cursor: Nat,
    pub storage_canister_ids: Vec<(Nat, Principal)>,
    pub txs_inner: Txs,
}