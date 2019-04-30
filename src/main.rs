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
    let mut active_id: ActiveID = ActiveID(Vec::new());

    // deserialize data from save.json file
    let data = fs::read_to_string("save.json").expect("Unable to read file");
    let mut users: Users = serde_json::from_str(&data.trim()).unwrap();

    // add current user ids to active_id
    users.0.iter().for_each(|u| active_id.0.push(u.id));

    while arg != QUIT {
        let _input_text = String::new();

        // display main menu
        main_menu();

        // parse input as an 8-bit signed integer
        arg = match io::get_arg().parse::<i8>() {
            Ok(n) => n,
            Err(_) => {
                println!("{}", "enter valid input".red());
                continue;
            }
        };

        // match option
        match arg {
            // add user
            ADD => {
                println!();
                println!("{}", "Add user".bright_blue().bold());

                // get new user
                let user = user::new_user(&active_id);

                // add id and user
                active_id.0.push(user.id);
                users.0.push(user);
                println!();
            }
            // view all users
            VIEW => {
                println!();
                println!("{}", "Users".bright_blue().bold());
                search::all(&users);
            }
            // remove user
            REMOVE => {
                println!("Enter name:");
                stdout().flush().expect("failed to flush stdout");
                user::remove_user(&get_arg(), &mut users, &mut active_id);
            }
            // search for user
            SEARCH => {
                // displays search options
                search::search_options();

                // parse input as an 8-bit signed integer
                let s_arg = match io::get_arg().parse::<i8>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("{}", "enter valid search option".red());
                        continue;
                    }
                };

                print!("{}", "> ".green());

                // get user input
                let input = get_arg();

                // search users according to search field and user input
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
            // save data
            SAVE => {
                save(&users);
                println!("{}", "Saved data successfully".bright_green())
            }
            // quit app
            QUIT => {
                save(&users);
                println!("{}", "quitting...".yellow())
            }
            _ => panic!("invalid option, quitting"),
        }
    }
}

/// Saves data in `users` to save.json
fn save(users: &Users) {
    // serialize user data to a string
    let data = serde_json::to_string(&users).unwrap();

    // write string to save.json
    fs::write("save.json", data).expect("Unable to write file");
}
