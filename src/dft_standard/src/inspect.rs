use crate::token::TokenStandard;
use candid::{Nat, Principal};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::inspect_message;

use crate::state::TOKEN;

static QUERY_METHODS: [&str; 17] = [
    "allowance",
    "allowancesOf",
    "balanceOf",
    "decimals",
    "desc",
    "fee",
    "logo",
    "meta",
    "name",
    "owner",
    "symbol",
    "tokenInfo",
    "totalSupply",
    "lastTransactions",
    "transactionById",
    "transactionByIndex",
    "__get_candid_interface_tmp_hack",
];

static OWNER_METHODS: [&str; 6] = [
    "setDesc", "setFee", "setFeeTo", "setFeeTo", "setLogo", "setOwner",
];
static HOLDER_METHODS: [&str; 3] = ["approve", "transfer", "burn"];

//static SPENDER_METHODS: [&str; 3] = ["transferFrom", "burnFrom"];
#[inspect_message]
fn inspect_message() {
    let method = api::call::method_name();
    let caller = api::caller();

    match &method[..] {
        m if QUERY_METHODS.contains(&m) => api::call::accept_message(),
        m if HOLDER_METHODS.contains(&m) => {
            let holder = match m {
                "approve" => {
                    let (sub_account, _, _) =
                        api::call::arg_data::<(Option<Subaccount>, String, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                "transfer" => {
                    let (sub_account, _, _) =
                        api::call::arg_data::<(Option<Subaccount>, String, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                "burn" => {
                    let (sub_account, _) = api::call::arg_data::<(Option<Subaccount>, Nat)>();
                    TokenHolder::new(caller, sub_account)
                }
                // never match this case
                _ => TokenHolder::new(caller, None),
            };

            // check caller's balance
            let balance = TOKEN.with(|token| token.borrow().balance_of(&holder));
            if balance > Nat::from(0) {
                api::call::accept_message();
            } else if caller == Principal::anonymous() {
                let err: ErrorInfo = DFTError::NotAllowAnonymous.into();
                let err_msg = format!("reject {:?}", err);
                api::print(err_msg.to_string());
                api::call::reject(err_msg.as_str());
            } else {
                let err: ErrorInfo = DFTError::InsufficientBalance.into();
                let err_msg = format!("reject {:?}", err);
                api::print(err_msg.to_string());
                api::call::reject(err_msg.as_str());
            }
        }
        m if OWNER_METHODS.contains(&m) => {
            // check if caller is owner
            let owner = TOKEN.with(|token| token.borrow().owner());
            if caller == owner {
                api::call::accept_message();
            } else {
                let err: ErrorInfo = DFTError::OnlyOwnerAllowCallIt.into();
                let err_msg = format!("reject {:?}", err);
                api::print(err_msg.to_string());
                api::call::reject(err_msg.as_str());
            }
        }
        _ => {
            api::call::accept_message();
            ic_cdk::println!("inspect: method not checked; accept");
        }
    }
}
