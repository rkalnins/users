use crate::io;
use colored::*;


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

pub fn log_user(user: &User) {
    println!("{} {}", "User: ".red(), user.id);
    io::log_str_field("name", &user.name);
    io::log_str_field("username", &user.username);
    io::log_str_field("email", &user.email);
    io::log_i64_field("age", user.age);
    println!();
}