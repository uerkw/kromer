use crate::entities::transaction::{self, Entity as Transaction};

use sea_orm::*;

pub struct TransactionController;

impl TransactionController {
    pub async fn total(conn: &DbConn) -> Result<u64, DbErr> {
        Transaction::find().count(conn).await
    }

    pub async fn all(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<transaction::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Transaction::find()
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    pub async fn latest(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<transaction::Model>, DbErr> {
        Transaction::find()
            .order_by_asc(transaction::Column::Time)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    pub async fn get_by_id(
        conn: &DbConn,
        id: i32,
    ) -> Result<Option<transaction::Model>, DbErr> {
        Transaction::find_by_id(id).one(conn).await
    }

}