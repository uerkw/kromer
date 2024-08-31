use kromer_economy_entity::{names, names::Entity as Name};

use sea_orm::*;

/// A helper struct that houses methods related to names
pub struct NameController;

impl NameController {
    /// Fetches the total number of names owned by an address
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch
    ///
    /// # Examples
    /// ```
    /// let total = NameController::names_owned_by_address(db, "kromernya1").await?;
    /// ```
    pub async fn names_owned_by_address(conn: &DbConn, address: &str) -> Result<u64, DbErr> {
        Name::find()
            .filter(names::Column::Owner.eq(address))
            .count(conn)
            .await
    }
}
