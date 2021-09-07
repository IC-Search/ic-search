use crate::{AppState, Environment, APP};
use candid::Principal;
use ic_cdk_macros::{query, update};
use std::cmp::min;

#[query]
fn get_cycles() -> u64 {
    APP.with(|state| state.borrow().get_cycles())
}

#[update]
fn deposit_cycles(max_amount: u64) -> u64 {
    APP.with(|state| state.borrow_mut().deposit_cycles(max_amount))
}

#[update]
async fn withdraw_cycles(max_amount: u64, destination: Principal) -> u64 {
    // Check how many cycles the caller is actually allowed to withdraw
    let (cycles, env) = APP.with(|state| state.borrow().prepare_withdraw_cycles(max_amount));

    // Return early if there are no cycles to withdraw
    if cycles == 0 {
        return 0;
    }

    // Deposit the cycles to the wallet canister
    if !env.send_cycles_to_canister(cycles, destination).await {
        return 0;
    }

    // If deposit was successful, update state and return
    APP.with(|state| state.borrow_mut().finish_withdraw_cycles(cycles));
    cycles
}

impl<E: Environment> AppState<E> {
    fn get_cycles(&self) -> u64 {
        self.unstaked_deposits
            .get(&self.env.get_non_anon_caller())
            .unwrap_or(&0)
            .to_owned()
    }

    fn deposit_cycles(&mut self, max_amount: u64) -> u64 {
        todo!()
    }

    /// Returns the amount of cycles that can actually be sent and the environments, such that we can send the cycles.
    fn prepare_withdraw_cycles(&self, max_amount: u64) -> (u64, E) {
        // Check that caller exists
        let caller = self.env.get_non_anon_caller();

        // Calculate the number of cycles that are withdrawable
        let cycles = min(
            self.unstaked_deposits.get(&caller).unwrap_or(&0),
            &max_amount,
        )
        .to_owned();

        (cycles, self.env.clone())
    }

    fn finish_withdraw_cycles(&mut self, amount: u64) {
        let caller = self.env.get_non_anon_caller();
        let cycles = self.unstaked_deposits.get(&caller).unwrap_or(&0);
        let new_cycles = cycles.checked_sub(amount).unwrap_or(0);

        if new_cycles == 0 {
            self.unstaked_deposits.remove(&caller);
        } else {
            self.unstaked_deposits.insert(caller, new_cycles);
        }
    }
}

// TODO: Tests
