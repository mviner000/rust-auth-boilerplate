use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use crate::schema::accounts;
use crate::domain::entities::account::{Account, UpdateAccountDto};
use async_trait::async_trait;
use crate::domain::repositories::account_repository::AccountRepository;

#[derive(Queryable, Selectable)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountRecord {
    pub id: i32,
    pub user_id: i32,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub default_avatar_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub user_id: i32,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<AccountRecord> for Account {
    fn from(record: AccountRecord) -> Self {
        Account {
            id: record.id,
            user_id: record.user_id,
            first_name: record.first_name,
            middle_name: record.middle_name,
            last_name: record.last_name,
            default_avatar_id: record.default_avatar_id,
            default_avatar: None, // You might want to load this separately if needed
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountChangeset {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<UpdateAccountDto> for AccountChangeset {
    fn from(dto: UpdateAccountDto) -> Self {
        Self {
            first_name: dto.first_name,
            middle_name: dto.middle_name,
            last_name: dto.last_name,
            updated_at: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Clone)]
pub struct AccountRepositoryImpl {
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>,
}

impl AccountRepositoryImpl {
    pub fn new(pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn find_by_user_id(&self, user_id: i32) -> Result<Account, Box<dyn std::error::Error>> {
        use crate::schema::accounts::dsl::*;

        let mut conn = self.pool.get()?;

        let record = accounts
            .filter(user_id.eq(user_id))
            .select(AccountRecord::as_select())
            .first(&mut conn)?;

        Ok(Account::from(record))
    }

    async fn update(&self, user_id: i32, dto: UpdateAccountDto) -> Result<Account, Box<dyn std::error::Error>> {
        use crate::schema::accounts::dsl::*;

        let mut conn = self.pool.get()?;

        let changeset = AccountChangeset::from(dto);

        let record = diesel::update(accounts.filter(user_id.eq(user_id)))
            .set(changeset)
            .returning(AccountRecord::as_select())
            .get_result(&mut conn)?;

        Ok(Account::from(record))
    }

    async fn set_default_avatar(&self, user_id: i32, avatar_id: i32) -> Result<Account, Box<dyn std::error::Error>> {
        use crate::schema::accounts::dsl::*;

        let mut conn = self.pool.get()?;

        let record = diesel::update(accounts)
            .filter(user_id.eq(user_id))
            .set((
                default_avatar_id.eq(Some(avatar_id)),
                updated_at.eq(chrono::Local::now().naive_utc()),
            ))
            .returning(AccountRecord::as_select())
            .get_result(&mut conn)?;

        Ok(Account::from(record))
    }
}