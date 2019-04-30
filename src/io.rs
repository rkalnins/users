use colored::*;
use std::io::{stdin, stdout, Write};

pub fn get_arg() -> String {
    let mut s_input_text = String::new();

    stdout().flush().expect("failed to flush stdout");
    stdin()
        .read_line(&mut s_input_text)
        .expect("failed to read from stdin");

    s_input_text.trim().to_string()
}

pub fn log_str_field(data: &str) {
    print!("{:#}", data);
}

pub fn log_i64_field(data: i64) {
    print!("{:#}", data);
}

pub fn get_trimmed_input(field: &str) -> String {
    print!("{}{} ", field, ">".green());
    stdout().flush().expect("failed to flush stdout");
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).unwrap();
    let result = input_text.trim();
    result.parse().unwrap()
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
