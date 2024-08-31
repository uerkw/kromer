use kromer_economy_entity::{
    addresses, addresses::Entity as Address, names, names::Entity as Name, transactions,
    transactions::Entity as Transaction,
};

use sea_orm::*;

/// A helper struct that houses methods related to transactions
pub struct TransactionController;


impl TransactionController {
    /// Fetches the total number of transactions
    ///
    /// # Arguments
    /// * `conn` - The database connection
    ///
    /// # Examples
    /// ```
    /// let total = TransactionController::count(&db).await?;
    /// ```
    pub async fn count(conn: &DbConn) -> Result<u64, DbErr> {
        Transaction::find().count(conn).await
    }
}