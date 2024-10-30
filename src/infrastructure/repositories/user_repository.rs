use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::domain::{
    entities::user::{User, CreateUserDto},
    repositories::user_repository::UserRepository,
};
use crate::domain::entities::user::UpdateUserDto;

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
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, user_id: i32) -> Result<User, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        let user = users
            .filter(id.eq(user_id))
            .first::<(i32, String, String, String)>(conn)?;

        Ok(User {
            id: user.0,
            username: user.1,
            email: user.2,
            password: user.3,
        })
    }

    async fn create(&self, user_dto: CreateUserDto) -> Result<User, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        let new_user = diesel::insert_into(users)
            .values((
                username.eq(&user_dto.username),
                email.eq(&user_dto.email),
                password.eq(&user_dto.password),
            ))
            .returning((id, username, email, password))
            .get_result::<(i32, String, String, String)>(conn)?;

        Ok(User {
            id: new_user.0,
            username: new_user.1,
            email: new_user.2,
            password: new_user.3,
        })
    }

    async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        let results = users
            .select((id, username, email, password))
            .load::<(i32, String, String, String)>(conn)?;

        Ok(results
            .into_iter()
            .map(|(user_id, user_username, user_email, user_password)| User {
                id: user_id,
                username: user_username,
                email: user_email,
                password: user_password,
            })
            .collect())
    }

    async fn update(&self, user_id: i32, user_dto: UpdateUserDto) -> Result<User, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        let updated_user = diesel::update(users)
            .filter(id.eq(user_id))
            .set((
                username.eq(&user_dto.username),
                email.eq(&user_dto.email),
                password.eq(&user_dto.password),
            ))
            .returning((id, username, email, password))
            .get_result::<(i32, String, String, String)>(conn)?;

        Ok(User {
            id: updated_user.0,
            username: updated_user.1,
            email: updated_user.2,
            password: updated_user.3,
        })
    }

    async fn delete(&self, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        let conn = &mut self.pool.get()?;
        diesel::delete(users)
            .filter(id.eq(user_id))
            .execute(conn)?;

        Ok(())
    }
}