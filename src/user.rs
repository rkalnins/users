use crate::io;

// see Cargo.toml for acknowledgement
use prettytable::{Cell, Row, Table};

// see Cargo.toml for acknowledgement
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub email: String,
    pub age: i64,
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Users(pub Vec<User>);

/// Remove user from `users` by name
pub fn remove_user(name: &str, users: &mut Users) {
    let index = users.0.iter().position(|u| u.name == name).unwrap();
    users.0.remove(index);
}

/// Prompts user for data fields and creates a new user
pub fn new_user() -> User {
    let name = io::get_trimmed_input("name");
    let username = io::get_trimmed_input("username");
    let email = io::get_trimmed_input("email");
    let age = io::parse_i64("age");
    let id = io::parse_i64("id");

    User {
        name,
        username,
        email,
        age,
        id,
    }
}

/// Prints table of all users in `users` parameter
pub fn log_users(users: &[&User]) {
    let mut table = Table::new();
    table.add_row(row!["id", "name", "username", "email", "age"]);

    for user in users {
        table.add_row(Row::new(vec![
            Cell::new(&user.id.to_string()),
            Cell::new(&user.name),
            Cell::new(&user.username),
            Cell::new(&user.email),
            Cell::new(&user.age.to_string()),
        ]));
    }

    table.printstd();
}
