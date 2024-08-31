use ::kromer_economy_entity::{
    addresses, addresses::Entity as Address, names, names::Entity as Name, transactions,
    transactions::Entity as Transaction,
};
use sea_orm::*;

pub struct Query;

impl Query {
    /// Fetches a single address from the database by its unique id
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `id` - The id of the address to fetch
    ///
    /// # Examples
    /// ```
    /// println!("TODO");
    /// ```
    pub async fn find_address_by_id(
        conn: &DbConn,
        id: i32,
    ) -> Result<Option<addresses::Model>, DbErr> {
        Address::find_by_id(id).one(conn).await
    }

    /// Fetches addresses from the database
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of addresses to fetch
    /// * `offset` - The offset to start from
    pub async fn fetch_addresses(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<addresses::Model>, DbErr> {
        // Clamp limit to be minimum 1 and maximum 1000
        let limit = limit.clamp(1, 1000); // NOTE: Even though this can panic, min is not above max so we are fine.

        // TODO: Add support for fetching name count.
        let addresses = Address::find()
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await?;

        Ok(addresses)
    }

    /// Fetches a single address from the database
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch
    ///
    /// # Examples
    /// ```
    /// let address = Query::find_address(db, "kromernya").await?;
    /// ```
    pub async fn find_address(
        conn: &DbConn,
        address: &str,
        _should_fetch_names: bool,
    ) -> Result<Option<addresses::Model>, DbErr> {
        // TODO: Add support for fetching name count.
        Address::find()
            .filter(addresses::Column::Address.eq(address))
            .one(conn)
            .await
    }

    /// Fetches the richest addresses from the database
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The number of addresses to fetch
    /// * `offset` - The offset for pagination
    ///
    /// # Examples
    /// ```
    /// let limit = 10;
    /// let offset = 0;
    /// let richest_addresses = Query::find_richest_addresses(db, limit, offset).await?;
    ///
    /// for (index, address) in richest_addresses.iter().enumerate() {
    ///     println!("{}. Address: {}, Balance: {}", index, address.address, address.balance);
    /// }
    /// ```
    pub async fn find_richest_addresses(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<addresses::Model>, DbErr> {
        Address::find()
            .order_by_desc(addresses::Column::Balance)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Counts the total number of addresses in the database
    ///
    /// # Arguments
    /// * `conn` - The database connection
    ///
    /// # Returns
    /// The total number of addresses as a `u64`
    ///
    /// # Examples
    /// ```
    /// let total = Query::count_total_addresses(&db).await?;
    /// println!("Total addresses: {}", total);
    /// ```
    pub async fn count_total_addresses(conn: &DbConn) -> Result<u64, DbErr> {
        Address::find().count(conn).await
    }

    /// Fetches all transactions for an address and the total number of transactions
    ///
    /// This checks both the `from` and `to` columns
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch transactions for
    ///
    /// # Examples
    /// ```
    /// let total = Query::count_total_transactions_from_address(&db, "kromernya").await?;
    /// println!("Total transactions: {}", total);
    /// ```
    pub async fn count_total_transactions_from_address(
        conn: &DbConn,
        address: &str,
    ) -> Result<u64, DbErr> {
        Transaction::find()
            .filter(
                transactions::Column::From
                    .eq(address)
                    .or(transactions::Column::To.eq(address)),
            )
            .count(conn)
            .await
    }

    /// Fetches all transactions for an address
    ///
    /// This checks both the `from` and `to` columns
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch transactions for
    ///
    /// # Examples
    /// ```
    /// let transactions = Query::find_transactions_from_address(&db, "kromernya").await?;
    /// ```
    pub async fn find_transactions_from_address(
        conn: &DbConn,
        address: &str,
    ) -> Result<Vec<transactions::Model>, DbErr> {
        Transaction::find()
            .filter(
                transactions::Column::From
                    .eq(address)
                    .or(transactions::Column::To.eq(address)),
            )
            .all(conn)
            .await
    }

    pub async fn count_names_owned_by_address(conn: &DbConn, address: &str) -> Result<u64, DbErr> {
        Name::find()
            .filter(names::Column::Owner.eq(address))
            .count(conn)
            .await
    }

    pub async fn find_names_owned_by_address(
        conn: &DbConn,
        address: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<names::Model>, DbErr> {
        Name::find()
            .filter(names::Column::Owner.eq(address))
            .order_by_asc(names::Column::Name)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }
}
