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
            "A Governance Dapp for voting on Internet Computer governance proposals",
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
            "Internet Identity service enables you to authenticate securely and anonymously\
                when accessing applications on the Internet Computer",
            vec![
                (tc(8.0), "IC"),
                (tc(2.0), "II"),
                (tc(10.0), "Internet"),
                (tc(2.0), "Computer"),
                (tc(10.0), "Identity"),
            ],
        );

        // Distrikt
        self.init_entry(
            "hy3go-2qaaa-aaaae-aaabq-cai",
            "Distrik",
            &principal_to_link("c7fao-laaaa-aaaae-aaa4q-cai"),
            "distrikt is a decentralized, professional social\
                    media network that empowers users to own and control their identity.",
            vec![
                (tc(8.0), "Social"),
                (tc(8.0), "Media"),
                (tc(2.0), "Distrikt"),
                (tc(2.0), "Disrupt"),
                (tc(5.0), "Facebook"),
            ],
        );

        // DSCVR
        self.init_entry(
            "g6mnv-cyaaa-aaaab-qaaka-cai",
            "DSCVR",
            &principal_to_link("h5aet-waaaa-aaaab-qaamq-cai"),
            "A decentralized social news aggregator built on the Internet Computer",
            vec![
                (tc(8.0), "Social"),
                (tc(8.0), "Media"),
                (tc(12.0), "Discover"),
                (tc(5.0), "DSCVR"),
                (tc(5.0), "Reddit"),
            ],
        );

        // OpenChat
        self.init_entry(
            "7y2se-wiaaa-aaaaf-aaaba-cai",
            "OpenChat",
            &principal_to_link("7e6iv-biaaa-aaaaf-aaada-cai"),
            "A Truly Decentralized Alternative to WhatsApp",
            vec![
                (tc(5.0), "Open"),
                (tc(5.0), "Chat"),
                (tc(12.0), "WhatsApp"),
                (tc(5.0), "Communication"),
                (tc(5.0), "IC"),
            ],
        );

        // Motoko School
        self.init_entry(
            "vrmfo-gqaaa-aaaah-aaaga-cai",
            "Motoko School",
            &principal_to_link("anyuk-uiaaa-aaaah-aaduq-cai"),
            "A collaborative online school to learn the Motoko programming language",
            vec![
                (tc(5.0), "Motoko"),
                (tc(5.0), "Programming"),
                (tc(5.0), "Language"),
                (tc(12.0), "Online"),
                (tc(5.0), "Lessons"),
                (tc(5.0), "Canister"),
            ],
        );

        // Motoko Playground
        self.init_entry(
            "a3hwk-dyaaa-aaaab-qaa4a-cai",
            "Motoko Playgrond",
            &principal_to_link("m7sm4-2iaaa-aaaab-qabra-cai"),
            "An online playground to develop and deploy motoko canisters",
            vec![
                (tc(5.0), "Motoko"),
                (tc(5.0), "Programming"),
                (tc(5.0), "Language"),
                (tc(12.0), "Online"),
                (tc(15.0), "Playground"),
                (tc(5.0), "Canister"),
            ],
        );

        // Canlista
        self.init_entry(
            "ljnyy-wqaaa-aaaae-qaacq-cai",
            "Canlista",
            &principal_to_link("m7sm4-2iaaa-aaaab-qabra-cai"),
            "Find, publish and extend applications and services built on the Internet Computer",
            vec![(tc(5.0), "IC"), (tc(5.0), "Listing")],
        );

        // IC Drive
        self.init_entry(
            "a3hwk-dyaaa-aaaab-qaa4a-cai",
            "IC Drive",
            &principal_to_link("rglue-kyaaa-aaaah-qakca-cai"),
            "Secure and Private Decentralized Storage App",
            vec![
                (tc(5.0), "IC"),
                (tc(15.0), "Drive"),
                (tc(15.0), "Cloud"),
                (tc(12.0), "Dropbox"),
                (tc(10.0), "File"),
                (tc(8.0), "Storage"),
            ],
        );

        // DeFind
        self.init_entry(
            "iqcq7-kaaaa-aaaai-qanaa-cai",
            "DeFind",
            &principal_to_link("ilhm2-qyaaa-aaaai-qancq-cai"),
            "A Stake based search engine ready for the Web 3.0",
            vec![
                (tc(5.0), "IC"),
                (tc(15.0), "Find"),
                (tc(15.0), "Search"),
                (tc(15.0), "Engine"),
                (tc(15.0), "Canister"),
                (tc(12.0), "Google"),
                (tc(10.0), "Web"),
            ],
        );
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
        self.env.set_caller_overwrite(caller);
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
        self.env.unset_caller_overwrite();
    }
}
/// Converts Principals (as Strings) into verified urls
fn principal_to_link(principal: &str) -> String {
    format!("https://{}.ic0.app", principal)
}

fn tc(tc: f64) -> u64 {
    (tc * 1_000_000_000f64) as u64
}
