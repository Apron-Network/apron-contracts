#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod services_statistics {

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
    use services_market::ServicesMarket;
    use services_market::Service;

    // service information
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct UsageRecord {
        id: u64,
        service_uuid: String,
        user_key: String,
        start_time: u64,
        end_time: u64,
        usage: u64,
        price_plan: String,
        cost: u64,
    }

    #[ink(storage)]
    pub struct ServicesStatistics {
        // Store a contract owner
        owner: AccountId,
        services: Option<ServicesMarket>,
        statistics_index: u64,
        // Store services statistics
        statistics_map: StorageHashMap<u64, UsageRecord>,
        services_map_by_uuid: StorageHashMap<String, Vec<u64>>,
        services_map_by_user_key: StorageHashMap<String, Vec<u64>>,
        services_map_by_provider: StorageHashMap<AccountId, Vec<u64>>,
    }

    #[ink(event)]
    pub struct SubmitUsageRecordEvent {
        id: u64,
        service_uuid: String,
        user_key: String,
        start_time: u64,
        end_time: u64,
    }

    impl ServicesStatistics {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(controller: AccountId, services_addr: AccountId) -> Self {
            let contract_instance: ServicesMarket = ink_env::call::FromAccountId::from_account_id(services_addr);
            Self {
                owner: controller,
                services: Some(contract_instance),
                statistics_index: 0,
                statistics_map: Default::default(),
                services_map_by_uuid: Default::default(),
                services_map_by_user_key: Default::default(),
                services_map_by_provider: Default::default(),
            }
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn submit_usage(&mut self, service_uuid: String, user_key: String,
                            start_time: u64, end_time: u64, usage: u64, price_plan: String, cost: u64) -> bool {

            assert_eq!(self.services.is_some(), true);
            let service = self.services.as_ref().unwrap().query_service_by_uuid(service_uuid.clone());
            let provider_addr = service.provider_owner;

            // fixme: temporally disable the check for hackathon demo
            // let caller = self.env().caller();
            // assert_eq!(service.provider_owner == caller, true);
            
            assert_eq!(self.statistics_index + 1 > self.statistics_index, true);

            self.statistics_map.insert(self.statistics_index, UsageRecord {
                id: self.statistics_index,
                service_uuid: service_uuid.clone(),
                user_key: user_key.clone(),
                start_time,
                end_time,
                price_plan,
                usage,
                cost,
            });

            let uuid_ids = self.services_map_by_uuid.entry(service_uuid.clone()).or_insert(Vec::new());
            uuid_ids.push(self.statistics_index);
            let user_key_ids = self.services_map_by_user_key.entry(user_key.clone()).or_insert(Vec::new());
            user_key_ids.push(self.statistics_index);
            let provider_ids = self.services_map_by_provider.entry(provider_addr.clone()).or_insert(Vec::new());
            provider_ids.push(self.statistics_index);

            self.env().emit_event(SubmitUsageRecordEvent {
                id: self.statistics_index,
                service_uuid,
                user_key,
                start_time,
                end_time,
            });
            self.statistics_index += 1;
            true
        }

        #[ink(message)]
        pub fn list_all_statistics(&self) -> Vec<UsageRecord> {
            let mut record_vec = Vec::new();
            let mut iter = self.statistics_map.values();
            let mut item = iter.next();
            while item.is_some() {
                record_vec.push(item.unwrap().clone());
                item = iter.next();
            }
            record_vec
        }

        /// query service usage statistics by index
        #[ink(message)]
        pub fn query_by_index(&self, id: u64) -> UsageRecord {
            self.statistics_map.get(&id).unwrap().clone()
        }

        /// query service usage statistics by service uuid
        #[ink(message)]
        pub fn query_by_service_uuid(&self, uuid: String) -> Vec<UsageRecord> {
            let uuid_ids = self.services_map_by_uuid.get(&uuid).unwrap();
            let mut service_vec = Vec::new();
            let mut uuid_ids_iter = uuid_ids.into_iter();
            let mut item = uuid_ids_iter.next();
            while item.is_some() {
                service_vec.push(self.statistics_map.get(item.unwrap()).unwrap().clone());
                item = uuid_ids_iter.next();
            }
            service_vec
        }

        /// query service usage statistics by user key
        #[ink(message)]
        pub fn query_by_user_key(&self, user_key: String) -> Vec<UsageRecord> {
            let user_key_ids = self.services_map_by_user_key.get(&user_key).unwrap();
            let mut service_vec = Vec::new();
            let mut iter = user_key_ids.into_iter();
            let mut item = iter.next();
            while item.is_some() {
                service_vec.push(self.statistics_map.get(item.unwrap()).unwrap().clone());
                item = iter.next();
            }
            service_vec
        }

        /// query service usage statistics by provider
        #[ink(message)]
        pub fn query_by_provider(&self, provider: AccountId) -> Vec<UsageRecord> {
            let provider_ids = self.services_map_by_provider.get(&provider).unwrap();
            let mut service_vec = Vec::new();
            let mut iter = provider_ids.into_iter();
            let mut item = iter.next();
            while item.is_some() {
                service_vec.push(self.statistics_map.get(item.unwrap()).unwrap().clone());
                item = iter.next();
            }
            service_vec
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        fn initContract() -> ServicesStatistics {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            ServicesStatistics{
                owner: accounts.alice,
                services: None,
                statistics_index: 0,
                statistics_map: Default::default(),
                services_map_by_uuid: Default::default(),
                services_map_by_user_key: Default::default(),
                services_map_by_provider: Default::default(),
            }
        }

        #[ink::test]
        #[should_panic]
        fn submit_usage() {
            let mut stats = initContract();
            let result = stats.submit_usage(String::from("59186d9a-ed85-4395-86a4-d58a7ab35f49"), String::from("5a9174970861974b524d566a"),
                                       1616739098000, 1616739698000, 10000, String::from("1/per 10000"), 1);
            assert!(result);
        }

        #[ink::test]
        #[should_panic]
        fn query_by_index() {
            let mut stats = initContract();
            let uuid = "59186d9a-ed85-4395-86a4-d58a7ab35f49";
            let user_key = "5a9174970861974b524d566a";
            let start_time = 1616739098000;
            let end_time = 1616739698000;
            let usage = 10000;
            let price_plan = "1/per 10000";
            let cost = 1;
            stats.submit_usage(uuid.to_string(), user_key.to_string(), start_time, end_time, usage, price_plan.to_string(), cost);
            let record = stats.query_by_index(0);
            assert_eq!(uuid, record.service_uuid);
            assert_eq!(user_key, record.user_key);
            assert_eq!(start_time, record.start_time);
            assert_eq!(end_time, record.end_time);
            assert_eq!(usage, record.usage);
            assert_eq!(price_plan, record.price_plan);
            assert_eq!(cost, record.cost);
        }

        #[ink::test]
        #[should_panic]
        fn query_by_provider() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut stats = initContract();
            let uuid = "59186d9a-ed85-4395-86a4-d58a7ab35f49";
            let user_key = "5a9174970861974b524d566a";
            let start_time = 1616739098000;
            let end_time = 1616739698000;
            let usage = 10000;
            let price_plan = "1/per 10000";
            let cost = 1;
            stats.submit_usage(uuid.to_string(), user_key.to_string(), start_time, end_time, usage, price_plan.to_string(), cost);
            let records = stats.query_by_provider(accounts.alice);
            assert_eq!(1, records.len());
            let record = &records[0];
            assert_eq!(uuid, record.service_uuid);
            assert_eq!(user_key, record.user_key);
            assert_eq!(start_time, record.start_time);
            assert_eq!(end_time, record.end_time);
            assert_eq!(usage, record.usage);
            assert_eq!(price_plan, record.price_plan);
            assert_eq!(cost, record.cost);
        }

        #[ink::test]
        #[should_panic]
        fn query_by_service_uuid() {
            let mut stats = initContract();
            let uuid = "59186d9a-ed85-4395-86a4-d58a7ab35f49";
            let user_key = "5a9174970861974b524d566a";
            let start_time = 1616739098000;
            let end_time = 1616739698000;
            let usage = 10000;
            let price_plan = "1/per 10000";
            let cost = 1;
            stats.submit_usage(uuid.to_string(), user_key.to_string(), start_time, end_time, usage, price_plan.to_string(), cost);
            let records = stats.query_by_service_uuid(uuid.to_string());
            assert_eq!(1, records.len());
            let record = &records[0];
            assert_eq!(uuid, record.service_uuid);
            assert_eq!(user_key, record.user_key);
            assert_eq!(start_time, record.start_time);
            assert_eq!(end_time, record.end_time);
            assert_eq!(usage, record.usage);
            assert_eq!(price_plan, record.price_plan);
            assert_eq!(cost, record.cost);
        }

        #[ink::test]
        #[should_panic]
        fn query_by_user_key() {
            let mut stats = initContract();
            let uuid = "59186d9a-ed85-4395-86a4-d58a7ab35f49";
            let user_key = "5a9174970861974b524d566a";
            let start_time = 1616739098000;
            let end_time = 1616739698000;
            let usage = 10000;
            let price_plan = "1/per 10000";
            let cost = 1;
            stats.submit_usage(uuid.to_string(), user_key.to_string(), start_time, end_time, usage, price_plan.to_string(), cost);
            let records = stats.query_by_user_key(user_key.to_string());
            assert_eq!(1, records.len());
            let record = &records[0];
            assert_eq!(uuid, record.service_uuid);
            assert_eq!(user_key, record.user_key);
            assert_eq!(start_time, record.start_time);
            assert_eq!(end_time, record.end_time);
            assert_eq!(usage, record.usage);
            assert_eq!(price_plan, record.price_plan);
            assert_eq!(cost, record.cost);
        }
    }
}
