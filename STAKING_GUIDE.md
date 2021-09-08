To add a website to be staked:

```
dfx canister call search set_description '(record { name="DeFind"; link="defind.ic0.app";description="The Internet Computers search engine" })'
```

To view a list of owned websites:

```
dfx canister call search get_websites '()'
```

To see how many unstaked cycles you have:

```
dfx canister call search get_unstaked_cycles '()'
```

Get your principal ID:

```
dfx identity get-principal
```

To deposit cycles to be staked run following command replacing <your principal id here> with your principal ID from the prior command:

```
dfx canister --wallet rwlgt-iiaaa-aaaaa-aaaaa-cai call --with-cycles 10000 search deposit_cycles '(principal "<your principal id here>", 10000:nat64)'
```

To stake deposited cycles on search terms:

```
dfx canister call search stake '("defind.ic0.app", vec { variant { Add=record { term="search"; value=1000:nat64 } } })'
```

To remove stake on search terms:

```
dfx canister call search stake '("defind.ic0.app", vec { variant { Remove=record { term="search"; value=1000:nat64 } } })'
```