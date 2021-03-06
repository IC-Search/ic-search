use crate::{clean_term, AppState, Environment, StakeDelta};
use crate::{Stake, Website, APP};
use ic_cdk_macros::{query, update};
use std::collections::HashMap;

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
        self.staked_websites
            .get(&website)
            .cloned()
            .unwrap_or_default()
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

    pub(crate) fn stake(&mut self, link: String, stake_deltas: Vec<StakeDelta>) -> Vec<Stake> {
        // Get the caller.
        let owner = self.env.get_non_anon_caller();

        // Create the website key.
        let website = Website { owner, link };

        // Transform staked_websites entry into a map.
        let default_stakes: Vec<(u64, String)> = vec![];
        let stakes = self
            .staked_websites
            .get(&website)
            .unwrap_or(&default_stakes);
        let mut term_balances: HashMap<_, _> = stakes
            .iter()
            .map(|(cycles, term)| (term.clone(), *cycles))
            .collect();

        // Load the deltas into their own vectors.
        let mut add_deltas: Vec<Stake> = vec![];
        let mut remove_deltas: Vec<Stake> = vec![];
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
            let term = clean_term(&stake.term);
            let balance = *term_balances.get(&term).unwrap_or(&0);
            if balance < stake.value {
                panic!(
                    "Term `{}` must have enough staked cycles to remove.",
                    stake.term
                );
            }

            let new_balance = balance.saturating_sub(stake.value);
            term_balances.insert(term, new_balance);
            reclaimed_cycles += stake.value;
        }

        // Update balances with the add deltas.
        let mut available_cycles =
            reclaimed_cycles + self.unstaked_deposits.get(&owner).unwrap_or(&0);
        if available_cycles == 0 && !add_deltas.is_empty() {
            panic!(
                "Principal {} does not have enough unstaked cycles.",
                owner.to_string()
            );
        }

        for stake in add_deltas {
            let term = clean_term(&stake.term);
            if available_cycles < stake.value {
                panic!(
                    "Not enough cycles available to stake term `{}`.",
                    stake.term
                );
            }

            term_balances
                .entry(term)
                .and_modify(|balance| {
                    *balance += stake.value;
                })
                .or_insert(stake.value);

            available_cycles -= stake.value;
        }

        // Update the `unstaked_deposits` balances
        if available_cycles == 0 {
            self.unstaked_deposits.remove(&owner);
        } else {
            self.unstaked_deposits.insert(owner, available_cycles);
        }

        // Update the indexes `staked_terms` and `staked_websites`.
        self._update_staked_websites_index(&website, &term_balances);
        self._update_staked_terms_index(&website, &term_balances);

        // Get the stakes for a single site.
        self._get_stakes(website)
    }

    fn _update_staked_websites_index(
        &mut self,
        website: &Website,
        term_balances: &HashMap<String, u64>,
    ) {
        let staked_website_terms: Vec<(u64, String)> = term_balances
            .iter()
            .filter_map(|(term, balance)| {
                if *balance > 0 {
                    Some((*balance, term.clone()))
                } else {
                    None
                }
            })
            .collect();
        if !staked_website_terms.is_empty() {
            self.staked_websites
                .insert(website.clone(), staked_website_terms);
        } else {
            self.staked_websites.remove(website);
        }
    }

    fn _update_staked_terms_index(
        &mut self,
        website: &Website,
        term_balances: &HashMap<String, u64>,
    ) {
        for (term, balance) in term_balances {
            let staked_websites = self
                .staked_terms
                .entry(term.clone())
                .or_insert_with(Vec::new);
            let maybe_stake_index = staked_websites.iter().position(|website_stake| {
                website_stake.1.link == website.link && website_stake.1.owner == website.owner
            });

            match maybe_stake_index {
                Some(index) => {
                    if *balance == 0 {
                        staked_websites.remove(index);
                    } else {
                        staked_websites[index] = (*balance, website.clone());
                    }
                }
                None => staked_websites.push((*balance, website.clone())),
            };

            if staked_websites.is_empty() {
                self.staked_terms.remove(term);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;

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
    #[should_panic(
        expected = "Principal 74xlx-rlmho-nkoz7-xqw2t-pqgyx-jp2kr-tx42t-ofajs-bx53e-7eito-h2i does not have enough unstaked cycles."
    )]
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
    #[should_panic(expected = "Term `test` must have enough staked cycles to remove.")]
    fn test_staked_term_not_enough_balance_for_remove_delta() {
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
            vec![StakeDelta::Remove(Stake { term, value: 801 })],
        );
    }

    #[test]
    fn test_one_staked_deposit_and_one_add_delta() {
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
        let stake = stakes.get(0).cloned().unwrap_or_default();
        assert_eq!(stake.term, "test");
        assert_eq!(stake.value, 100);
    }

    #[test]
    #[should_panic(expected = "Not enough cycles available to stake term `term3`.")]
    fn test_one_staked_deposit_and_too_many_add_deltas() {
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
            vec![
                StakeDelta::Add(Stake {
                    term: String::from("test"),
                    value: 500,
                }),
                StakeDelta::Add(Stake {
                    term: String::from("term2"),
                    value: 500,
                }),
                StakeDelta::Add(Stake {
                    term: String::from("term3"),
                    value: 500,
                }),
            ],
        );
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

        assert!(app.staked_websites.is_empty());
        assert!(app.staked_terms.is_empty());
    }

    #[test]
    fn test_one_staked_deposit_and_add_and_remove_deltas() {
        let default_stake = Stake {
            term: String::from("test"),
            value: 0,
        };
        let term = String::from("test");
        let term2 = String::from("term2");
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
            vec![
                StakeDelta::Remove(Stake {
                    term: String::from("test"),
                    value: 800,
                }),
                StakeDelta::Add(Stake {
                    term: String::from("term2"),
                    value: 1000,
                }),
            ],
        );

        assert_eq!(stakes.len(), 1);
        let stake = stakes.get(0).cloned().unwrap_or_default();
        assert_eq!(stake.term, "term2");
        assert_eq!(stake.value, 1000);

        assert_eq!(
            *app.unstaked_deposits
                .get(&test_principal_id(0))
                .unwrap_or(&0),
            0
        );
    }

    #[test]
    fn test_multiple_staked_deposit_and_add_and_remove_deltas() {
        let default_stake = Stake {
            term: String::from("test"),
            value: 0,
        };
        let term = String::from("test");
        let term2 = String::from("term2");
        let mut app = test_state_for_staking(
            TestEnvironment::new(),
            vec![
                (test_principal_id(0), 200),
                (test_principal_id(1), 1000),
                (test_principal_id(2), 500),
            ],
            vec![(test_website(0), test_website_description(0))],
            vec![
                (
                    test_website(0),
                    vec![(600, term.clone()), (200, term2.clone())],
                ),
                (test_website(1), vec![(200, term.clone())]),
                (test_website(2), vec![(500, term2.clone())]),
            ],
            vec![
                (
                    term.clone(),
                    vec![(600, test_website(0)), (200, test_website(1))],
                ),
                (
                    term2.clone(),
                    vec![(200, test_website(0)), (500, test_website(2))],
                ),
            ],
        );
        app.env.set_caller(test_principal_id(0));
        let stakes = app.stake(
            test_url(0),
            vec![
                StakeDelta::Remove(Stake {
                    term: String::from("test"),
                    value: 600,
                }),
                StakeDelta::Add(Stake {
                    term: String::from("term2"),
                    value: 800,
                }),
            ],
        );

        assert_eq!(stakes.len(), 1);
        let stake = stakes.get(0).cloned().unwrap_or_default();
        assert_eq!(stake.term, "term2");
        assert_eq!(stake.value, 1000);

        assert_eq!(
            *app.unstaked_deposits
                .get(&test_principal_id(0))
                .unwrap_or(&0),
            0
        );
        assert_eq!(
            *app.unstaked_deposits
                .get(&test_principal_id(1))
                .unwrap_or(&0),
            1000
        );
        assert_eq!(
            *app.unstaked_deposits
                .get(&test_principal_id(2))
                .unwrap_or(&0),
            500
        );

        let staked_website_0 = app
            .staked_websites
            .get(&test_website(0))
            .cloned()
            .unwrap_or(vec![]);
        assert_eq!(staked_website_0.len(), 1);
        let staked_website_0_term_1 = staked_website_0
            .get(0)
            .cloned()
            .unwrap_or((0, String::from("blargh")));
        assert_eq!(staked_website_0_term_1.0, 1000);
        assert_eq!(staked_website_0_term_1.1, term2.clone());

        // Evaluate that the `staked_terms` hashmap has been maintained correctly.
        let term1_stakes = app.staked_terms.get(&term).cloned().unwrap_or(vec![]);
        assert_eq!(term1_stakes.len(), 1);
        let term1_stakes_1 = term1_stakes.get(0).cloned().unwrap_or((0, test_website(0)));
        assert_eq!(term1_stakes_1.0, 200);
        assert_eq!(term1_stakes_1.1.owner, test_principal_id(1));

        let term2_stakes = app.staked_terms.get(&term2).cloned().unwrap_or(vec![]);
        assert_eq!(term2_stakes.len(), 2);
        let term2_stakes_1 = term2_stakes.get(0).cloned().unwrap_or((0, test_website(0)));
        assert_eq!(term2_stakes_1.0, 1000);
        assert_eq!(term2_stakes_1.1.owner, test_principal_id(0));
        let term2_stakes_2 = term2_stakes.get(1).cloned().unwrap_or((0, test_website(0)));
        assert_eq!(term2_stakes_2.0, 500);
        assert_eq!(term2_stakes_2.1.owner, test_principal_id(2));
    }
}
