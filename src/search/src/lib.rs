// TODO: Remove after implementing functionality
#![allow(unused_variables)]

mod deposit;
mod manage;
mod search;
mod stake;

use candid::{CandidType, Deserialize};
use ic_cdk::{api::time, caller, export::Principal};
use ic_cdk_macros::query;
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

thread_local! {
    static APP: RefCell<AppState<CanisterEnvironment>> = RefCell::new(AppState::new(CanisterEnvironment));
}

#[query]
fn greet(name: String) -> String {
    format!("Hello {}", name)
}

#[query]
fn greet_caller() -> String {
    format!("Hello {}", caller())
}

/// This structure holds the whole state of the app.
///
/// NOTE: `staked_websites` and `staked_terms` describe the same data, and need to be kept in sync.
/// The reason we keep the data twice is because we need fast access by terms and by website keys.
#[derive(Debug, Clone)]
struct AppState<E: Environment> {
    /// Handle to the environment.
    env: E,

    /// These are the unstaked tokens, the website owners have currently deposited on the service.
    unstaked_deposits: HashMap<Principal, u64>,

    /// The website descriptions.
    websites: HashMap<Website, WebsiteDescription>,

    /// Stores the stakes such that they are searchable by website.
    staked_websites: HashMap<Website, (u64, String)>,

    /// Stores the stakes such that they are searchable by term.
    staked_terms: HashMap<String, (u64, Website)>,
}

impl<E: Environment> AppState<E> {
    fn new(env: E) -> Self {
        Self {
            env,
            unstaked_deposits: HashMap::new(),
            websites: HashMap::new(),
            staked_websites: HashMap::new(),
            staked_terms: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct WebsiteDescription {
    name: String,
    link: String,
    description: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct Website {
    owner: Principal,
    link: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct Stake {
    term: String,
    value: i64,
}

trait Environment: Debug {
    fn get_caller(&self) -> Principal;
    fn get_time(&self) -> u64;
}

#[derive(Debug, Clone)]
struct CanisterEnvironment;

impl Environment for CanisterEnvironment {
    fn get_caller(&self) -> Principal {
        caller()
    }

    fn get_time(&self) -> u64 {
        time()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestEnvironment {
        caller: Principal,
        time: u64,
    }

    impl Environment for TestEnvironment {
        fn get_caller(&self) -> Principal {
            self.caller
        }

        fn get_time(&self) -> u64 {
            self.time
        }
    }
}
