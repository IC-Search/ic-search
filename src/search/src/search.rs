use crate::APP;
use crate::{AppState, Environment, WebsiteDescription};
use ic_cdk_macros::query;

#[query]
fn search(terms: Vec<String>, page: u64, entries_per_page: u64) -> Vec<WebsiteDescription> {
    APP.with(|state| state.borrow().search(terms, page, entries_per_page))
}

impl<E: Environment> AppState<E> {
    fn search(
        &self,
        terms: Vec<String>,
        page: u64,
        entries_per_page: u64,
    ) -> Vec<WebsiteDescription> {
        // - Initialize Score map
        // - For each term:
        // -    Get stake
        // -    For each stake:
        // -        Add stake to website score
        // - Map to vec
        // - Sort websites by score
        // - Chunk by `entries_per_page`
        // - Take chunk number `page`
        // - Map to description
        // - Return
        todo!()
    }
}

// TODO: Tests
