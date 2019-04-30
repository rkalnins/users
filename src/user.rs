use crate::io;

// see Cargo.toml for acknowledgement
use prettytable::{Cell, Row, Table};

// see Cargo.toml for acknowledgement
use colored::Colorize;
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

pub struct ActiveID(pub Vec<i64>);

/// Remove user from `users` by name
pub fn remove_user(name: &str, users: &mut Users, active_id: &mut ActiveID) {
    // get index of user
    let index = users.0.iter().position(|u| u.name == name).unwrap();

    // remove id and user
    active_id.0.remove(index);
    users.0.remove(index);
}

/// Prompts user for data fields and creates a new user
pub fn new_user(active_id: &ActiveID) -> User {
    // prompt for input
    let name = io::get_trimmed_input("name");
    let username = io::get_trimmed_input("username");
    let email = io::get_trimmed_input("email");
    let age = io::parse_i64("age");
    let mut id = io::parse_i64("id");

    println!();

    // check if id is already in use, if so increment by one until the requested id is available
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

    // create table
    let mut table = Table::new();

    // add header row
    table.add_row(row!["id", "name", "username", "email", "age"]);

    // add user data to table rows
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
