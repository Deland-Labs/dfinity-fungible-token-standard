use super::{Metadata, TokenHolder, Txs};
use candid::{CandidType, Deserialize, Nat, Principal};
use std::string::String;

#[derive(CandidType, Debug, Deserialize)]
pub struct TokenPayload {
    pub token_id: Principal,
    pub owner: Principal,
    pub fee_to: TokenHolder,
    pub meta: Metadata,
    pub desc: Vec<(String, String)>,
    pub logo: Vec<u8>,
    pub balances: Vec<(TokenHolder, Nat)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, Nat)>)>,
    pub nonces: Vec<(Principal, u64)>,
    pub tx_index_cursor: Nat,
    pub storage_canister_ids: Vec<(Nat, Principal)>,
    pub txs_inner: Txs,
}
