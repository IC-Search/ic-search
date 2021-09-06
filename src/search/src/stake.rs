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
        todo!()
    }

    fn stake(&mut self, link: String, stake_delta: Vec<Stake>) -> Vec<Stake> {
        todo!()
    }
}
