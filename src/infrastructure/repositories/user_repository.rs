use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::domain::{
    entities::user::{User, CreateUserDto},
    repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_name(&self, username: &str) -> Result<User, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        let user = users
            .filter(name.eq(username))
            .first::<(i32, String)>(conn)?;

        Ok(User {
            id: user.0,
            name: user.1,
        })
    }

    async fn create(&self, user_dto: CreateUserDto) -> Result<User, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        let new_user = diesel::insert_into(users)
            .values(name.eq(user_dto.name))
            .returning((id, name))
            .get_result::<(i32, String)>(conn)?;

        Ok(User {
            id: new_user.0,
            name: new_user.1,
        })
    }
}