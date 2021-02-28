#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::services_market::ServicesMarket;
pub use self::services_market::Service;

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

    // service information
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Service {
        // service id
        pub id: u64,
        // service uuid
        pub uuid: String,
        // service's provider name
        pub provider_name: String,
        // service's provider owner
        pub provider_owner: AccountId,
        pub create_time: u64,
        pub name: String,
        pub logo: String,
        pub desc: String,
        pub usage: String,
        pub price_plan: String,
        pub declaimer: String,
    }

    #[ink(storage)]
    pub struct ServicesMarket {
        // Store a contract owner
        owner: AccountId,
        services_index: u64,
        // Store services
        services_map: StorageHashMap<u64, Service>,
        services_map_by_uuid: StorageHashMap<String, u64>,
    }

    #[ink(event)]
    pub struct AddServiceEvent {
        service_id: u64,
        service_uuid: String,
        provider_owner: AccountId,
        create_time: u64,
    }

    impl ServicesMarket {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            Self {
                owner: controller,
                services_index: 100000,
                services_map: Default::default(),
                services_map_by_uuid: Default::default(),
            }
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn add_service(&mut self, uuid: String, name: String, desc: String, logo: String, create_time: u64,
                           provider_name: String, provider_owner: AccountId, usage: String, price_plan: String, declaimer: String) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.owner, true);
            assert_eq!(self.services_index + 1 > self.services_index, true);
            self.services_map.insert(self.services_index, Service {
                id: self.services_index,
                uuid: uuid.clone(),
                name,
                create_time,
                provider_name,
                provider_owner,
                desc,
                logo,
                price_plan,
                usage,
                declaimer,
            });
            self.services_map_by_uuid.insert(uuid.clone(), self.services_index);
            self.env().emit_event(AddServiceEvent {
                service_id: self.services_index,
                service_uuid: uuid,
                provider_owner,
                create_time,
            });
            self.services_index += 1;
            true
        }

        /// query service by id
        #[ink(message)]
        pub fn query_service_by_id(&self, id: u64) -> Service {
            self.services_map.get(&id).unwrap().clone()
        }

        /// query service by uuid
        #[ink(message)]
        pub fn query_service_by_uuid(&self, uuid: String) -> Service {
            let id = self.services_map_by_uuid.get(&uuid).unwrap();
            self.services_map.get(&id).unwrap().clone()
        }

        /// query services
        #[ink(message)]
        pub fn list_services(&self) -> Vec<Service> {
            let mut service_vec = Vec::new();
            let mut iter = self.services_map.values();
            let mut item = iter.next();
            while item.is_some() {
                service_vec.push(item.unwrap().clone());
                item = iter.next();
            }
            service_vec
        }
    }
}