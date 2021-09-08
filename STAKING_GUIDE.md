# Staking Guide

Set up some environment variables what will come in handy:

```bash
export PRINCIPAL=$(dfx identity --network ic get-principal)
export WALLET=$(dfx identity --network ic get-wallet)
```

Add a website description to your account:

```bash
dfx canister --network ic call search set_description '(record { name="DeFind"; link="defind.ic0.app";description="The Internet Computers search engine" })'
```

View a list of owned websites:

```
dfx canister --network ic call search get_websites '()'
```

To see how many unstaked cycles you have:

```
dfx canister --network ic call search get_unstaked_cycles '()'
```

To deposit cycles to be staked run following command:

```
dfx canister --network ic --wallet $WALLET call --with-cycles 10000 search deposit_cycles "(principal \"${PRINCIPAL}\", 10000:nat64)"
```

Stake deposited cycles on search terms:

```
dfx canister --network ic call search stake '("defind.ic0.app", vec { variant { Add=record { term="search"; value=1000:nat64 } } })'
```

Remove stakes on search terms:

```
dfx canister --network ic call search stake '("defind.ic0.app", vec { variant { Remove=record { term="search"; value=1000:nat64 } } })'
```