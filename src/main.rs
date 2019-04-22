use std::io::{stdin, stdout, Write};
extern crate colored;

use colored::*;

fn main() {
    println!("{}", "User management system".blue());
    let mut arg: i8 = 0;
    let mut users: Vec<User> = Vec::new();

    while arg != 5 {
        let mut input_text = String::new();

        println!();
        println!("1. Add user");
        println!("2. View User");
        println!("3. Find Users");
        println!("4. Remove User");
        println!("5. Quit");
        print!("{}", "> ".green());

        stdout().flush().expect("failed to flush stdout");
        stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        arg = match trimmed.parse::<i8>() {
            Ok(n) => n,
            Err(_) => {
                println!("enter valid input");
                continue;
            }
        };

        match arg {
            1 => {
                println!();
                println!("Add user");
                users.push(new_user());
                println!();
            }
            2 => {
                println!();
                println!("View users:");
                println!();

                for user in users.iter() {
                    println!("{} {}", "User: ".red(), user.id);
                    log_str_field("name", &user.name);
                    log_str_field("username", &user.username);
                    log_str_field("email", &user.email);
                    log_i64_field("age", user.age);
                    println!();
                }
            }
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("{}", "quitting...".yellow()),
            _ => panic!("not an option"),
        }
    }
}

fn log_str_field(field: &str, data: &str) {
    println!("{}: {:#}", field, data);
}

fn log_i64_field(field: &str, data: i64) {
    println!("{}: {:#}", field, data);
}

fn get_trimmed_input(field: &str) -> String {
    print!("{}{} ", field, ">".green());
    stdout().flush().expect("failed to flush stdout");
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).unwrap();
    let result = input_text.trim();
    result.parse().unwrap()
}

fn safe_i64_parse(field: &str) -> i64 {
    match get_trimmed_input(field).parse::<i64>() {
        Ok(n) => n,
        Err(err) => {
            println!("enter valid {}, {}", field, err);
            safe_i64_parse(field)
        }
    }
}

fn new_user() -> User {
    User {
        name: get_trimmed_input("name"),
        username: get_trimmed_input("username"),
        email: get_trimmed_input("email"),
        age: safe_i64_parse("age"),
        id: safe_i64_parse("id"),
    }
}

struct User {
    name: String,
    username: String,
    email: String,
    age: i64,
    id: i64,
}
