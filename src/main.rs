use std::io::{stdin, stdout, Write};

fn main() {
    println!("User management system");
    let mut arg: i8 = 0;

    while arg != 5 {
        let mut input_text = String::new();

        println!();
        println!("1. Add user");
        println!("2. View User");
        println!("3. Find Users");
        println!("4. Remove User");
        println!("5. Quit");
        print!("> ");

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


    }
}

struct User {
    name: String,
    username: String,
    email: String,
    age: i8,
    id: i64,
}
