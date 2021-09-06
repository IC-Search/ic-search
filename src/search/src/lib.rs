use candid::CandidType;
use ic_cdk::{api::time, caller, export::Principal};
use ic_cdk_macros::query;
use std::{cell::RefCell, fmt::Debug};

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

#[derive(Debug, Clone)]
struct AppState<E: Environment> {
    env: E,
}

impl<E: Environment> AppState<E> {
    fn new(env: E) -> Self {
        Self { env }
    }
}

#[derive(Debug, Clone, CandidType)]
struct WebsiteDescription {
    name: String,
    link: String,
    description: String,
}

#[derive(Debug, Clone, CandidType)]
struct Website {
    owner: Principal,
    link: String,
}

#[derive(Debug, Clone, CandidType)]
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
