#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::route_manage::{
    RouteManage,
};

#[ink::contract]
mod route_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };
    #[ink(storage)]
    pub struct RouteManage {
        owner:AccountId,
        route_map:StorageHashMap<String,AccountId>,
    }

    impl RouteManage {
        #[ink(constructor)]
        pub fn new() -> Self {
             Self {
                owner:Self::env().caller(),
                route_map : StorageHashMap::new(),
            }
        }
        fn only_core(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }

        #[ink(message)]
        pub fn add_route(&mut self, name: String,value:AccountId) -> bool {
            self.only_core(Self::env().caller());
            self.route_map.insert(name,value);
            true
        }

        #[ink(message)]
        pub fn query_route_by_name(&self, name: String) -> AccountId {
            self.route_map.get(&name).unwrap().clone()
        }
        #[ink(message)]
        pub fn change_route(&mut self,name:String,value:AccountId) -> bool {
             self.only_core(Self::env().caller());
            self.route_map[&name] = value;
            true
        }
    }

}
