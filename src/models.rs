use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holder {
    pub id: String,
    pub value: String,
}
