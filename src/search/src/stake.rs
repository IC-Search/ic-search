use std::collections::HashMap;

use crate::{AppState, Environment, StakeDelta};
use crate::{Stake, Website, APP};
use ic_cdk_macros::{query, update};

#[query]
fn get_stakes(website: Website) -> Vec<Stake> {
    APP.with(|state| state.borrow().get_stakes(website))
}

#[update]
fn stake(link: String, stake_delta: Vec<StakeDelta>) -> Vec<Stake> {
    APP.with(|state| state.borrow_mut().stake(link, stake_delta))
}

impl<E: Environment> AppState<E> {
    fn _get_stakes(&self, website: Website) -> Vec<Stake> {
        let default: Vec<(u64, String)> = Vec::new();
        self.staked_websites
            .get(&website)
            .unwrap_or(&default)
            .iter()
            .map(|stake| Stake {
                term: stake.1.clone(),
                value: stake.0,
            })
            .collect()
    }

    fn get_stakes(&self, website: Website) -> Vec<Stake> {
        // Get the caller
        let owner = self.env.get_non_anon_caller();
        if owner != website.owner {
            panic!("Principal is not the owner of the website.");
        }

        // Get the stakes for a single site.
        self._get_stakes(website)
    }

    fn stake(&mut self, link: String, stake_deltas: Vec<StakeDelta>) -> Vec<Stake> {
        // Get the caller.
        let owner = self.env.get_non_anon_caller();

        // Create the website key.
        let website = Website { owner, link };

        // Transform staked_websites entry into a map.
        let default_stakes: Vec<(u64, String)> = Vec::new();
        let stakes = self
            .staked_websites
            .get(&website)
            .unwrap_or(&default_stakes);
        let mut term_balances: HashMap<_, _> = stakes
            .iter()
            .map(|(cycles, term)| (term.clone(), *cycles))
            .collect();

        // Load the deltas into their own vectors.
        let mut add_deltas: Vec<Stake> = Vec::new();
        let mut remove_deltas: Vec<Stake> = Vec::new();
        for delta in stake_deltas {
            match delta {
                StakeDelta::Add(stake) => add_deltas.push(stake),
                StakeDelta::Remove(stake) => remove_deltas.push(stake),
            };
        }

        // Updates balances with the remove deltas.
        let mut reclaimed_cycles: u64 = 0;
        for stake in remove_deltas {
            let balance = *term_balances.get(&stake.term).unwrap_or(&0);
            if balance < stake.value {
                panic!(
                    "Term {} must have cycles enough staked to remove.",
                    stake.term
                );
            }

            let new_balance = balance.checked_sub(stake.value).unwrap_or(0);
            if new_balance == 0 {
                term_balances.remove(&stake.term);
            } else {
                term_balances.insert(stake.term, new_balance);
            }

            reclaimed_cycles += stake.value;
        }

        // Update balances with the add deltas.
        let mut available_cycles =
            reclaimed_cycles + self.unstaked_deposits.get(&owner).unwrap_or(&0);
        if available_cycles == 0 && !add_deltas.is_empty() {
            panic!("Principal does not have enough unstaked cycles.");
        }

        for stake in add_deltas {
            if available_cycles < stake.value {
                panic!("Not enough cycles available to stake term {}.", stake.term);
            }

            term_balances
                .entry(stake.term.clone())
                .and_modify(|balance| {
                    *balance += stake.value;
                })
                .or_insert(stake.value);

            available_cycles -= stake.value;
        }

        // Get the stakes for a single site.
        self._get_stakes(website)
    }
}

// TODO: Tests
