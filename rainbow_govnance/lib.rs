#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod rainbow_govnance {

    use route_manage::RouteManage;

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };



    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct Proposal {
        // base module contract's address
        title: String,
        desc: String,
        start_block:u32,
        end_block:u32,
        for_votes:u64,
        against_votes:u64,
        owner:AccountId,
        proposal_id:u64,
        canceled:bool,
        executed:bool,
        receipts:StorageHashMap<AccountId, Receipt>,
    }
    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct Receipt {
        has_voted:bool,
        support: bool,
        votes:u64
    }


    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u64,
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        title:String
    }
    /// The kind of access allows for a storage cell.
    #[derive(
    Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo,
    )]
    pub enum ProposalState {
        Canceled,
        Pending,
        Active,
        Defeated,
        Succeeded,
        Executed,
        Expired,
        Queued
    }
    #[ink(storage)]
    pub struct RainbowGovnance {
        owner: AccountId,
        proposals:StorageHashMap<u64, Proposal>,
        voting_delay:u32,
        voting_period:u32,
        proposal_length:u64,
        route_addr:AccountId,
    }

    impl RainbowGovnance {
        #[ink(constructor)]
        pub fn new(route_addr:AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                proposals:StorageHashMap::new(),
                voting_delay:1,
                voting_period:259200, //3 days
                proposal_length:0,
                route_addr
            }
        }

        #[ink(message)]
        pub fn propose(&mut self,title:String,desc:String) -> bool {
            let start_block = self.env().block_number() + self.voting_delay;
            let end_block = start_block + self.voting_period;
            let proposal_id = self.proposal_length.clone();
            self.proposal_length += 1;
            let proposal_info = Proposal{
                proposal_id,
                title,
                desc,
                start_block,
                end_block,
                for_votes:0,
                against_votes:0,
                owner:Self::env().caller(),
                canceled:false,
                executed:false,
                receipts : StorageHashMap::new()
            };
            self.proposals.insert(proposal_id, proposal_info);
            self.env().emit_event(ProposalCreated{
                proposal_id,
                creator: self.env().caller(),
                title
            });
            true
        }
        #[ink(message)]
        pub fn state(&self,index:u64) -> ProposalState {
            let block_number = self.env().block_number();
            let proposal:Proposal =  self.proposals.get(&index).unwrap().clone();
            if proposal.canceled {return ProposalState::Canceled }
            else if block_number <= proposal.start_block { return ProposalState::Pending}
            else if block_number <= proposal.end_block { return ProposalState::Active}
            else if proposal.for_votes <= proposal.against_votes { return ProposalState::Defeated}
            else if proposal.executed { return ProposalState::Executed}
            else if block_number >  proposal.end_block{ return ProposalState::Expired}
            else { return ProposalState::Queued }
        }
        #[ink(message)]
        pub fn  exec(&mut self,index:u64,target_name:String,exec_string:String) -> bool {
            let mut proposal:Proposal = self.route_map.get(&inde).unwrap().clone();
            assert!(self.state(index) ==  ProposalState::Queued);
            let addr =  self.get_contract_addr(&target_name);

            //todo 调用其他合约

            proposal.executed = true;

            true

        }
        #[ink(message)]
        pub fn get_contract_addr(&self,target_name:String) ->AccountId {
            let route_instance: RouteManage = ink_env::call::FromAccountId::from_account_id(self.route_addr);
            return route_instance.query_route_by_name(&target_name);
        }
        #[ink(message)]
        pub fn cast_vote(&self,proposal_id:u64,support:bool) ->bool {
            let caller = Self::env().caller();
            assert!(self.state(proposal_id) ==  ProposalState::Active);
            let mut proposal:Proposal = self.route_map.get(&proposal_id).unwrap().clone();
            let receipts =  proposal.receipts.get(&caller).unwrap().clone();
            assert!(receipts.has_voted ==  false);

        }
    }


    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;
    //
    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let rainbowGovnance = RainbowGovnance::default();
    //         assert_eq!(rainbowGovnance.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut rainbowGovnance = RainbowGovnance::new(false);
    //         assert_eq!(rainbowGovnance.get(), false);
    //         rainbowGovnance.flip();
    //         assert_eq!(rainbowGovnance.get(), true);
    //     }
    // }
}