//! This contract implements a voting system per different ideas.
//!
//! The contract provides methods to [vote]  and
//! [get it's current value][get_vote]
//!
//! [vote]: struct.Counter.html#method.increment
//! [get_vote]: struct.Counter.html#method.get_num

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};
// use near_sdk::{AccountId};


near_sdk::setup_alloc!();


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal{
    name: String,
    description : String,
    vote_count : i64, // i64 is signed. max votes allowed per proposal 9223372036854776000
    votes: LookupMap<String, i8>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposals{
    proposals: LookupMap<String, Proposal> // key is the proposal name
}

impl Default for Proposals {
    fn default() -> Self {

        let proposal_1 = Proposal{
            name: "NFT MAKER APP".to_string(),
            description: "The Nft Maker App".to_string(),
            vote_count: 0,
            votes: LookupMap::new(0)
        };

        let proposal_2 = Proposal{
            name: "HomePage".to_string(),
            description: "The Home App for your Wallet".to_string(),
            vote_count: 0,
            votes: LookupMap::new(0)
        };

        let mut initial_proposals = LookupMap::new(0);
        initial_proposals.insert(&proposal_1.name, &proposal_1);
        initial_proposals.insert(&proposal_2.name, &proposal_2);

        Self {
            proposals: initial_proposals,
        }
    }
}

#[near_bindgen]
impl Proposals {

    pub fn get_votes_for_proposal(&self, proposal_name:String) -> i64 {

        let proposal = self.proposals.get(&proposal_name);
        return proposal.unwrap().vote_count;

    }


    pub fn vote(&mut self, proposal_name: String) {

        let voter = env::signer_account_id();
        // TODO if the proposal does not exist add a new one?
        let mut proposal = self.proposals.get(&proposal_name).unwrap();

        if proposal.votes.contains_key(&voter){
            let log_message = format!("You have already vote for {}", proposal_name);
            env::log(log_message.as_bytes());
            return;
        }

        proposal.votes.insert(&voter, &1);
        proposal.vote_count += 1;

        self.proposals.insert(&proposal_name, &proposal);

        let log_message = format!("You have vote for {} now it has {} votes", proposal_name, proposal.vote_count);
        env::log(log_message.as_bytes());
        after_proposal_vote();
    }

}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_proposal_vote() {
    env::log("Thank you for voting".as_bytes());
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn vote() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate proposal variable with the defaults values
        let proposal = "NFT MAKER APP".to_string();

        let mut proposals = Proposals::default();
        println!(" VOTES BEFORE {}", proposals.get_votes_for_proposal(proposal.clone()));
        proposals.vote(proposal.clone());
        let votes = proposals.get_votes_for_proposal(proposal.clone());
        println!("Votes for NFT MAKER APP now: {}", votes);
        // confirm that we received 1 comparing to votes
        assert_eq!(1, votes);
    }
}