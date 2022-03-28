# `Voting Contract` 

This is an example smart contract on the [NEAR protocol](https://near.org/developers/). This example keeps track of votes for a proposal that you can deploy on the NEAR blockchain. These are the steps to deploy to the testnet.

## Instructions

The example is intended for developers and some level of techincal expertise is assumed. If you don't know what terminal is, this probably isn't for you.


### Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install `near-cli` (NEAR command line)

```
npm install -g near-cli
```

### Create NEAR Wallet

You need to do this on the testnet otherwise you'll be paying for gas fees.

[wallet.testnet.near.org](https://wallet.testnet.near.org/)

Write your wallet address somewhere, we'll need it later. It should look something like this: `mytestaccount.testnet`





### Building the contract

First download the code from this repo.

```
git clone git@github.com:fioritonicolas/voting-contract.git
cd voting-contract
```

Next test and compile the rust code

```
cargo test -- --nocapture
cargo build --target wasm32-unknown-unknown --release

```

Excellent, this should give you something to deploy. Lets deploy the contract to the testnet. This is where you will need your wallet address from before.

```
near deploy --wasmFile target/wasm32-unknown-unknown/release/voting_contract.wasm --accountId mytestaccount.testnet
```

This should return with a URL that lets you view your contract on the blockchain. It will be a transaction like this:

[explorer.testnet.near.org/transactions/HpyWnTM4KUnt45MoQiAgwco2pMSQqDmoSn2CwtU6N4mU](https://explorer.testnet.near.org/transactions/HpyWnTM4KUnt45MoQiAgwco2pMSQqDmoSn2CwtU6N4mU)

🚨 It's important to note that your wallet address and contract name are the same! 

### Voting on the contract

First you need to login

```
near-cli login
```

Right now there are two proposals that you can vote on:

1. NFT MAKER APP
2. HomePage

Here's how you vote for NFT Maker

```
near call mytestaccount.testnet vote '{"proposal_name":"NFT MAKER APP"}' --accountId mytestaccount.testnet
```


✅ **Congrats!** You deployed and voted on a smart contract you deploye to the blockchain.


## Some votes on other test contracts

- **First vote**  
[explorer.testnet.near.org/transactions/CRaR8mLaUpyRqrXWsq4jY1NTjGTviZta8xAKp4f3r2W1](https://explorer.testnet.near.org/transactions/CRaR8mLaUpyRqrXWsq4jY1NTjGTviZta8xAKp4f3r2W1)


- **Trying to vote with the same account in the same proposal you can´t**
[explorer.testnet.near.org/transactions/C8XkEP93VHiTWTzbtRXUYMxFfXNQ58TqRysPZyJXBjRM](https://explorer.testnet.near.org/transactions/C8XkEP93VHiTWTzbtRXUYMxFfXNQ58TqRysPZyJXBjRM)


- **Voting from a different account:**  
[explorer.testnet.near.org/transactions/6rUuM8LHJUBWA9FYhDncF7Z3hkijb6UPLvTcgov7WcDL](https://explorer.testnet.near.org/transactions/6rUuM8LHJUBWA9FYhDncF7Z3hkijb6UPLvTcgov7WcDL)

