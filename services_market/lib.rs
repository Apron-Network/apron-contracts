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
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Service {
        // service id
        pub index: u64,
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
        services_map_by_provider: StorageHashMap<AccountId, Vec<u64>>,
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
                services_index: 0,
                services_map: Default::default(),
                services_map_by_uuid: Default::default(),
                services_map_by_provider: Default::default(),
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
                index: self.services_index,
                uuid: uuid.clone(),
                name,
                create_time,
                provider_name,
                provider_owner: provider_owner.clone(),
                desc,
                logo,
                price_plan,
                usage,
                declaimer,
            });
            self.services_map_by_uuid.insert(uuid.clone(), self.services_index);
            let id_list = self.services_map_by_provider.entry(provider_owner).or_insert(Vec::new());
            id_list.push(self.services_index);
            self.env().emit_event(AddServiceEvent {
                service_id: self.services_index,
                service_uuid: uuid,
                provider_owner,
                create_time,
            });
            self.services_index += 1;
            true
        }

        /// query service by index
        #[ink(message)]
        pub fn query_service_by_index(&self, index: u64) -> Service {
            self.services_map.get(&index).unwrap().clone()
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

        /// query services
        #[ink(message)]
        pub fn list_services_provider(&self, provider: AccountId) -> Vec<Service> {
            let mut service_vec = Vec::new();
            let mut iter = self.services_map_by_provider.get(&provider).unwrap().into_iter();
            let mut item = iter.next();
            while item.is_some() {
                service_vec.push(self.services_map.get(item.unwrap()).unwrap().clone());
                item = iter.next();
            }
            service_vec
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut market = ServicesMarket::new(accounts.alice);

            let uuid = "asdfsdfsdfsdfsdfsdfsdfasdf".to_string();
            let name = "first service".to_string();
            let desc = "desc of service".to_string();
            let logo = "logo logo".to_string();
            let create_time = 1615442616;
            let provider_name = "provider".to_string();
            let provider_addr = accounts.alice;
            let usage = "usage of first service".to_string();
            let price_plan = "the price plan of first service".to_string();
            let declaimer = "the declaimer".to_string();

            market.add_service(uuid, name, desc, logo, create_time, provider_name, provider_addr, usage, price_plan, declaimer);

            let services = market.list_services();
            let length = services.len() as u32;
            assert!(length == 1);
            for s in services{
                let debug_msg = format!("service is {}, {}, {}, {}", &s.index, &s.uuid, &s.name, &s.provider_name);
                ink_env::debug_println(&debug_msg)
            }
        }
    }
}
