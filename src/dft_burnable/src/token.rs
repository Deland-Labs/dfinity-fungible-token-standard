use candid::{Nat, Principal};
use dft_standard::token::TokenBasic;
use dft_types::*;

pub trait BurnableExtension {
    //burn
    fn burn(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
    //burn from
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
}

// imple BurnableExtension for TokenBasic
impl BurnableExtension for TokenBasic {
    fn burn(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        value: Nat,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;
        let created_at = created_at.unwrap_or(now.clone());
        self._burn(owner, value, created_at, now)
    }
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;
        let created_at = created_at.unwrap_or(now.clone());
        self._burn_from(spender, owner, value, created_at, now)
    }
}
