use kromer_economy_entity::{addresses, names, names::Entity as Name};
use sea_orm::*;
use sqlx::types::chrono;

#[derive(Debug)]
pub struct NameRegistration {
    pub name: String,
    pub owner: addresses::Model,
}

/// A helper struct that houses methods related to names
pub struct NameController;

impl NameController {
    /// Fetches the total number of names
    ///
    /// # Arguments
    /// * `conn` - The database connection
    ///
    /// # Examples
    /// ```
    /// let total = NameController::count(&db).await?;
    /// ```
    pub async fn name_count(conn: &DbConn) -> Result<u64, DbErr> {
        Name::find().count(conn).await
    }

    /// Fetches the total number of names owned by an address
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch names for
    ///
    /// # Examples
    /// ```
    /// let total = NameController::names_owned_by_address(&db, "kromernya1").await?;
    /// ```
    pub async fn names_owned_by_address(conn: &DbConn, address: &str) -> Result<u64, DbErr> {
        Name::find()
            .filter(names::Column::Owner.eq(address))
            .count(conn)
            .await
    }

    /// Fetches a list of names
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of names to fetch
    /// * `offset` - The offset to start from
    ///
    /// # Examples
    /// ```
    /// let names = NameController::list_names(&db, 50, 0).await?;
    /// ```
    pub async fn list_names(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<names::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Name::find()
            .order_by_desc(names::Column::Registered)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Fetches a specific name
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `name` - The name to fetch
    ///
    /// # Examples
    /// ```
    /// let name = NameController::get_name(&db, "example").await?;
    /// ```
    pub async fn get_name(conn: &DbConn, name: &str) -> Result<Option<names::Model>, DbErr> {
        Name::find()
            .filter(names::Column::Name.eq(name))
            .one(conn)
            .await
    }

    /// Checks if a name is available
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `name` - The name to check
    ///
    /// # Examples
    /// ```
    /// let is_available = NameController::is_name_available(&db, "example").await?;
    /// ```
    pub async fn is_name_available(conn: &DbConn, name: &str) -> Result<bool, DbErr> {
        let count = Name::find()
            .filter(names::Column::Name.eq(name))
            .count(conn)
            .await?;
        Ok(count == 0)
    }

    /// Fetches the newest names
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of names to fetch
    /// * `offset` - The offset to start from
    ///
    /// # Examples
    /// ```
    /// let newest_names = NameController::get_newest_names(&db, 10, 0).await?;
    /// ```
    pub async fn get_newest_names(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<names::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Name::find()
            .order_by_desc(names::Column::Registered)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Creates a new name
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `registration` - The NameRegistration struct containing name and owner details
    ///
    /// # Examples
    /// ```
    /// let owner = AddressController::fetch_address(&db, "address").await?.unwrap();
    /// let registration = NameRegistration {
    ///     name: "example".to_string(),
    ///     owner,
    /// };
    /// let new_name = NameController::register_name(&db, registration).await?;
    /// ```
    pub async fn register_name(
        conn: &DbConn,
        registration: NameRegistration,
    ) -> Result<names::Model, DbErr> {
        let new_name = names::ActiveModel {
            name: Set(registration.name),
            owner: Set(registration.owner.address.clone()),
            original_owner: Set(Some(registration.owner.address)),
            registered: Set(chrono::Utc::now().fixed_offset()),
            ..Default::default()
        };

        new_name.insert(conn).await
    }
}
