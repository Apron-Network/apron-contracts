#![cfg_attr(not(feature = "std"), no_std)]

/// SBP M1
///
/// Please double check that no logic can be moved into the runtime. Contracts are less efficient, powerful and might open security issues`.
///
/// Contract should be built with `--release` to ensure a minimal contract size. It's built in debug mode by default.
///
/// You are using a lot of String types in your contract. String's should only be used when absolutely necessary. See the ink! FAQ for more details: (https://paritytech.github.io/ink-docs/faq/#how-do-i-use-string-in-my-contract)
/// Using String pulls in logic needed for e.g. UTF-8 handling, which you might not need, but it blows up the contract file size. 
/// Also, for String dynamic allocation is needed, increasing the file size of the contract as well and being expensive in terms of performance.
/// The contract size is important, since it affects how much it costs to deploy/use the contract on chain and how high the throughput on a parachain can be.

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
        }
    };
    use page_helper::PageParams;
    use page_helper::PageResult;

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
        pub schema: String,
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

    #[ink(event)]
    pub struct UpdateServiceEvent {
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
                           provider_name: String, provider_owner: AccountId, usage: String, schema: String, price_plan: String, declaimer: String) -> bool {
            let controller = self.env().caller();
            assert_eq!(controller == self.owner, true);
            let index_opt = self.services_map_by_uuid.get(&uuid);
            if let Some(&index) = index_opt {
                self.services_map.insert(index, Service {
                    index,
                    uuid: uuid.clone(),
                    name,
                    create_time,
                    provider_name,
                    provider_owner: provider_owner.clone(),
                    desc,
                    logo,
                    price_plan,
                    usage,
                    schema,
                    declaimer,
                });
                self.env().emit_event(UpdateServiceEvent {
                    service_id: index,
                    service_uuid: uuid,
                    provider_owner,
                    create_time,
                });
                return true
            }
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
                schema,
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

        #[ink(message)]
        pub fn list_services_by_page(&self, params: PageParams) -> PageResult<Service> {
            let total = self.services_map.len() as u64;
            let (start, end, pages) = self.calPages(&params, total);
            let mut service_vec = Vec::new();
            for i in start..end {
                let opt = self.services_map.get(&i);
                if let Some(s) = opt {
                    service_vec.push(s.clone());
                }
            }
            return PageResult{
                success: true,
                err: String::from("success"),
                total,
                pages,
                page_index: params.page_index,
                page_size: params.page_size,
                data: service_vec,
            }
        }

        /// query services
        #[ink(message)]
        pub fn list_services_provider(&self, provider: AccountId, params: PageParams) -> PageResult<Service> {
            let provider_ids = self.services_map_by_provider.get(&provider).unwrap();
            let total = provider_ids.len() as u64;
            let (start, end, pages) = self.calPages(&params, total);
            let mut service_vec = Vec::new();
            for i in start..end {
                service_vec.push(self.services_map.get(&provider_ids[i as usize]).unwrap().clone());
            }
            return PageResult{
                success: true,
                err: String::from("success"),
                total,
                pages,
                page_index: params.page_index,
                page_size: params.page_size,
                data: service_vec,
            }
        }

        fn calPages(&self, params: &PageParams, total: u64) -> (u64, u64, u64) {
            let start = params.page_index * params.page_size;
            let mut end = start + params.page_size;
            if end > total {
                end = total
            }
            assert!(params.page_size <= 0 || start >= total || start < end, "wrong params");
            let mut pages = total / params.page_size;
            if total % params.page_size > 0 {
                pages += 1;
            }
            (start, end, pages)
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
            let schema = "http".to_string();
            let price_plan = "the price plan of first service".to_string();
            let declaimer = "the declaimer".to_string();

            let ret = market.add_service(uuid, name, desc, logo, create_time, provider_name, provider_addr, usage, schema, price_plan, declaimer);
            assert!(ret);

            let services = market.list_services();
            let length = services.len() as u32;
            assert!(length == 1);
            for s in services{
                let debug_msg = format!("service is {}, {}, {}, {}", &s.index, &s.uuid, &s.name, &s.provider_name);
                ink_env::debug_println(&debug_msg);
                assert_eq!("asdfsdfsdfsdfsdfsdfsdfasdf", s.uuid);
                assert_eq!("first service", s.name);
                assert_eq!("desc of service", s.desc);
                assert_eq!("logo logo", s.logo);
                assert_eq!(1615442616, s.create_time);
                assert_eq!("provider", s.provider_name);
                assert_eq!(accounts.alice, s.provider_owner);
                assert_eq!("usage of first service", s.usage);
                assert_eq!("http", s.schema);
                assert_eq!("the price plan of first service", s.price_plan);
                assert_eq!("the declaimer", s.declaimer);
            }
        }

        #[ink::test]
        fn add_service_invalid_owner() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut market = ServicesMarket::new(accounts.alice);

            let uuid = "asdfsdfsdfsdfsdfsdfsdfasdf".to_string();
            let name = "first service".to_string();
            let desc = "desc of service".to_string();
            let logo = "logo logo".to_string();
            let create_time = 1615442616;
            let provider_name = "provider".to_string();
            let provider_addr = accounts.bob;
            let usage = "usage of first service".to_string();
            let schema = "http".to_string();
            let price_plan = "the price plan of first service".to_string();
            let declaimer = "the declaimer".to_string();

            let ret = market.add_service(uuid, name, desc, logo, create_time, provider_name, provider_addr, usage, schema, price_plan, declaimer);
            assert_eq!(ret, true);
        }

        #[ink::test]
        fn query_index_works() {
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
            let schema = "http".to_string();
            let price_plan = "the price plan of first service".to_string();
            let declaimer = "the declaimer".to_string();

            let ret = market.add_service(uuid, name, desc, logo, create_time, provider_name, provider_addr, usage, schema, price_plan, declaimer);
            assert_eq!(ret, true);

            let s = market.query_service_by_index(0);
            let debug_msg = format!("service is {}, {}, {}, {}", &s.index, &s.uuid, &s.name, &s.provider_name);
            ink_env::debug_println(&debug_msg);
            assert_eq!("asdfsdfsdfsdfsdfsdfsdfasdf", s.uuid);
            assert_eq!("first service", s.name);
            assert_eq!("desc of service", s.desc);
            assert_eq!("logo logo", s.logo);
            assert_eq!(1615442616, s.create_time);
            assert_eq!("provider", s.provider_name);
            assert_eq!(accounts.alice, s.provider_owner);
            assert_eq!("usage of first service", s.usage);
            assert_eq!("http", s.schema);
            assert_eq!("the price plan of first service", s.price_plan);
            assert_eq!("the declaimer", s.declaimer);
        }

        #[ink::test]
        fn query_uuid_works() {
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
            let schema = "http".to_string();
            let price_plan = "the price plan of first service".to_string();
            let declaimer = "the declaimer".to_string();

            let ret = market.add_service(uuid, name, desc, logo, create_time, provider_name, provider_addr, usage, schema, price_plan, declaimer);
            assert_eq!(ret, true);

            let s = market.query_service_by_uuid("asdfsdfsdfsdfsdfsdfsdfasdf".to_string());
            let debug_msg = format!("service is {}, {}, {}, {}", &s.index, &s.uuid, &s.name, &s.provider_name);
            ink_env::debug_println(&debug_msg);
            assert_eq!("asdfsdfsdfsdfsdfsdfsdfasdf", s.uuid);
            assert_eq!("first service", s.name);
            assert_eq!("desc of service", s.desc);
            assert_eq!("logo logo", s.logo);
            assert_eq!(1615442616, s.create_time);
            assert_eq!("provider", s.provider_name);
            assert_eq!(accounts.alice, s.provider_owner);
            assert_eq!("usage of first service", s.usage);
            assert_eq!("http", s.schema);
            assert_eq!("the price plan of first service", s.price_plan);
            assert_eq!("the declaimer", s.declaimer);
        }

        #[ink::test]
        fn list_by_provider_works() {
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
            let schema = "http".to_string();
            let price_plan = "the price plan of first service".to_string();
            let declaimer = "the declaimer".to_string();

            let ret = market.add_service(uuid, name, desc, logo, create_time, provider_name, provider_addr, usage, schema, price_plan, declaimer);
            assert!(ret);

            let services = market.list_services_provider(accounts.alice);
            let length = services.len() as u32;
            assert!(length == 1);
            for s in services{
                let debug_msg = format!("service is {}, {}, {}, {}", &s.index, &s.uuid, &s.name, &s.provider_name);
                ink_env::debug_println(&debug_msg);
                assert_eq!("asdfsdfsdfsdfsdfsdfsdfasdf", s.uuid);
                assert_eq!("first service", s.name);
                assert_eq!("desc of service", s.desc);
                assert_eq!("logo logo", s.logo);
                assert_eq!(1615442616, s.create_time);
                assert_eq!("provider", s.provider_name);
                assert_eq!(accounts.alice, s.provider_owner);
                assert_eq!("usage of first service", s.usage);
                assert_eq!("http", s.schema);
                assert_eq!("the price plan of first service", s.price_plan);
                assert_eq!("the declaimer", s.declaimer);
            }
        }
    }
}
