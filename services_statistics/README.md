# Apron service statistics Contract

service statistics is a contract to statistic service.

## Modules

### Service
```rust
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
```

## Interfaces

### instance module
instance module, input controller and service market.
```bash
type: tx
definition: pub fn new(controller: AccountId, services_addr: AccountId) -> Self;
```

### submit service usage
submit service usage, the signer must provider of service.
```bash
type: tx
definition: pub fn submit_usage(&mut self, service_uuid: String, user_key: String,
                            start_time: u64, end_time: u64, usage: u64, price_plan: String, cost: u64) -> bool;
```

### query service by id
query service by id.
```bash
type: query
definition: pub fn query_service_by_id(&self, id: u64) -> UsageRecord;
```

### query service by uuid
query service by uuid.
```bash
type: query
definition: pub fn query_service_by_uuid(&self, uuid: String) -> Vec<UsageRecord>;
```

### query service by user key
query service by user key.
```bash
type: query
definition: pub fn query_service_by_user_key(&self, user_key: String) -> Vec<UsageRecord>;
```


### query service by provider
query service by the provider.
```bash
type: query
definition: pub fn query_by_provider(&self, provider: AccountId) -> Vec<UsageRecord>;
```
