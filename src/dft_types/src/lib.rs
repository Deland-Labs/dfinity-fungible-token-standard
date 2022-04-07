mod account_identifier;
pub mod constants;
mod desc_keys;
mod errors;
mod fee;
mod http;
mod metadata;
mod token_holder;
mod token_info;
mod token_metrics;
mod token_payload;
mod transaction_result;
mod tx_record;
mod actor_response;
mod block;
mod blockchain;

use candid::Nat;
use candid::Principal;
use std::collections::HashMap;
use std::string::String;

pub type TransactionIndex = Nat;
pub type TransactionId = String;
pub type ExtendData = HashMap<String, String>;
pub type Balances = HashMap<TokenHolder, Nat>;
pub type StorageCanisterIds = HashMap<Nat, Principal>;
pub type Txs = Vec<TxRecord>;
pub type Allowances = HashMap<TokenHolder, HashMap<TokenHolder, Nat>>;

pub use account_identifier::AccountIdentifier;
pub use account_identifier::Subaccount;
pub use account_identifier::SUB_ACCOUNT_ZERO;
pub use fee::Fee;
pub use metadata::Metadata;
pub use token_holder::TokenHolder;

pub type TransferFrom = TokenHolder;
pub type TokenReceiver = TokenHolder;

pub type BlockHash=[u8;32];
pub type BlockHeight=u64;
pub type TransactionHash=[u8;32];

pub use desc_keys::DESC_KEYS;
pub use errors::*;
pub use http::*;
pub use token_info::TokenInfo;
pub use token_metrics::TokenMetrics;
pub use token_payload::TokenPayload;
pub use transaction_result::TransactionResponse;
pub use tx_record::*;
pub use actor_response::*;
pub use block::*;
pub use blockchain::*;

#[test]
fn test_nat_size() {
    let nat_size = std::mem::size_of::<Nat>();
    assert_eq!(24, nat_size, "nat_size is not {}", 24);
}
