//use limiter_core;

// use crate::memory::memory_store;
// use std::collections::HashMap;
// use std::sync::RwLock;

pub trait Memory {
    fn memory_status() {
        println!("you using in mememory storage");
    }

    //fn add_new_bucket(&self) -> Self;

    fn find_bucket(&self, id: String) -> Option<String>;

    //fn update_bucket(&self, id: String, v: String);

    //fn delate_bucket(id: String);
}

