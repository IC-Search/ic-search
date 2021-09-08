# DeFind staking based Search Engine for the Web 3.0

DeFind is a project to tackle one of the most daunting tasks of the web in a new way: **finding stuff**.


Web 2.0 Search Engines such as Google employ web crawlers to feed data into a search algorithm.

This approach creates a weird mix of incentives that we consider harmful to the open web:

- There is an incentive of website owners to trick the web crawlers, to make their content more relevant than it is
- Using traffic statistics creates a grey market of traffic dealers employing bots or clickworkers to generate artificial traffic
- The algorithm has to be kept a trade secret in order to not be exploitable
- Small changes to the algorithm can yield vastly different search results, which can potentially threaten the existence of a smaller website

As a result, Search Engine Optimizers (SEO) find that the efficacy of their work is hard to verify and there are a lot of bad actors among their ranks.
There is just too much "magic" involved.


DeFind turns the game around.
Instead of a secret algorithm and a webcrawler, DeFind employs a public algorithm and let the owners of websites / canisters choose the input to the algorithm themselves.

In order to not be exploitable, the advertisers have to back up their search terms with value, i.e. stake on the search terms under which they want their service to appear.

With DeFind:
- Advertisers know exactly, what they get for their money
- SEO is data science and not voodoo
- The algorithm itself can be simple (and even be changed / improved) later, without risking smaller websites' existence

## MVP

The MVP as part of the Dfinihack hackathon showcases:

- Entering search terms and getting search results
- Registering websites using Internet Identity
- Staking terms on websites

## Roadmap

There are a couple of big items on the Roadmap

### Algorithm

The current algorithm is designed to be as simple as possible to show the possibities of the DeFind approach. It comes with a bunch of limitations:

- Search terms need to be exact
- Alost similar search terms are counted as totally different terms

Therefore, a future implementation might accomodate a smarter algorithm, that has some form of distance measurement and allows for an elastic search like experience.
If a search term is close to a staked term, the staked term might still controbute to a websites score, for example.
### Monetization

Developing and running a service costs money. In the MVP implementation, the service is effectively free, as advertisers can unstake their websites and regain all their money, even though we had costs running the service and provided value to the advertiser.

A monetization scheme is needed. We envision a scheme, where all stakes have a slight negative interest, proportional to the total amount of cycles burned by running the service.

### Public Analytics

SEOs will want to know the hottest terms to stake on in order to get the most out of their allocated stakes. In order to keep this a zero information game, we might collect this data as well on our side. Note that the traffic data will not be used as an input to the algorithm but only as guidance to SEOs, such that they can make the decisions that are best for their companies.