pub(crate) struct CheckRequestInput<BS> {
    pub id: String,
    pub bucket_state: BS,
    pub refill_rate: u64,
    pub capacity: u64,
    pub consume: u64,
}
