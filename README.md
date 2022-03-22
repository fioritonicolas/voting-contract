## This contract will keep track of votes for different proposal on block chain

Right now it only has two proposal and it does not allow any aditional proposal so you can vote either for

- NFT MAKER APP
- HomePage

### Contract Name

votingnico.testnet

Deployed https://explorer.testnet.near.org/transactions/HpyWnTM4KUnt45MoQiAgwco2pMSQqDmoSn2CwtU6N4mU

To vote with the near-cli installed and logged in:

To vote for NFT MAKER APP
```near call votingnico.testnet vote '{"proposal_name":"NFT MAKER APP"}' --accountId YOUR_ACCOUNT.testnet```
To vote for Homepage
```near call votingnico.testnet vote '{"proposal_name":"HomePage"}' --accountId YOUR_ACCOUNT.testnet```


To test this contract:

```cargo test -- --nocapture```

To build this contract:

```cargo build --target wasm32-unknown-unknown --release```

To deploy this contract:

```near deploy --wasmFile target/wasm32-unknown-unknown/release/voting_contract.wasm --accountId votingnico.testnet```

