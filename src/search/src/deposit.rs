use crate::{AppState, Environment, APP};
use candid::Principal;
use ic_cdk_macros::{query, update};

#[query]
fn get_cycles() -> u64 {
    APP.with(|state| state.borrow().get_cycles())
}

#[update]
fn deposit_cycles(max_amount: u64) -> u64 {
    APP.with(|state| state.borrow_mut().deposit_cycles(max_amount))
}

#[update]
fn withdraw_cycles(amount: u64, destination: Principal) -> u64 {
    APP.with(|state| state.borrow_mut().withdraw_cycles(amount, destination))
}

impl<E: Environment> AppState<E> {
    fn get_cycles(&self) -> u64 {
        todo!()
    }

    fn deposit_cycles(&mut self, max_amount: u64) -> u64 {
        todo!()
    }

    fn withdraw_cycles(&mut self, amount: u64, destination: Principal) -> u64 {
        todo!()
    }
}

// TODO: Tests
