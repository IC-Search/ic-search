// TODO: Remove after implementing functionality
#![allow(unused_variables)]

mod deposit;
mod init;
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
    static APP: RefCell<AppState<CanisterEnvironment>>
        = RefCell::new(AppState::new(CanisterEnvironment::new()));
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

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, Hash)]
struct Website {
    owner: Principal,
    link: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
enum StakeDelta {
    Add(Stake),
    Remove(Stake),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct Stake {
    pub term: String,
    pub value: u64,
}

impl Default for Stake {
    fn default() -> Self {
        Self {
            term: Default::default(),
            value: Default::default(),
        }
    }
}

pub(crate) fn clean_term(term: &String) -> String {
    term.to_lowercase().trim().to_string()
}

#[async_trait]
trait Environment: Debug + Clone {
    fn get_caller(&self) -> Principal;
    fn get_time(&self) -> u64;
    async fn send_cycles_to_canister(&self, amount: u64, destination: Principal) -> bool;
    fn accept_cycles(&self, amount: u64) -> u64;

    fn get_non_anon_caller(&self) -> Principal {
        let caller = self.get_caller();
        if caller == Principal::anonymous() {
            panic!("Anonymous caller not supported");
        }
        caller
    }
}

#[derive(Debug, Clone)]
struct CanisterEnvironment {
    /// Can be used to manually overwrite the caller.
    ///
    /// Usefull for initializations, updates, etc.
    caller_overwrite: Option<Principal>,
}

impl CanisterEnvironment {
    fn new() -> Self {
        Self {
            caller_overwrite: None,
        }
    }

    pub(crate) fn set_caller_overwrite(&mut self, caller: Principal) {
        self.caller_overwrite = Some(caller);
    }

    pub(crate) fn unset_caller_overwrite(&mut self) {
        self.caller_overwrite = None;
    }
}

#[async_trait]
impl Environment for CanisterEnvironment {
    fn get_caller(&self) -> Principal {
        match self.caller_overwrite {
            // Return the caller overwrite, if one is set
            Some(caller) => caller,
            None => caller(),
        }
    }

    fn get_time(&self) -> u64 {
        time()
    }

    async fn send_cycles_to_canister(&self, amount: u64, destination: Principal) -> bool {
        match call_with_payment(destination, "wallet_receive", (), amount).await {
            Ok(()) => true,
            Err((_, string)) => false,
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

    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct TestEnvironment(Arc<Mutex<TestEnvironmentInner>>);

    #[derive(Debug, Clone)]
    struct TestEnvironmentInner {
        caller: Principal,
        time: u64,
        cycles_sent: Option<(u64, Principal)>,
        cycles_in_msg: u64,
    }

    impl TestEnvironment {
        pub(crate) fn new() -> Self {
            Self(Arc::new(Mutex::new(TestEnvironmentInner {
                caller: Principal::anonymous(),
                time: 0,
                cycles_sent: None,
                cycles_in_msg: 0,
            })))
        }

        pub(crate) fn set_caller(&self, caller: Principal) {
            self.lock().caller = caller;
        }

        pub(crate) fn set_cycles_in_msg(&self, cycles: u64) {
            self.lock().cycles_in_msg = cycles;
        }

        pub(crate) fn get_cycles_sent(&self) -> Option<(u64, Principal)> {
            self.lock().cycles_sent
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

        async fn send_cycles_to_canister(&self, amount: u64, destination: Principal) -> bool {
            self.lock().cycles_sent = Some((amount, destination));
            true
        }

        fn accept_cycles(&self, max_amount: u64) -> u64 {
            min(max_amount, self.lock().cycles_in_msg)
        }
    }

    pub(crate) fn test_principal_id(seed: u64) -> Principal {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut data: [u8; 29] = [0; 29];
        rng.try_fill(&mut data).unwrap();
        Principal::from_slice(&data)
    }

    pub(crate) fn test_website_name(seed: u64) -> String {
        lipsum::lipsum_words_from_seed(2, seed)
    }

    pub(crate) fn test_url(_seed: u64) -> String {
        // TODO: Come up with a good scheme here
        String::from("")
    }

    pub(crate) fn test_website(seed: u64) -> Website {
        Website {
            owner: test_principal_id(seed),
            link: test_url(seed),
        }
    }

    pub(crate) fn test_website_description(seed: u64) -> WebsiteDescription {
        WebsiteDescription {
            name: test_website_name(seed),
            link: test_url(seed),
            description: lipsum::lipsum_words_from_seed(30, seed),
        }
    }

    #[test]
    fn test_clean_term() {
        let t1 = String::from("TERM");
        assert_eq!(clean_term(&t1), "term");

        let t2 = String::from(" Term1 teRm2 ");
        assert_eq!(clean_term(&t2), "term1 term2");

        let t3 = String::from("\t\nterm\n\t");
        assert_eq!(clean_term(&t3), "term");
    }
}
