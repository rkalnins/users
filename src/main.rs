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
use crate::user::{ActiveID, Users};

mod io;
mod search;
mod user;

const ADD: i8 = 1;
const VIEW: i8 = 2;
const REMOVE: i8 = 3;
const SEARCH: i8 = 4;
const SAVE: i8 = 5;
const QUIT: i8 = 6;

fn main() {
    println!("{}", "User management system".blue().bold());

    let mut arg: i8 = 0;
    let data = fs::read_to_string("save.json").expect("Unable to read file");
    let mut users: Users = serde_json::from_str(&data.trim()).unwrap();
    let mut active_id: ActiveID = ActiveID(Vec::new());

    users.0.iter().for_each(|u| active_id.0.push(u.id));

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
                println!("{}", "Add user".bright_blue().bold());
                let user = user::new_user(&active_id);
                active_id.0.push(user.id);
                users.0.push(user);
                println!();
            }
            VIEW => {
                println!();
                println!("{}", "Users".bright_blue().bold());
                search::all(&users);
            }
            REMOVE => {
                println!("Enter name:");
                stdout().flush().expect("failed to flush stdout");
                user::remove_user(&get_arg(), &mut users, &mut active_id);
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
            SAVE => {
                save(&users);
                println!("{}", "Saved data successfully".bright_green())
            }
            QUIT => {
                save(&users);
                println!("{}", "quitting...".yellow())
            }
            _ => panic!("invalid option, quitting"),
        }
    }
}

fn save(users: &Users) {
    let data = serde_json::to_string(&users).unwrap();
    fs::write("save.json", data).expect("Unable to write file");
}
