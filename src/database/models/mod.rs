pub mod name;
pub mod player;
pub mod transaction;
pub mod wallet;

use serde::{Deserialize, Serialize, Serializer};
use surrealdb::sql::Thing;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CountResponse {
    pub count: usize,
}

pub fn serialize_table<S>(x: &Thing, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let raw = x.to_raw();
    s.serialize_str(&raw)
}

pub fn serialize_table_opt<S>(x: &Option<Thing>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(thing) => s.serialize_str(&thing.to_raw()),
        None => s.serialize_none(),
    }
}
