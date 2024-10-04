use surrealdb::{
    engine::any::Any,
    sql::{Datetime, Id, Thing},
    Surreal,
};

use super::serialize_table_opt;
use crate::routes::PaginationParams;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Model {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_table_opt"
    )]
    pub id: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transfered: Option<Datetime>,
    pub name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_table_opt"
    )]
    pub original_owner: Option<Thing>,
    pub owner: Thing,
    pub registered: Datetime,
}

impl Model {
    /// Get a name from its unique ID
    pub async fn get(db: &Surreal<Any>, id: String) -> Result<Option<Model>, surrealdb::Error> {
        let thing: Thing = id.try_into().unwrap();
        let q = "SELECT * FROM name WHERE id = $id;";

        let mut response = db.query(q).bind(("id", thing)).await?;
        let model: Option<Model> = response.take(0)?;

        Ok(model)
    }

    /// Get a name from its unique ID, not including the table part
    pub async fn get_partial(
        db: &Surreal<Any>,
        id: String,
    ) -> Result<Option<Model>, surrealdb::Error> {
        let id = Id::from(id);
        let thing = Thing::from(("name", id));

        let q = "SELECT * FROM name WHERE id = $id;";

        let mut response = db.query(q).bind(("id", thing)).await?;
        let model: Option<Model> = response.take(0)?;

        Ok(model)
    }

    /// Get name from its name field
    pub async fn get_by_name(
        db: &Surreal<Any>,
        name: String,
    ) -> Result<Option<Model>, surrealdb::Error> {
        let q = "SELECT * from name where name = $name;";

        let mut response = db.query(q).bind(("name", name)).await?;
        let model: Option<Model> = response.take(0)?;

        Ok(model)
    }

    /// Get name from its name field, omitting id.
    pub async fn get_by_name_excl(
        db: &Surreal<Any>,
        name: String,
    ) -> Result<Option<Model>, surrealdb::Error> {
        let q = "SELECT * OMIT id from name where name = $name;";

        let mut response = db.query(q).bind(("name", name)).await?;
        let model: Option<Model> = response.take(0)?;

        Ok(model)
    }

    /// Get all names, omitting id.
    pub async fn all(
        db: &Surreal<Any>,
        pagination: &PaginationParams,
    ) -> Result<Vec<Model>, surrealdb::Error> {
        let limit = pagination.limit.unwrap_or(50);
        let offset = pagination.offset.unwrap_or(0);
        let limit = limit.clamp(1, 1000);

        let q = "SELECT * OMIT id from name LIMIT $limit START $offset";

        let mut response = db
            .query(q)
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?;
        let models: Vec<Model> = response.take(0)?;

        Ok(models)
    }
}
