

use argon2::{self, PasswordHash};
use chrono::Utc;
use sea_orm::*;

use crate::{entities::{address::{self, Entity as Address, Model}, name::{self, Entity as Name}, transaction::{self, Entity as Transaction}}, pg_utils::crypto::{make_v2_address, sha256}};

pub struct AddressController;

impl AddressController {
    pub async fn fetch_address(
        conn: &DbConn,
        address: &str,
        _should_fetch_names: bool,
    ) -> Result<Option<Model>, DbErr> {
        Address::find()
            .filter(address::Column::Address.eq(address))
            .one(conn)
            .await
    }

    pub async fn addresses(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Model>, DbErr> {
        let limit = limit.clamp(1,1000);

        Address::find().limit(limit).offset(offset).all(conn).await
    }

    pub async fn count(conn: &DbConn) -> Result<u64, DbErr> {
        Address::find().count(conn).await
    }

    pub async fn richest(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Address::find()
            .order_by_desc(address::Column::Balance)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    pub async fn transactions(
        conn: &DbConn,
        address: &str,
    ) -> Result<Vec<transaction::Model>, DbErr> {
        Transaction::find()
            .filter(
                transaction::Column::From
                    .eq(address)
                    .or(transaction::Column::To.eq(address)),
            )
            .all(conn)
            .await
    }

    pub async fn names(
        conn: &DbConn,
        address: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<name::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Name::find()
            .filter(name::Column::Owner.eq(address))
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    pub async fn get_from_private_key_hash<'a>(
        conn: &DbConn,
        hash: &PasswordHash<'a>,
    ) -> Result<Option<Model>, DbErr> {

        Address::find()
            .filter(address::Column::PrivateKey.eq(hash.serialize().to_string()))
            .one(conn)
            .await
    }

    pub async fn verify_address(
        conn: &DbConn,
        private_key: String,
    ) -> Result<Option<Model>, DbErr> {
        let krist_address: String = make_v2_address(&private_key, "k");

        let string_to_hash = [krist_address, private_key].join("");
        let hash: String = sha256(&string_to_hash);

        Address::find()
            .filter(address::Column::PrivateKey.eq(hash))
            .one(conn)
            .await
    }

    pub async fn create_wallet(
        conn: &DbConn,
        private_key: String,
    ) -> Result<Model, DbErr> {
        let krist_address: String = make_v2_address(&private_key, "k");

        let string_to_hash = [krist_address.clone(), private_key].join("");
        let hash: String = sha256(&string_to_hash);

        let wallet = address::ActiveModel {
           address: Set(krist_address),
           balance: Set(500.0),
           total_in: Set(0.0),
           total_out: Set(0.0),
           first_seen: Set(Utc::now().naive_utc()),
           private_key: Set(Some(hash)),
           alert: Set(None),
           locked: Set(false),
           ..Default::default()
        };

        wallet.insert(conn).await

    }

}