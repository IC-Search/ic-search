use crate::{AppState, Environment, APP};
use candid::Principal;
use ic_cdk_macros::{query, update};
use std::cmp::min;

#[query]
fn get_unstaked_cycles() -> u64 {
    APP.with(|state| state.borrow().get_unstaked_cycles())
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
    fn get_unstaked_cycles(&self) -> u64 {
        self.unstaked_deposits
            .get(&self.env.get_non_anon_caller())
            .unwrap_or(&0)
            .to_owned()
    }

    fn deposit_cycles(&mut self, max_amount: u64) -> u64 {
        // Get the caller
        let caller = self.env.get_non_anon_caller();

        // Accept the cycles
        let accepted_cycles = self.env.accept_cycles(max_amount);

        // Register accepted cycles in the app state
        self.unstaked_deposits
            .entry(caller)
            .and_modify(|current_cycles| *current_cycles += accepted_cycles)
            .or_insert(accepted_cycles);

        // Return the number of accepted cycles
        accepted_cycles
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::test::*;

    fn test_state_for_deposit(
        env: TestEnvironment,
        mut unstaked_deposits: Vec<(Principal, u64)>,
    ) -> AppState<TestEnvironment> {
        AppState {
            env,
            unstaked_deposits: unstaked_deposits.drain(..).collect(),
            website_owners: HashMap::new(),
            websites: HashMap::new(),
            staked_websites: HashMap::new(),
            staked_terms: HashMap::new(),
        }
    }

    // TODO: Tests, that multiple accounts can deposit cycles and
    // check their balance of unstaked tokens

    /// Tests that an anonymous account can not deposit cycles
    #[test]
    #[should_panic]
    fn anon_can_not_deposit() {
        let env = TestEnvironment::new();
        env.set_caller(Principal::anonymous());
        env.set_max_cycles_to_accept(Some(200));
        let mut app = test_state_for_deposit(TestEnvironment::new(), vec![]);
        app.deposit_cycles(100);
    }

    /// Tests that an anonymous account can not withdraw cycles
    #[test]
    #[should_panic]
    fn anon_can_not_withdraw() {
        let env = TestEnvironment::new();
        env.set_caller(Principal::anonymous());
        env.set_max_cycles_to_accept(Some(200));
        let app = test_state_for_deposit(TestEnvironment::new(), vec![]);
        // NOTE: The preparation allready fails
        app.prepare_withdraw_cycles(100);
    }
}
