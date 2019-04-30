extern crate colored;

use std::io::{stdin, stdout, Write};

use colored::*;

mod io;

fn main() {
    println!("{}", "User management system".blue().bold());

    let mut arg: i8 = 0;
    let mut users: Vec<User> = Vec::new();

    while arg != 6 {
        let mut input_text = String::new();

        println!();
        println!("1. Add user");
        println!("2. View User");
        println!("3. Find Users");
        println!("4. Remove User");
        println!("5. Search");
        println!("6. Quit");
        print!("{}", "> ".green());

        arg = match io::get_arg().parse::<i8>() {
            Ok(n) => n,
            Err(_) => {
                println!("{}", "enter valid input".red());
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
                println!("Users:");
                println!();

                for user in users.iter() {
                    println!("{} {}", "User: ".red(), user.id);
                    io::log_str_field("name", &user.name);
                    io::log_str_field("username", &user.username);
                    io::log_str_field("email", &user.email);
                    io::log_i64_field("age", user.age);
                    println!();
                }
            }
            3 => println!("3"),
            4 => {
                println!("Enter name:");
                stdout().flush().expect("failed to flush stdout");
                let mut input_text = String::new();
                stdin().read_line(&mut input_text).unwrap();
                let result = input_text.trim();
                remove_user(&result, &mut users);
            }
            5 => {
                println!("Search by");
                println!();
                println!("1. Name");
                println!("2. Username");
                println!("3. Email");
                println!("4. Age");
                println!("5. ID");
                println!("6. Back");
                print!("{}", "> ".green());

                let s_arg = match io::get_arg().parse::<i8>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("{}", "enter valid search option".red());
                        continue;
                    }
                };

                print!("{}", "> ".green());

                match s_arg {
                    1 => { println!("1") }
                    2 => { println!("2") }
                    3 => { println!("3") }
                    4 => { println!("4") }
                    5 => { println!("5") }
                    6 => continue,
                    _ => panic!("invalid option, quitting")
                }
            }
            6 => println!("{}", "quitting...".yellow()),
            _ => panic!("invalid option, quitting")
        }
    }
}


fn remove_user(name: &str, users: &mut Vec<User>) {
    let index = users.iter().position(|u| &u.name == name).unwrap();
    users.remove(index);
}

fn new_user() -> User {
    let name = io::get_trimmed_input("name");
    let username = io::get_trimmed_input("username");
    let email = io::get_trimmed_input("email");
    let age = io::safe_i64_parse("age");
    let id = io::safe_i64_parse("id");

    let user = User {
        name,
        username,
        email,
        age,
        id,
    };

    user
}

struct User {
    name: String,
    username: String,
    email: String,
    age: i64,
    id: i64,
}
