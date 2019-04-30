// see Cargo.toml for acknowledgement
use crate::colored::*;

use crate::user::{log_users, User, Users};

/// All searches work similarly:
///         Loop through all users, if the current data is equal to the target data,
///         add the user to the `_ok` vector. Then print all users in the `_ok` vector.
///
/// The first search `by_name()` is explicitly written out with a `for` loop and `if ()`
/// statement. The others are shortened with the use of the iterator, inline foreach loops,
/// and closures (similar to lambda functions)

/// Searches in `_users` for user with `name`
pub fn by_name(_name: &str, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();

    for _u in &_users.0 {
        if _u.name == _name {
            _ok.push(&_u);
        }
    }

    log_users(&_ok);
}

/// Searches in `_users` for user with `username`
pub fn by_username(_username: &str, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();
    _users.0.iter().for_each(|_u| {
        if _username == _u.username {
            _ok.push(_u)
        }
    });
    log_users(&_ok)
}

/// Searches in `_users` for user with `email`
pub fn by_email(_email: &str, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();
    _users.0.iter().for_each(|_u| {
        if _email == _u.email {
            _ok.push(_u)
        }
    });
    log_users(&_ok)
}

/// Searches in `_users` for user with `id`
pub fn by_id(_id: i64, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();
    _users.0.iter().for_each(|_u| {
        if _id == _u.id {
            _ok.push(_u)
        }
    });
    log_users(&_ok)
}

/// Searches in `_users` for user with `age`
pub fn by_age(_age: i64, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();
    _users.0.iter().for_each(|_u| {
        if _age == _u.age {
            _ok.push(_u)
        }
    });
    log_users(&_ok)
}

/// logs all users
pub fn all(_users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();
    _users.0.iter().for_each(|_u| _ok.push(_u));
    log_users(&_ok)
}

/// Prints search menu options
pub fn search_options() {
    println!("Search by");
    println!();
    println!("1. Name");
    println!("2. Username");
    println!("3. Email");
    println!("4. Age");
    println!("5. ID");
    println!("6. Back");
    print!("{}", "> ".green());
}
