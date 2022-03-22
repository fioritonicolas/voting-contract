## This contract will keep track of votes for different proposal on near blockchain

Right now it only has two proposal and it does not allow any aditional proposal so you can vote either for

- NFT MAKER APP
- HomePage

### Contract Name

votingnico.testnet

Deployed https://explorer.testnet.near.org/transactions/HpyWnTM4KUnt45MoQiAgwco2pMSQqDmoSn2CwtU6N4mU

Some votes from this contract:

First vote:

https://explorer.testnet.near.org/transactions/CRaR8mLaUpyRqrXWsq4jY1NTjGTviZta8xAKp4f3r2W1

Trying to vote with the same account in the same proposal you can´t:

https://explorer.testnet.near.org/transactions/C8XkEP93VHiTWTzbtRXUYMxFfXNQ58TqRysPZyJXBjRM

Voting from a different account:

https://explorer.testnet.near.org/transactions/6rUuM8LHJUBWA9FYhDncF7Z3hkijb6UPLvTcgov7WcDL


To vote you need to have the near-cli installed and be logged in:

To vote for NFT MAKER APP:

```near call votingnico.testnet vote '{"proposal_name":"NFT MAKER APP"}' --accountId YOUR_ACCOUNT.testnet```

To vote for Homepage: 

```near call votingnico.testnet vote '{"proposal_name":"HomePage"}' --accountId YOUR_ACCOUNT.testnet```


To test this contract:

```cargo test -- --nocapture```

To build this contract:

```cargo build --target wasm32-unknown-unknown --release```

To deploy this contract:

```near deploy --wasmFile target/wasm32-unknown-unknown/release/voting_contract.wasm --accountId votingnico.testnet```

