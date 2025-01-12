use surrealdb::{
    engine::any::Any,
    sql::{Datetime, Id, Thing},
    Surreal,
};

use super::{serialize_table_opt, CountResponse};
use crate::{models::transactions::TransactionType, routes::PaginationParams};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Model {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_table_opt"
    )]
    pub id: Option<Thing>,
    pub amount: f64,
    pub from: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    pub timestamp: Datetime,
    pub to: Thing,
    pub transaction_type: TransactionType,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransactionCreateData {
    pub from: Thing,
    pub to: Thing,
    pub amount: f64,
    pub metadata: Option<String>,
}

impl Model {
    /// Get a transaction from its unique ID
    pub async fn get(db: &Surreal<Any>, id: String) -> Result<Option<Model>, surrealdb::Error> {
        let thing: Thing = id.try_into().unwrap();
        let q = "SELECT * FROM transaction WHERE id = $id;";

        let mut response = db.query(q).bind(("id", thing)).await?;
        let model: Option<Model> = response.take(0)?;

        Ok(model)
    }

    /// Get a transaction from its unique ID, not including the table part
    pub async fn get_partial(
        db: &Surreal<Any>,
        id: String,
    ) -> Result<Option<Model>, surrealdb::Error> {
        let id = Id::from(id);
        let thing = Thing::from(("transaction", id));

        let q = "SELECT * FROM transaction WHERE id = $id;";

        let mut response = db.query(q).bind(("id", thing)).await?;
        let model: Option<Model> = response.take(0)?;

        Ok(model)
    }

    /// Get all transactions, omitting id.
    pub async fn all(
        db: &Surreal<Any>,
        pagination: &PaginationParams,
    ) -> Result<Vec<Model>, surrealdb::Error> {
        let limit = pagination.limit.unwrap_or(50);
        let offset = pagination.offset.unwrap_or(0);
        let limit = limit.clamp(1, 1000);

        let q = "SELECT * OMIT id from transaction LIMIT $limit START $offset";

        let mut response = db
            .query(q)
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?;
        let models: Vec<Model> = response.take(0)?;

        Ok(models)
    }

    /// Get the total amount of transactions in the database
    pub async fn count(db: &Surreal<Any>) -> Result<usize, surrealdb::Error> {
        let q = "(SELECT count() FROM transaction GROUP BY count)[0] or { count: 0 }";

        let mut response = db.query(q).await?;
        let count: Option<CountResponse> = response.take(0)?;
        let count = count.unwrap_or_default(); // Its fine, we make sure we always get a response with the `or` statement in the query.

        Ok(count.count)
    }
}
