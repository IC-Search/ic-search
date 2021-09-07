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
            None => return Vec::new(),
        }
    }

    fn set_description(&mut self, website: WebsiteDescription) {
        // Get the caller
        let owner = self.env.get_non_anon_caller();

        // Check if the principal has any websites.
        if !self.website_owners.contains_key(&owner) {
            // Adds an empty vector if principal does not have websites.
            self.website_owners.insert(owner, Vec::new());
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
        let website = Website {
            owner,
            link: link.clone(),
        };

        // Remove from websites
        self.websites.remove(&website);

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
            self.staked_terms.insert(term, filtered_stakes);
        }

        // Update the unstaked_deposits for the owner.
        self.unstaked_deposits
            .entry(owner)
            .and_modify(|deposits| *deposits += total_staked_cycles)
            .or_insert(0);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
}
