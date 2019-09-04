use colored::*;

// standard library
use std::io::{stdin, stdout, Write};

pub fn get_arg() -> String {
    flush_read().trim().to_string()
}

/// Prints main menu options
pub fn main_menu() {
    println!();
    println!("1. Add user");
    println!("2. View all");
    println!("3. Remove user");
    println!("4. Search");
    println!("5. Save");
    println!("6. Quit");
    print!("{}", "> ".green());
}

/// Flushes std output and reads in the next line
fn flush_read() -> String {
    let mut s_input_text = String::new();
    stdout().flush().expect("failed to flush stdout");
    stdin()
        .read_line(&mut s_input_text)
        .expect("failed to read from stdin");

    s_input_text
}

/// Returns input `field` trimmed and parsed
pub fn get_trimmed_input(field: &str) -> String {
    print!("{}{} ", field, ">".green());
    flush_read().trim().parse().unwrap()
}

/// Returns input `field` as `i64`
pub fn parse_i64(field: &str) -> i64 {
    match get_trimmed_input(field).parse::<i64>() {
        Ok(n) => n,
        Err(err) => {
            println!("enter valid {}, {}", field, err);
            parse_i64(field)
        }
    }
}
