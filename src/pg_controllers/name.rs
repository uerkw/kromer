use crate::entities::{address, name, name::Entity as Name};
use sea_orm::*;
use sqlx::types::chrono;

#[derive(Debug)]
pub struct NameRegistration {
    pub name: String,
    pub owner: address::Model,
}

pub struct NameController;

impl NameController {
    pub async fn name_count(conn: &DbConn) -> Result<u64, DbErr> {
        Name::find().count(conn).await
    }

    pub async fn names_owned_by_address(conn: &DbConn, address: &str) -> Result<u64, DbErr> {
        Name::find()
            .filter(name::Column::Owner.eq(address))
            .count(conn)
            .await
    }

    pub async fn list_names(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<name::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Name::find()
            .order_by_desc(name::Column::Registered)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    pub async fn get_name(conn: &DbConn, name: &str) -> Result<Option<name::Model>, DbErr> {
        Name::find()
            .filter(name::Column::Name.eq(name))
            .one(conn)
            .await
    }

    pub async fn is_name_available(conn: &DbConn, name: &str) -> Result<bool, DbErr> {
        let count = Name::find()
            .filter(name::Column::Name.eq(name))
            .count(conn)
            .await?;

        Ok(count == 0)
    }

    pub async fn get_newest_names(
        conn: &DbConn,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<name::Model>, DbErr> {
        let limit = limit.clamp(1, 1000);

        Name::find()
            .order_by_desc(name::Column::Registered)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await
    }

    pub async fn register_name(
        conn: &DbConn,
        registration: NameRegistration,
    ) -> Result<name::Model, DbErr> {
        let new_name = name::ActiveModel {
            name: Set(registration.name),
            owner: Set(registration.owner.address.clone()),
            original_owner: Set(Some(registration.owner.address)),
            registered: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        new_name.insert(conn).await
    }

}