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
                        self.websites
                            .get(website)
                            .map(|description| description.clone())
                    })
                    .collect::<Vec<WebsiteDescription>>()
            })
            // Return empty vector if the nth page does not exist
            .unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    fn test_state_for_search(
        env: TestEnvironment,
        mut websites: Vec<(Website, WebsiteDescription)>,
        mut staked_terms: Vec<(String, Vec<(u64, Website)>)>,
    ) -> AppState<TestEnvironment> {
        AppState {
            env,
            unstaked_deposits: HashMap::new(),
            website_owners: HashMap::new(),
            websites: websites.drain(..).collect(),
            staked_websites: HashMap::new(),
            staked_terms: staked_terms.drain(..).collect(),
        }
    }

    /// Tests that the ordering of the websites with respect to a single term are correct
    #[test]
    fn test_single_term_ordering() {
        let app = test_state_for_search(
            TestEnvironment::new(),
            vec![
                (test_website(0), test_website_description(0)),
                (test_website(1), test_website_description(1)),
                (test_website(2), test_website_description(2)),
                (test_website(3), test_website_description(3)),
            ],
            vec![(
                String::from("Test"),
                vec![
                    (1, test_website(0)),
                    (2, test_website(1)),
                    (3, test_website(2)),
                    (4, test_website(3)),
                ],
            )],
        );

        // Check that a different term yields empty result
        let result = app.search(vec![String::from("Nottest")], 0, 100);
        assert!(result.is_empty());

        // Check that with the correct term, the values are orderd in descending order
        let result = app.search(vec![String::from("Test")], 0, 100);
        assert_eq!(result[0].name, test_website_name(3));
        assert_eq!(result[1].name, test_website_name(2));
        assert_eq!(result[2].name, test_website_name(1));
        assert_eq!(result[3].name, test_website_name(0));
    }

    /// Tests that entries with more matching terms get ranked higher
    #[test]
    fn test_multi_term_ordering() {
        let app = test_state_for_search(
            TestEnvironment::new(),
            vec![
                (test_website(0), test_website_description(0)),
                (test_website(1), test_website_description(1)),
                (test_website(2), test_website_description(2)),
                (test_website(3), test_website_description(3)),
            ],
            vec![
                (
                    String::from("Term1"),
                    vec![
                        (1, test_website(0)),
                        (1, test_website(1)),
                        (1, test_website(2)),
                        (1, test_website(3)),
                    ],
                ),
                (
                    String::from("Term2"),
                    vec![
                        (1, test_website(1)),
                        (1, test_website(2)),
                        (1, test_website(3)),
                    ],
                ),
                (
                    String::from("Term3"),
                    vec![(1, test_website(2)), (1, test_website(3))],
                ),
                (String::from("Term4"), vec![(1, test_website(3))]),
            ],
        );

        let search_terms: Vec<String> = ["Term1", "Term2", "Term3", "Term4"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        let result = app.search(search_terms, 0, 100);

        assert_eq!(result[0].name, test_website_name(3));
        assert_eq!(result[1].name, test_website_name(2));
        assert_eq!(result[2].name, test_website_name(1));
        assert_eq!(result[3].name, test_website_name(0));
    }
}
