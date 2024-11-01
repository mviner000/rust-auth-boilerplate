use bcrypt::{hash_with_salt, DEFAULT_COST};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::io::{self, Write};
use rpassword::read_password;

// Define the schema
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(user_roles -> users (user_id));
joinable!(user_roles -> roles (role_id));
allow_tables_to_appear_in_same_query!(users, roles, user_roles);

// Define structs
#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Insertable)]
#[diesel(table_name = user_roles)]
struct NewUserRole {
    user_id: i32,
    role_id: i32,
}

fn main() {
    dotenv().ok();

    // Get environment variables
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let secret_key = std::env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set");

    // Establish connection
    let mut conn = PgConnection::establish(&database_url)
        .expect("Error connecting to database");

    // Get user input
    let username = get_input("Enter username: ");
    let email = get_input("Enter email: ");

    // Get password securely
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_password().expect("Error reading password");

    print!("Confirm password: ");
    io::stdout().flush().unwrap();
    let password_confirm = read_password().expect("Error reading password confirmation");

    if password != password_confirm {
        eprintln!("Passwords do not match!");
        std::process::exit(1);
    }

    // Create a salt from the secret key
    let salt = secret_key.as_bytes()[..16].try_into()
        .expect("SECRET_KEY must be at least 16 bytes");

    // Hash the password with the salt
    let hashed_password = hash_with_salt(password.as_bytes(), DEFAULT_COST, salt)
        .expect("Error hashing password")
        .to_string();

    // Start a transaction
    conn.transaction(|conn| {
        // Create the user
        let new_user = NewUser {
            username,
            email,
            password: hashed_password,
        };

        // Insert the user and get their ID
        let user_id = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(users::id)
            .get_result::<i32>(conn)?;

        // Get the superuser role ID
        let superuser_role_id = roles::table
            .filter(roles::name.eq("superuser"))
            .select(roles::id)
            .first::<i32>(conn)?;

        // Create the user-role association
        let new_user_role = NewUserRole {
            user_id,
            role_id: superuser_role_id,
        };

        // Insert the user-role association
        diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .execute(conn)?;

        Result::<_, diesel::result::Error>::Ok(())
    })
        .expect("Error creating superuser with role");

    println!("\nSuperuser created successfully with superuser role!");
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}