use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use async_trait::async_trait;
use chrono::Utc;
use crate::schema::avatars;
use crate::domain::entities::avatar::Avatar;
use crate::domain::repositories::avatar_repository::AvatarRepository;

#[derive(Queryable, Debug)]
pub(crate) struct AvatarRecord {
    pub id: i32,
    pub account_id: i32,
    pub avatar_300x300_url: Option<String>,
    pub avatar_40x40_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct AvatarRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AvatarRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AvatarRepository for AvatarRepositoryImpl {
    async fn create(&self, account_id: i32, avatar_300x300_url: String, avatar_40x40_url: String) -> Result<Avatar, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;

        let record = diesel::insert_into(avatars::table)
            .values((
                avatars::account_id.eq(account_id),
                avatars::avatar_300x300_url.eq(avatar_300x300_url),
                avatars::avatar_40x40_url.eq(avatar_40x40_url),
                avatars::created_at.eq(Utc::now().naive_utc()),
                avatars::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<AvatarRecord>(conn)?;

        Ok(Avatar {
            id: record.id,
            account_id: record.account_id,
            avatar_300x300_url: record.avatar_300x300_url,
            avatar_40x40_url: record.avatar_40x40_url,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }

    async fn find_by_account_id(&self, account_id: i32) -> Result<Vec<Avatar>, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;

        let records = avatars::table
            .filter(avatars::account_id.eq(account_id))
            .order_by(avatars::created_at.desc())
            .load::<AvatarRecord>(conn)?;

        Ok(records.into_iter().map(|record| Avatar {
            id: record.id,
            account_id: record.account_id,
            avatar_300x300_url: record.avatar_300x300_url,
            avatar_40x40_url: record.avatar_40x40_url,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }).collect())
    }

    async fn find_latest_by_account_id(&self, account_id: i32) -> Result<Option<Avatar>, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;

        let record = avatars::table
            .filter(avatars::account_id.eq(account_id))
            .order_by(avatars::created_at.desc())
            .first::<AvatarRecord>(conn)
            .optional()?;

        Ok(record.map(|record| Avatar {
            id: record.id,
            account_id: record.account_id,
            avatar_300x300_url: record.avatar_300x300_url,
            avatar_40x40_url: record.avatar_40x40_url,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }))
    }
}