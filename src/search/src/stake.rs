use std::collections::HashMap;

use crate::{AppState, Environment, StakeDelta};
use crate::{Stake, Website, APP};
use ic_cdk::export::Principal;
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
                StakeDelta::Add(stake) => {
                    if stake.value > 0 {
                        add_deltas.push(stake);
                    }
                }
                StakeDelta::Remove(stake) => {
                    if stake.value > 0 {
                        remove_deltas.push(stake);
                    }
                }
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

        // Update the balances
        if available_cycles == 0 {
            self.unstaked_deposits.remove(&owner);
        } else {
            self.unstaked_deposits.insert(owner, available_cycles);
        }

        let mut staked_website: Vec<(u64, String)> = Vec::with_capacity(term_balances.len());
        for (term, balance) in term_balances {
            staked_website.push((balance, term.clone()));
            let stake_entries = self.staked_terms.entry(term.clone()).or_insert(Vec::new());
            let maybe_staked = stake_entries.iter().position(|entry| entry.1 == website);
            let new_stake_entry = (balance, website.clone());
            match maybe_staked {
                Some(index) => {
                    std::mem::replace(&mut stake_entries[index], new_stake_entry);
                }
                None => stake_entries.push(new_stake_entry),
            };
        }

        self.staked_websites.insert(website.clone(), staked_website);
        // Get the stakes for a single site.
        self._get_stakes(website)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test::*, WebsiteDescription};

    fn test_state_for_staking(
        env: TestEnvironment,
        mut unstaked_deposits: Vec<(Principal, u64)>,
        mut websites: Vec<(Website, WebsiteDescription)>,
        mut staked_websites: Vec<(Website, Vec<(u64, String)>)>,
        mut staked_terms: Vec<(String, Vec<(u64, Website)>)>,
    ) -> AppState<TestEnvironment> {
        AppState {
            env,
            unstaked_deposits: unstaked_deposits.drain(..).collect(),
            website_owners: HashMap::new(),
            websites: websites.drain(..).collect(),
            staked_websites: staked_websites.drain(..).collect(),
            staked_terms: staked_terms.drain(..).collect(),
        }
    }

    #[test]
    #[should_panic(expected = "Principal does not have enough unstaked cycles.")]
    fn test_empty_unstaked_deposits() {
        let mut app =
            test_state_for_staking(TestEnvironment::new(), vec![], vec![], vec![], vec![]);
        app.env.set_caller(test_principal_id(0));
        app.stake(
            test_url(0),
            vec![StakeDelta::Add(Stake {
                term: String::from("test"),
                value: 1,
            })],
        );
    }

    #[test]
    fn test_one_staked_deposit_and_one_add_delta() {
        let default_stake = Stake {
            term: String::from("test"),
            value: 0,
        };
        let mut app = test_state_for_staking(
            TestEnvironment::new(),
            vec![(test_principal_id(0), 1000)],
            vec![],
            vec![],
            vec![],
        );
        app.env.set_caller(test_principal_id(0));
        let stakes = app.stake(
            test_url(0),
            vec![StakeDelta::Add(Stake {
                term: String::from("test"),
                value: 100,
            })],
        );

        assert_eq!(stakes.len(), 1);
        assert_eq!(
            *app.unstaked_deposits
                .get(&test_principal_id(0))
                .unwrap_or(&0),
            900
        );
        let stake = stakes.get(0).unwrap_or(&default_stake);
        assert_eq!(stake.term, "test");
        assert_eq!(stake.value, 100);
    }

    #[test]
    fn test_one_staked_deposit_and_one_remove_delta() {
        let default_stake = Stake {
            term: String::from("test"),
            value: 0,
        };
        let term = String::from("test");
        let mut app = test_state_for_staking(
            TestEnvironment::new(),
            vec![(test_principal_id(0), 200)],
            vec![(test_website(0), test_website_description(0))],
            vec![(test_website(0), vec![(800, term.clone())])],
            vec![(term.clone(), vec![(800, test_website(0))])],
        );
        app.env.set_caller(test_principal_id(0));
        let stakes = app.stake(
            test_url(0),
            vec![StakeDelta::Remove(Stake {
                term: String::from("test"),
                value: 800,
            })],
        );

        assert_eq!(stakes.len(), 0);
        assert_eq!(
            *app.unstaked_deposits
                .get(&test_principal_id(0))
                .unwrap_or(&0),
            1000
        );
    }
}
