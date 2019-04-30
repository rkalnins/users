// see Cargo.toml for acknowledgement
extern crate colored;

use colored::*;

// see Cargo.toml for acknowledgement
#[macro_use]
extern crate prettytable;

// see Cargo.toml for acknowledgement
extern crate serde;

// standard library
use std::fs;
use std::io::{stdout, Write};

use crate::io::{get_arg, main_menu};

mod io;
mod search;
mod user;

const ADD: i8 = 1;
const VIEW: i8 = 2;
const REMOVE: i8 = 3;
const SEARCH: i8 = 4;
const QUIT: i8 = 5;

fn main() {
    println!("{}", "User management system".blue().bold());

    let mut arg: i8 = 0;
    let data = fs::read_to_string("save.json").expect("Unable to read file");
    let mut users: user::Users = serde_json::from_str(&data.trim()).unwrap();

    while arg != QUIT {
        let _input_text = String::new();

        main_menu();

        arg = match io::get_arg().parse::<i8>() {
            Ok(n) => n,
            Err(_) => {
                println!("{}", "enter valid input".red());
                continue;
            }
        };

        match arg {
            ADD => {
                println!();
                println!("Add user");
                users.0.push(user::new_user());
                println!();
            }
            VIEW => {
                println!();
                println!("Users:");
                search::all(&users);
            }
            REMOVE => {
                println!("Enter name:");
                stdout().flush().expect("failed to flush stdout");
                user::remove_user(&get_arg(), &mut users);
            }
            SEARCH => {
                search::search_options();

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
            QUIT => {
                let data = serde_json::to_string(&users).unwrap();
                fs::write("save.json", data).expect("Unable to write file");
                println!("{}", "quitting...".yellow())
            }
            _ => panic!("invalid option, quitting"),
        }
    }
}
