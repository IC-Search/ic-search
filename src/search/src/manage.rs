use crate::{AppState, Environment, WebsiteDescription};
use crate::{Website, APP};
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
        // Get the caller
        let owner = self.env.get_non_anon_caller();

        // Get owned website descriptions
        match self.website_owners.get(&owner) {
            Some(websites) => websites
                .iter()
                .map(|link| {
                    let website = Website {
                        owner,
                        link: link.clone(),
                    };
                    self.websites
                        .get(&website)
                        .expect("Unable to find owned website in websites.")
                        .clone()
                })
                .collect::<Vec<WebsiteDescription>>(),
            None => vec![],
        }
    }

    pub(crate) fn set_description(&mut self, website: WebsiteDescription) {
        // Get the caller
        let owner = self.env.get_non_anon_caller();

        // Check if the principal has any websites.
        if !self.website_owners.contains_key(&owner) {
            // Adds an empty vector if principal does not have websites.
            self.website_owners.insert(owner, vec![]);
        }

        // Get the principal's websites.
        let owned_websites = self
            .website_owners
            .get_mut(&owner)
            .expect("Unable to find principal's owned websites.");

        // Check if the website is owned.
        // If not, add it to owned_websites.
        let website_is_owned = owned_websites.iter().any(|link| link == &website.link);
        if !website_is_owned {
            owned_websites.push(website.link.clone());
        }

        // Add the website to owned websites.
        let owned_website = Website {
            owner,
            link: website.link.clone(),
        };
        self.websites.insert(owned_website, website);
    }

    fn remove_website(&mut self, link: String) {
        // Get the caller
        let owner = self.env.get_non_anon_caller();

        // Get the principal's owned websites.
        let owned_websites = self
            .website_owners
            .get_mut(&owner)
            .expect("Principal does not have any owned websites.");

        // Get the position in owned_websites for the provided link.
        // Panic if it cannot be found.
        let index = owned_websites
            .iter()
            .position(|owned_link| owned_link == &link)
            .expect("Principal does not own website.");

        // Remove the link from owned websites
        owned_websites.remove(index);

        // Make the website key
        let website = Website { owner, link };

        // Remove from websites
        self.websites.remove(&website);

        // Remove from website_owners
        let owned_links = self.website_owners.get(&owner).cloned().unwrap_or_default();
        let filtered_owned_links: Vec<String> = owned_links
            .into_iter()
            .filter(|link| link != &website.link)
            .collect();
        if filtered_owned_links.is_empty() {
            self.website_owners.remove(&owner);
        } else {
            self.website_owners
                .insert(owner.clone(), filtered_owned_links);
        }

        // If there are currently no stakes on the website, exit early.
        if !self.staked_websites.contains_key(&website) {
            return;
        }

        // Remove the website from the staked_websites.
        let staked = self
            .staked_websites
            .remove(&website)
            .expect("Stake could not be found for principal.");

        // Remove the staked_terms and total the staked cycles.
        let mut total_staked_cycles: u64 = 0;
        for (staked_cycles, term) in staked {
            let stakes = self
                .staked_terms
                .get(&term)
                .expect("Stake could not be found for term.");
            let filtered_stakes = stakes
                .iter()
                .filter_map(|stake| {
                    if stake.1 != website {
                        Some(stake.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<(u64, Website)>>();
            total_staked_cycles += staked_cycles;
            if filtered_stakes.is_empty() {
                self.staked_terms.remove(&term);
            } else {
                self.staked_terms.insert(term, filtered_stakes);
            }
        }

        // Update the unstaked_deposits for the owner.
        self.unstaked_deposits
            .entry(owner)
            .and_modify(|deposits| *deposits += total_staked_cycles)
            .or_insert(total_staked_cycles);
    }
}

#[cfg(test)]
mod test {
    use candid::Principal;

    use super::*;
    use crate::test::*;

    fn test_state_for_managing(
        env: TestEnvironment,
        mut unstaked_deposits: Vec<(Principal, u64)>,
        mut websites: Vec<(Website, WebsiteDescription)>,
        mut website_owners: Vec<(Principal, Vec<String>)>,
        mut staked_websites: Vec<(Website, Vec<(u64, String)>)>,
        mut staked_terms: Vec<(String, Vec<(u64, Website)>)>,
    ) -> AppState<TestEnvironment> {
        AppState {
            env,
            unstaked_deposits: unstaked_deposits.drain(..).collect(),
            website_owners: website_owners.drain(..).collect(),
            websites: websites.drain(..).collect(),
            staked_websites: staked_websites.drain(..).collect(),
            staked_terms: staked_terms.drain(..).collect(),
        }
    }

    #[test]
    fn test_get_descriptions() {
        let app = test_state_for_managing(
            TestEnvironment::new(),
            vec![],
            vec![(test_website(0), test_website_description(0))],
            vec![(test_principal_id(0), vec![test_url(0)])],
            vec![],
            vec![],
        );
        app.env.set_caller(test_principal_id(0));
        let websites = app.get_websites();
        assert_eq!(websites.len(), 1);
        let websites_0 = websites.get(0).cloned().unwrap_or(WebsiteDescription {
            name: String::from("invalid"),
            link: String::from("invalid"),
            description: String::from("invalid"),
        });
        assert_eq!(websites_0.name, test_website_name(0));
        assert_eq!(
            websites_0.description,
            test_website_description(0).description
        );
        assert_eq!(websites_0.link, test_url(0));
    }

    #[test]
    fn test_add_website_description() {
        let mut app = test_state_for_managing(
            TestEnvironment::new(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );
        let principal = test_principal_id(0);
        app.env.set_caller(principal.clone());
        let website = test_website(0);
        let website_description = test_website_description(0);
        app.set_description(website_description.clone());

        let owned_links = app
            .website_owners
            .get(&principal)
            .cloned()
            .unwrap_or_default();
        assert!(!owned_links.is_empty());
        let link_0 = owned_links.get(0).cloned().unwrap_or_default();
        assert_eq!(link_0, test_url(0));

        let found_website_description = app
            .websites
            .get(&test_website(0))
            .cloned()
            .unwrap_or(test_website_description(1));
        assert_eq!(found_website_description.name, website_description.name);
        assert_eq!(found_website_description.link, website_description.link);
        assert_eq!(
            found_website_description.description,
            website_description.description
        );
    }

    #[test]
    fn test_update_website_description() {
        let principal = test_principal_id(0);
        let mut website_description = test_website_description(0);
        website_description.description = String::from("test");
        let mut app = test_state_for_managing(
            TestEnvironment::new(),
            vec![],
            vec![(test_website(0), test_website_description(0))],
            vec![(principal.clone(), vec![test_url(0)])],
            vec![],
            vec![],
        );
        app.env.set_caller(principal.clone());
        let website = test_website(0);
        app.set_description(website_description.clone());

        let owned_links = app
            .website_owners
            .get(&principal)
            .cloned()
            .unwrap_or_default();
        assert!(!owned_links.is_empty());
        assert!(owned_links.len() == 1);
        let link_0 = owned_links.get(0).cloned().unwrap_or_default();
        assert_eq!(link_0, test_url(0));

        let found_website_description = app
            .websites
            .get(&test_website(0))
            .cloned()
            .unwrap_or(test_website_description(1));
        assert_eq!(found_website_description.name, website_description.name);
        assert_eq!(found_website_description.link, website_description.link);
        assert_eq!(found_website_description.description, "test");
    }

    #[test]
    fn test_remove_website_only_one_website_no_stake() {
        let principal = test_principal_id(0);
        let mut app = test_state_for_managing(
            TestEnvironment::new(),
            vec![],
            vec![(test_website(0), test_website_description(0))],
            vec![(principal.clone(), vec![test_url(0)])],
            vec![],
            vec![],
        );
        app.env.set_caller(principal);
        app.remove_website(test_url(0));

        assert!(app.websites.is_empty());
        assert!(app.website_owners.is_empty());
    }

    #[test]
    fn test_remove_website_only_one_website_with_stake() {
        let test_term = String::from("test");
        let principal = test_principal_id(0);
        let mut app = test_state_for_managing(
            TestEnvironment::new(),
            vec![],
            vec![(test_website(0), test_website_description(0))],
            vec![(principal.clone(), vec![test_url(0)])],
            vec![(test_website(0), vec![(100, test_term.clone())])],
            vec![(test_term.clone(), vec![(100, test_website(0))])],
        );
        app.env.set_caller(principal);
        app.remove_website(test_url(0));

        assert!(app.websites.is_empty());
        assert!(app.website_owners.is_empty());
        let unstaked_deposit = app.unstaked_deposits.get(&principal).cloned().unwrap_or(0);
        assert_eq!(unstaked_deposit, 100);
    }

    #[test]
    fn test_remove_website_multiple_websites_with_stake() {
        let test_term = String::from("test");
        let principal = test_principal_id(0);
        let mut app = test_state_for_managing(
            TestEnvironment::new(),
            vec![
                (principal.clone(), 900),
                (test_principal_id(1), 2000),
                (test_principal_id(2), 3000),
            ],
            vec![(test_website(0), test_website_description(0))],
            vec![(principal.clone(), vec![test_url(0)])],
            vec![
                (test_website(0), vec![(100, test_term.clone())]),
                (test_website(1), vec![(100, test_term.clone())]),
                (test_website(2), vec![(100, test_term.clone())]),
            ],
            vec![(
                test_term.clone(),
                vec![
                    (100, test_website(0)),
                    (100, test_website(1)),
                    (100, test_website(2)),
                ],
            )],
        );
        app.env.set_caller(principal);
        app.remove_website(test_url(0));

        assert!(app.websites.is_empty());
        assert!(app.website_owners.is_empty());
        let unstaked_deposit_0 = app.unstaked_deposits.get(&principal).cloned().unwrap_or(0);
        assert_eq!(unstaked_deposit_0, 1000);

        let unstaked_deposit_1 = app
            .unstaked_deposits
            .get(&test_principal_id(1))
            .cloned()
            .unwrap_or(0);
        assert_eq!(unstaked_deposit_1, 2000);
        let unstaked_deposit_2 = app
            .unstaked_deposits
            .get(&test_principal_id(2))
            .cloned()
            .unwrap_or(0);
        assert_eq!(unstaked_deposit_2, 3000);

        let staked_terms = app
            .staked_terms
            .get(&test_term)
            .cloned()
            .unwrap_or_default();
        assert!(!staked_terms.is_empty());
        let term_1 = staked_terms
            .get(0)
            .cloned()
            .unwrap_or((0, test_website(20)));
        assert_eq!(100, term_1.0);
        assert_eq!(test_principal_id(1), term_1.1.owner);
        let term_2 = staked_terms
            .get(1)
            .cloned()
            .unwrap_or((0, test_website(20)));
        assert_eq!(100, term_2.0);
        assert_eq!(test_principal_id(2), term_2.1.owner);
    }
}
