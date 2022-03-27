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
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        let nonce = self.get_verified_nonce(caller, nonce)?;
        self._burn(caller, owner, owner, value, nonce, now)
    }
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        let nonce = self.get_verified_nonce(caller, nonce)?;
        // debit spender's allowance
        self.debit_allowance(owner, spender, value.clone())?;
        self._burn(caller, spender, owner, value, nonce, now)
    }
}
