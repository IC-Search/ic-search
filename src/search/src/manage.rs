use crate::APP;
use crate::{AppState, Environment, WebsiteDescription};
use ic_cdk_macros::{query, update};

#[query]
fn get_websites() -> Vec<WebsiteDescription> {
    APP.with(|state| state.borrow().get_websites())
}

#[update]
fn set_description(website: WebsiteDescription) {
    APP.with(|state| state.borrow_mut().set_description(website))
}

#[update]
fn remove_website(link: String) {
    APP.with(|state| state.borrow_mut().remove_website(link))
}

impl<E: Environment> AppState<E> {
    fn get_websites(&self) -> Vec<WebsiteDescription> {
        todo!()
    }

    fn set_description(&mut self, website: WebsiteDescription) {
        todo!()
    }

    fn remove_website(&mut self, link: String) {
        todo!()
    }
}

// TODO: Tests
