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
        todo!()
    }
}
