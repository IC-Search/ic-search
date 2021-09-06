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
async fn withdraw_cycles(amount: u64, destination: Principal) -> u64 {
    todo!()
}

impl<E: Environment> AppState<E> {
    fn get_cycles(&self) -> u64 {
        self.unstaked_deposits
            .get(&self.env.get_caller())
            .unwrap_or(&0)
            .to_owned()
    }

    fn deposit_cycles(&mut self, max_amount: u64) -> u64 {
        todo!()
    }

    /// Returns the amount of cycles that can actually be sent and the environments, such that we can send the cycles.
    fn prepare_withdraw_cycles(
        &mut self,
        max_amount: u64,
        destination: Principal,
    ) -> Option<(u64, E)> {
        // Check that caller exists and the amount
        todo!()
    }

    fn finish_withdraw_cycles(&mut self, amount: u64) {
        let caller = self.env.get_caller();
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
