use serde::{Serialize, de::DeserializeOwned};
pub trait AsyncQueryDatabase<V>
where
    V: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    fn create(
        &self,
        id: String,
        data: V,
        expires_at: Option<u64>,
    ) -> impl Future<Output = ()> + Send;
    fn find(&self, id: String) -> impl Future<Output = Option<V>> + Send
    where
        V: Clone + Send;

    fn update_bucket_status(
        &self,
        id: String,
        data: V,
        expires_at: Option<u64>,
    ) -> impl Future<Output = ()> + Send;
    fn delete_bucket(&self, id: String) -> impl Future<Output = ()> + Send;
}

pub trait QueryDatabase<K, V> {
    fn create(&self, id: K, data: V, expires_at: Option<u64>);
    fn find(&self, id: K) -> Option<V>
    where
        V: Clone;

    fn update_bucket_status(&self, id: K, data: V, expires_at: Option<u64>);
    fn delete_bucket(&self, id: K);
}
