use std::io::{stdin};
use structopt::StructOpt;


fn main() {
    let args = Cli::from_args();
}

#[derive(StructOpt)]
struct Cli {
    operation: String,
}

struct User {
    name: String,
    username: String,
    email: String,
    age: i8,
    id: i64
}


