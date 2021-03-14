# Apron service market Contract

service market is a contract to manager services.

## Modules

### Service
```rust
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
```

## Interfaces

### instance module
instance module.
```bash
type: tx
definition: pub fn new(controller: AccountId) -> Self;
```

### add service
add service to market.
```bash
type: tx
definition: pub fn add_service(&mut self, uuid: String, name: String, desc: String, logo: String, create_time: u64,
                           provider_name: String, provider_owner: AccountId, usage: String, price_plan: String, declaimer: String) -> bool;
```

### query service by id
query service by id.
```bash
type: query
definition: pub fn query_service_by_id(&self, id: u64) -> Service;
```

### query service by uuid
query service by uuid.
```bash
type: query
definition: pub fn query_service_by_uuid(&self, uuid: String) -> Service;
```


### query service by provider
query service by the provider address.
```bash
type: query
definition: pub fn list_services_provider(&self) -> Vec<Service>;
```

### list services
list services.
```bash
type: query
definition: pub fn list_services_provider(&self, provider: AccountId) -> Vec<Service>;
```
