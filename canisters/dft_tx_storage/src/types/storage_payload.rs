use super::Txs;
use candid::Nat;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

#[derive(CandidType, Debug, Deserialize)]
pub struct StoragePayload {
    pub dft_id: Principal,
    pub tx_start_index: Nat,
    pub txs: Txs,
}