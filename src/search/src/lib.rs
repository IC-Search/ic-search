// TODO: Remove after implementing functionality
#![allow(unused_variables)]

mod deposit;
mod manage;
mod search;
mod stake;

use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use ic_cdk::{
    api::{
        call::{call_with_payment, msg_cycles_accept},
        time,
    },
    caller,
    export::Principal,
};
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
/// For the same reason, we need to keep `website` and `website_owners` in sync.
#[derive(Debug, Clone)]
struct AppState<E: Environment> {
    /// Handle to the environment.
    env: E,

    /// These are the unstaked tokens, the website owners have currently deposited on the service.
    unstaked_deposits: HashMap<Principal, u64>,

    /// Maps principals to websites, which is useful to know, which websites are staked.
    website_owners: HashMap<Principal, Vec<String>>,

    /// The website descriptions.
    websites: HashMap<Website, WebsiteDescription>,

    /// Stores the stakes such that they are searchable by website.
    staked_websites: HashMap<Website, Vec<(u64, String)>>,

    /// Stores the stakes such that they are searchable by term.
    staked_terms: HashMap<String, Vec<(u64, Website)>>,
}

impl<E: Environment> AppState<E> {
    fn new(env: E) -> Self {
        Self {
            env,
            unstaked_deposits: HashMap::new(),
            website_owners: HashMap::new(),
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

#[async_trait]
trait Environment: Debug {
    fn get_caller(&self) -> Principal;
    fn get_time(&self) -> u64;
    async fn send_cycles_to_canister(&self, amount: u64, destination: Principal);
    fn accept_cycles(&self, amount: u64) -> u64;
}

#[derive(Debug, Clone)]
struct CanisterEnvironment;

#[async_trait]
impl Environment for CanisterEnvironment {
    fn get_caller(&self) -> Principal {
        caller()
    }

    fn get_time(&self) -> u64 {
        time()
    }

    async fn send_cycles_to_canister(&self, amount: u64, destination: Principal) {
        match call_with_payment(destination, &"wallet_receive", (), amount).await {
            Ok(()) => (),
            Err((_, string)) => panic!("Unexpected error {}", string),
        }
    }

    fn accept_cycles(&self, amount: u64) -> u64 {
        msg_cycles_accept(amount)
    }
}

#[cfg(test)]
mod test {
    use std::{
        cmp::min,
        sync::{Arc, Mutex, MutexGuard},
    };

    use super::*;

    #[derive(Debug, Clone)]
    struct TestEnvironment(Arc<Mutex<TestEnvironmentInner>>);

    #[derive(Debug, Clone)]
    struct TestEnvironmentInner {
        caller: Principal,
        time: u64,
        cycles_sent: Option<(u64, Principal)>,
        max_cycles_to_accept: Option<u64>,
    }

    impl TestEnvironment {
        #[allow(dead_code)]
        fn new() -> Self {
            Self(Arc::new(Mutex::new(TestEnvironmentInner {
                caller: Principal::anonymous(),
                time: 0,
                cycles_sent: None,
                max_cycles_to_accept: None,
            })))
        }

        fn lock(&self) -> MutexGuard<TestEnvironmentInner> {
            self.0.lock().unwrap()
        }
    }

    #[async_trait]
    impl Environment for TestEnvironment {
        fn get_caller(&self) -> Principal {
            self.lock().caller
        }

        fn get_time(&self) -> u64 {
            self.lock().time
        }

        async fn send_cycles_to_canister(&self, amount: u64, destination: Principal) {
            self.lock().cycles_sent = Some((amount, destination))
        }

        fn accept_cycles(&self, amount: u64) -> u64 {
            match self.lock().max_cycles_to_accept {
                Some(max_amount) => min(max_amount, amount),
                None => amount,
            }
        }
    }
}
