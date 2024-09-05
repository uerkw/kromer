use kromer_economy_entity::transactions::{self, Entity as Transaction};

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
    pub async fn total(conn: &DbConn) -> Result<u64, DbErr> {
        Transaction::find().count(conn).await
    }

    /// Fetches all transactions
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of transactions to fetch
    /// * `offset` - The number of transactions to skip
    ///
    /// # Examples
    /// ```
    /// let transactions = TransactionController::all(&db).await?;
    /// ```
    pub async fn all(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<transactions::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Transaction::find()
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Fetches the latest transactions
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of transactions to fetch
    /// * `offset` - The number of transactions to skip
    ///
    /// # Examples
    /// ```
    /// let transactions = TransactionController::latest(&db).await?;
    /// ```
    pub async fn latest(conn: &DbConn, limit: u64, offset: u64) -> Result<Vec<transactions::Model>, DbErr> {
        Transaction::find()
            .order_by_asc(transactions::Column::Time)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Fetches a specific transaction by its ID
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `id` - The ID of the transaction
    ///
    /// # Examples
    /// ```
    /// let transaction = TransactionController::get_by_id(&db, 1).await?;
    /// ```
    pub async fn get_by_id(conn: &DbConn, id: i32) -> Result<Option<transactions::Model>, DbErr> {
        Transaction::find_by_id(id).one(conn).await
    }
}
