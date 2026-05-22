pub struct Requester {
    pub identity: Identity,
    pub request_weight: u64,
}

pub enum Identity {
    UserId(String),
    ApiKey(String),
    IpAddress(String),
    ServiceInstance(String),
    Jwt(String),
}
