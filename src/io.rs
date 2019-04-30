use colored::*;
use std::io::{stdin, stdout, Write};

pub fn get_arg() -> String {
    flush_read().trim().to_string()
}

fn flush_read() -> String {
    let mut s_input_text = String::new();
    stdout().flush().expect("failed to flush stdout");
    stdin()
        .read_line(&mut s_input_text)
        .expect("failed to read from stdin");

    s_input_text
}

pub fn get_trimmed_input(field: &str) -> String {
    print!("{}{} ", field, ">".green());
    flush_read().trim().parse().unwrap()
}

pub fn parse_i64(field: &str) -> i64 {
    match get_trimmed_input(field).parse::<i64>() {
        Ok(n) => n,
        Err(err) => {
            println!("enter valid {}, {}", field, err);
            parse_i64(field)
        }
    }
}
