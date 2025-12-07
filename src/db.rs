use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::Holder;

pub type Db = Arc<Mutex<HashMap<String, Holder>>>;

pub fn init_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}
