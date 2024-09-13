use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use kromer_economy_entity::{
    addresses, addresses::Entity as Address, names, names::Entity as Name, transactions,
    transactions::Entity as Transaction,
};

use sea_orm::*;

/// A helper struct that houses methods related to addresses
pub struct AddressController;

// NOTE(sov): I am not sure about the function names, we can change them later.
impl AddressController {
    /// Fetches a single address from the database
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch
    /// * `should_fetch_names` - Whether to fetch the name of the address
    ///
    /// # Examples
    /// ```
    /// let address = AddressController:find_address(db, "kromernya1", true).await?;
    /// ```
    pub async fn fetch_address(
        conn: &DbConn,
        address: &str,
        _should_fetch_names: bool,
    ) -> Result<Option<addresses::Model>, DbErr> {
        Address::find()
            .filter(addresses::Column::Address.eq(address))
            .one(conn)
            .await
    }

    /// Fetches the addresses from the database
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of addresses to fetch
    /// * `offset` - The offset to start from
    ///
    /// # Examples
    /// ```
    /// let limit = 10;
    /// let offset = 0;
    /// let addresses = AddressController:addresses(db, limit, offset).await?;
    /// ```
    pub async fn addresses(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<addresses::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Address::find().limit(limit).offset(offset).all(conn).await
    }

    /// Fetches the total number of addresses
    ///
    /// # Arguments
    /// * `conn` - The database connection
    ///
    /// # Examples
    /// ```
    /// let total = AddressController:count(db).await?;
    /// ```
    pub async fn count(conn: &DbConn) -> Result<u64, DbErr> {
        Address::find().count(conn).await
    }

    /// Fetches the addresses with the highest balance and filters them descendingly
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `limit` - The maximum number of addresses to fetch
    /// * `offset` - The offset to start from
    ///
    /// # Examples
    /// ```
    /// let limit = 10;
    /// let offset = 0;
    /// let addresses = AddressController:richest(db, limit, offset).await?;
    ///
    /// for (index, address) in addresses.iter().enumerate() {
    ///     println!("{}. Address: {}, Balance: {}", index, address.address, address.balance);
    /// }
    /// ````
    pub async fn richest(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<addresses::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Address::find()
            .order_by_desc(addresses::Column::Balance)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Fetches all transactions for an address
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch transactions for
    ///
    /// # Examples
    /// ```
    /// let transactions = AddressController:find_transactions_from_address(&db, "kromernya1").await?;
    /// ```
    pub async fn transactions(
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

    /// Fetches all names owned by an address
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `address` - The address to fetch names for
    /// * `limit` - The maximum number of names to fetch
    /// * `offset` - The offset to start from
    ///
    /// # Examples
    /// ```
    /// let names = AddressController:names(&db, "kromernya1").await?;
    /// ```
    pub async fn names(
        conn: &DbConn,
        address: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<names::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Name::find()
            .filter(names::Column::Owner.eq(address))
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    /// Fetches the address from the database when given a private key
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `private_key` - The private key to fetch the address for
    ///
    /// # Examples
    /// ```
    /// let db_hash = PasswordHash::new("keykeykey1")
    ///     .map_err(|_| DbErr::Custom("Failed to parse stored password hash".to_string()))?;
    /// let address = AddressController::get_from_private_key_hash(&db, &).await?;
    /// ```
    pub async fn get_from_private_key_hash<'a>(
        conn: &DbConn,
        hash: &PasswordHash<'a>,
    ) -> Result<Option<addresses::Model>, DbErr> {        
        Address::find()
            .filter(addresses::Column::PrivateKey.eq(hash.serialize().to_string())) // TODO: Check using argon2 crate
            .one(conn)
            .await
    }
}
