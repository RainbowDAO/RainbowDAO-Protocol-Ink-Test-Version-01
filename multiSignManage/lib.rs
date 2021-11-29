#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::multisign::{
    Multisign,
};
use ink_lang as ink;

#[ink::contract]
mod multisign {
    use alloc::string::String;
    //Defining a mutable array
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    //trade information
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Transaction{
        to:AccountId,
        amount:u128,
        signature_count:i32,
        signatures: BTreeMap<AccountId, i32>,

    }

    
    #[ink(storage)]
    pub struct Multisign {
        owner:AccountId,
        transaction_id:u64,  //information id
        manager:Vec<AccountId>, //A collection of addresses for the multi-check management collection
        min_sign_count:i32, //Minimum number of transaction signatures
        transactions:StorageHashMap<u64,Transaction>, //transaction set information id->Transaction
    }
    impl Multisign {
        #[ink(constructor)]
        pub fn new (owner:AccountId,min_sign_count:i32) ->Self{
            let mut manager:Vec<AccountId>=Vec::new();
            manager.push(owner);
            Self{
                owner:Self::env().caller(),
                transaction_id:0,
                manager,
                transactions:StorageHashMap::new(),
                min_sign_count,
            }
        }
        ///Create a trading
        #[ink(message)]
        pub fn creat_transfer(&mut self,to:AccountId,amount:u128)->bool{
            self.caller_is_manager();
            let from = self.env().caller();
            self.transaction_id +=1;
            assert_eq!(self.env().balance()>=amount,true);
            self.transactions.insert(self.transaction_id,
                Transaction{
                    to,
                    amount,
                    signature_count:0,
                    signatures: BTreeMap::new(),
                }
            );
            
            true
            
        }

        ///Signature trading
        #[ink(message)]
        pub fn sign_transaction(&mut self, transaction_id: u64) -> bool {
            self.caller_is_manager();
            let from = self.env().caller();
            //Obtain transaction information by transaction ID
            let mut t = self.transactions.get_mut(&transaction_id).unwrap();
            //Check whether a signature is available
            let if_sign = t.signatures.get(&from);
            assert!(if_sign == None);
            //1 indicates a signature.the administrator's address and signature are added to the signatures collection
            t.signatures.insert(from, 1);
            t.signature_count += 1;
            let addr = t.to;
            let num = t.amount;
            //Determine the number of signatures and transfer the specified number of assets when the conditions are met
            if t.signature_count >= self.min_sign_count {
                self.env().transfer(addr, num);
            }

            true
        }
        ///Obtain transaction information by transaction ID
        #[ink(message)]
        pub fn get_transaction(&self,trans_id: u64) -> Transaction {
            self.transactions.get(&trans_id).unwrap().clone()
        }
        ///add admin
        #[ink(message)]
        pub fn add_manager(&mut self,addr: AccountId) -> bool {
            self.caller_is_owner();
            self.manager.push(addr);
            true
        }
        ///Removing an Administrator
        #[ink(message)]
        pub fn remove_manager(&mut self,addr: AccountId) -> bool {
            self.caller_is_owner(); 
            self.manager.contains(&addr)   
        }
        ///Is it the owner himself
        fn caller_is_owner(&self) -> bool{
            self.owner == self.env().caller()
        }
        ///Is it the administrator
        fn caller_is_manager(&self) -> bool {
            let caller = self.env().caller();
            self.manager.contains(&caller) || self.owner == caller

        }

    }
}
