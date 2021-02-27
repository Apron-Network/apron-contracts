#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod services_market {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        }
    };

    // service infomation
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct ServiceInfo {
        id: u64,
        name: String,
        desc: String,
        thumb: String,
    }

    #[ink(storage)]
    pub struct ServicesMarket {
        // Store a contract owner
        owner: AccountId,
        services_index: u64,
        // Store services
        services_map: StorageHashMap<u64, ServiceInfo>,
    }

    impl ServicesMarket {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            Self {
                owner: controller,
                services_index: 100000,
                services_map: Default::default(),
            }
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn add_service(&mut self, name: String, desc: String, thumb: String) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.owner, true);
            assert_eq!(self.services_index + 1 > self.services_index, true);
            self.services_map.insert(self.services_index, ServiceInfo{
                id: self.services_index,
                name,
                desc,
                thumb,
            });
            self.services_index += 1;
            true
        }

        /// query service's owner
        #[ink(message)]
        pub fn query_service(&self, id: u64) -> ServiceInfo {
            self.services_map.get(&id).unwrap().clone()
        }
    }
}
