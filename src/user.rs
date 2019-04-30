use crate::io;

// see Cargo.toml for acknowledgement
use prettytable::{Cell, Row, Table};

// see Cargo.toml for acknowledgement
use serde::{Deserialize, Serialize};
use colored::Colorize;

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

pub struct ActiveID(pub Vec<i64>);

/// Remove user from `users` by name
pub fn remove_user(name: &str, users: &mut Users, active_id: &mut ActiveID) {
    let index = users.0.iter().position(|u| u.name == name).unwrap();
    active_id.0.remove(index);
    users.0.remove(index);
}

/// Prompts user for data fields and creates a new user
pub fn new_user(active_id: &ActiveID) -> User {
    let name = io::get_trimmed_input("name");
    let username = io::get_trimmed_input("username");
    let email = io::get_trimmed_input("email");
    let age = io::parse_i64("age");
    let mut id = io::parse_i64("id");

    println!();
    while active_id.0.contains(&id) {
        println!("{} {}", id, "ID already active".red());
        id += 1;
    }

    println!("{} {}", "final id is".green(), id);

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
