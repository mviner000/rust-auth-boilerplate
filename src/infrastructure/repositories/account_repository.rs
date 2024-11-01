use crate::schema::accounts;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use chrono::Utc;

use crate::domain::entities::account::{Account, UpdateAccountDto};
use crate::domain::repositories::account_repository::AccountRepository;

#[derive(Queryable)]
struct AccountRecord {
    id: i32,
    user_id: i32,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    avatar_300x300_url: Option<String>,
    avatar_40x40_url: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct AccountRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AccountRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn find_by_user_id(&self, user_id: i32) -> Result<Account, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        let record = accounts::table
            .filter(accounts::user_id.eq(user_id))
            .first::<AccountRecord>(conn)?;

        Ok(Account {
            id: record.id,
            user_id: record.user_id,
            first_name: record.first_name,
            middle_name: record.middle_name,
            last_name: record.last_name,
            avatar_300x300_url: record.avatar_300x300_url,
            avatar_40x40_url: record.avatar_40x40_url,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }


    async fn update(&self, user_id: i32, account_dto: UpdateAccountDto) -> Result<Account, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        let record = diesel::update(accounts::table)
            .filter(accounts::user_id.eq(user_id))
            .set((
                accounts::first_name.eq(account_dto.first_name),
                accounts::middle_name.eq(account_dto.middle_name),
                accounts::last_name.eq(account_dto.last_name),
                accounts::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<AccountRecord>(conn)?;

        Ok(Account {
            id: record.id,
            user_id: record.user_id,
            first_name: record.first_name,
            middle_name: record.middle_name,
            last_name: record.last_name,
            avatar_300x300_url: record.avatar_300x300_url,
            avatar_40x40_url: record.avatar_40x40_url,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }

    async fn update_avatar(&self, user_id: i32, new_avatar_300x300_url: String, new_avatar_40x40_url: String) -> Result<Account, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        let record = diesel::update(accounts::table)
            .filter(accounts::user_id.eq(user_id))
            .set((
                accounts::avatar_300x300_url.eq(new_avatar_300x300_url),
                accounts::avatar_40x40_url.eq(new_avatar_40x40_url),
                accounts::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<AccountRecord>(conn)?;

        Ok(Account {
            id: record.id,
            user_id: record.user_id,
            first_name: record.first_name,
            middle_name: record.middle_name,
            last_name: record.last_name,
            avatar_300x300_url: record.avatar_300x300_url,
            avatar_40x40_url: record.avatar_40x40_url,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}