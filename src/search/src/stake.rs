use crate::{AppState, Environment};
use crate::{Stake, Website, APP};
use ic_cdk_macros::{query, update};

#[query]
fn get_stakes(website: Website) -> Vec<Stake> {
    APP.with(|state| state.borrow().get_stakes(website))
}

#[update]
fn stake(link: String, stake_delta: Vec<Stake>) -> Vec<Stake> {
    APP.with(|state| state.borrow_mut().stake(link, stake_delta))
}

impl<E: Environment> AppState<E> {
    fn get_stakes(&self, website: Website) -> Vec<Stake> {
        // Get the caller
        let owner = self.env.get_non_anon_caller();
        if owner != website.owner {
            panic!("Principal is not the owner of the website.");
        }

        let default: Vec<(u64, String)> = Vec::new();
        // Get the stakes for a single site.
        self.staked_websites
            .get(&website)
            .unwrap_or(&default)
            .iter()
            .map(|stake| Stake {
                term: stake.1.clone(),
                value: stake.0 as i64,
            })
            .collect()
    }

    fn stake(&mut self, link: String, stake_delta: Vec<Stake>) -> Vec<Stake> {
        todo!()
    }
}

// TODO: Tests
