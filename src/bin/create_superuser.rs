use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::io::{self, Write};

// Define the schema directly in this file
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

// Define a local User struct
#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

fn main() {
    dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Establish connection
    let mut conn = PgConnection::establish(&database_url)
        .expect("Error connecting to database");

    // Get user input
    let username = get_input("Enter username: ");
    let email = get_input("Enter email: ");
    let password = get_input("Enter password: ");

    // Hash the password
    let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
        .expect("Error hashing password");

    // Create the user
    let new_user = NewUser {
        username,
        email,
        password: hashed_password,
    };

    // Insert the user into the database
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error creating superuser");

    println!("Superuser created successfully!");
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}