extern crate colored;

use std::io::{stdout, Write};

use colored::*;
use crate::io::get_arg;

mod io;
mod search;
mod user;

fn main() {
    println!("{}", "User management system".blue().bold());

    let mut arg: i8 = 0;
    let mut users: Vec<user::User> = Vec::new();

    while arg != 6 {
        let _input_text = String::new();

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
                users.push(user::new_user());
                println!();
            }
            2 => {
                println!();
                println!("Users:");
                println!();

                for _u in users.iter() {
                    user::log_user(_u)
                }
            }
            3 => println!("3"),
            4 => {
                println!("Enter name:");
                stdout().flush().expect("failed to flush stdout");
                user::remove_user(&get_arg(), &mut users);
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
                let input = get_arg();

                match s_arg {
                    1 => search::by_name(&input, &users),
                    2 => search::by_username(&input, &users),
                    3 => search::by_email(&input, &users),
                    4 => search::by_age(io::parse_i64(&input), &users),
                    5 => search::by_id(io::parse_i64(&input), &users),
                    6 => continue,
                    _ => panic!("invalid option, quitting"),
                }
            }
            6 => println!("{}", "quitting...".yellow()),
            _ => panic!("invalid option, quitting"),
        }
    }
}
