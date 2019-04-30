use crate::io;

use prettytable::{Table, Row, Cell};

pub struct User {
    pub name: String,
    pub username: String,
    pub email: String,
    pub age: i64,
    pub id: i64,
}

pub fn remove_user(name: &str, users: &mut Vec<User>) {
    let index = users.iter().position(|u| u.name == name).unwrap();
    users.remove(index);
}

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

pub fn log_users(users: &Vec<&User>) {
    let mut table = Table::new();
    table.add_row(row!["id", "name", "username", "email", "age"]);

    for user in users {
        table.add_row(Row::new(vec![
            Cell::new(&user.id.to_string()),
            Cell::new(&user.name),
            Cell::new(&user.username),
            Cell::new(&user.email),
            Cell::new(&user.age.to_string())]));
    }

    table.printstd();
}