use std::convert::TryFrom;

use candid::Principal;
use ic_cdk_macros::init;

use crate::{AppState, CanisterEnvironment, Stake, StakeDelta, WebsiteDescription, APP};

#[init]
fn init() {
    APP.with(|state| state.borrow_mut().init())
}

impl AppState<CanisterEnvironment> {
    fn init(&mut self) {
        // NNS Dapp
        self.init_entry(
            "r7inp-6aaaa-aaaaa-aaabq-cai",
            "NNS Dapp",
            &principal_to_link("qoctq-giaaa-aaaaa-aaaea-cai"),
            "UI of the Internet Computer's Network Nervous System",
            vec![
                (tc(10.0), "IC"),
                (tc(2.0), "NNS"),
                (tc(10.0), "Internet"),
                (tc(10.0), "Computer"),
                (tc(5.0), "DFINITY"),
            ],
        );

        // II
        self.init_entry(
            "r7inp-6aaaa-aaaaa-aaabq-cai",
            "Internet Identity",
            &principal_to_link("rdmx6-jaaaa-aaaaa-aaadq-cai"),
            "Internet Identity privaciy preserving authentication system",
            vec![
                (tc(8.0), "IC"),
                (tc(2.0), "II"),
                (tc(10.0), "Internet"),
                (tc(2.0), "Computer"),
                (tc(10.0), "Identity"),
            ],
        );

        // TODO: Distrikt, DSCVR, Motoko School, Motoko Playground, OpenChat, Canlista, DeFind,
    }

    /// Enters a valid website and stakes it with terms.
    fn init_entry(
        &mut self,
        controller: &str,
        name: &str,
        link: &str,
        desc: &str,
        stakes: Vec<(u64, &str)>,
    ) {
        // Parse the caller
        let caller = match Principal::try_from(controller) {
            Err(_) => return,
            Ok(caller) => caller,
        };

        // Manually increase the amount of unstaked tokens such that there are
        // enough tokens for staking. Here we are effectively donating tokens.
        let stakes_needed: u64 = stakes.iter().map(|(stake, _)| stake).sum();
        self.unstaked_deposits
            .entry(caller.clone())
            .and_modify(|stake| *stake += stakes_needed)
            .or_insert(stakes_needed);

        // Now we use a caller overwrite to set the correct caller and then initiate a number of
        // calls that set the website description and stake the values on it
        let mut env = self.env.clone();
        env.with_caller_overwrite(caller, || {
            // Set the description
            self.set_description(WebsiteDescription {
                name: name.to_string(),
                link: link.to_string(),
                description: desc.to_string(),
            });

            // Stake the terms on the website
            let stake_deltas = stakes
                .iter()
                .map(|(value, term)| {
                    StakeDelta::Add(Stake {
                        term: term.to_string(),
                        value: *value,
                    })
                })
                .collect();
            self.stake(link.to_string(), stake_deltas);
        });
    }
}
/// Converts Principals (as Strings) into verified urls
fn principal_to_link(principal: &str) -> String {
    format!("https://{}.ic0.app", principal)
}

fn tc(tc: f64) -> u64 {
    (tc * 1_000_000_000f64) as u64
}
