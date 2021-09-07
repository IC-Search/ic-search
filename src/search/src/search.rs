use crate::{AppState, Environment, WebsiteDescription};
use crate::{Website, APP};
use ic_cdk_macros::query;
use std::cmp::Reverse;
use std::collections::HashMap;

type Score = u64;

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
        // - Initialize Score map
        let mut score: HashMap<Website, Score> = HashMap::new();

        // Calculate
        for term in terms {
            // Get stake
            let empty_vec = vec![];
            let stakes = self.staked_terms.get(&term).unwrap_or(&empty_vec);

            // Calculate the scores
            for (stake, website) in stakes {
                score
                    .entry(website.clone())
                    .and_modify(|score| *score += stake)
                    .or_insert(*stake);
            }
        }

        // Turn score map into vec and sort descending by score
        let mut score: Vec<(Website, u64)> = score.drain().collect();
        score.sort_by_key(|(_, score)| Reverse(*score));

        score
            // Chunk by `entries_per_page`
            .chunks(entries_per_page as usize)
            // Take nths page
            .nth(page as usize)
            // Map the page of website keys to a page of website descriptions
            .map(|page| {
                page.iter()
                    .filter_map(|(website, _)| {
                        self.websites.get(website).map(|website| website.clone())
                    })
                    .collect::<Vec<WebsiteDescription>>()
            })
            // Return empty vector if the nth page does not exist
            .unwrap_or_default()
    }
}

// TODO: Tests
