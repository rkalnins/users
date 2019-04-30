use crate::user::{User, Users, log_users};

pub fn by_name(_name: &str, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();

    for _u in &_users.0 {
        if _u.name == _name {
            _ok.push(&_u);
        }
    }

    log_users(&_ok);
}

pub fn by_username(_username: &str, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();

    for _u in &_users.0 {
        if _u.username == _username {
            _ok.push(&_u);
        }
    }

    log_users(&_ok);
}

pub fn by_email(_email: &str, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();

    for _u in &_users.0 {
        if _u.email == _email {
            _ok.push(&_u);
        }
    }

    log_users(&_ok);
}

pub fn by_id(_id: i64, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();

    for _u in &_users.0 {
        if _u.id == _id {
            _ok.push(&_u);
        }
    }

    log_users(&_ok);
}

pub fn by_age(_age: i64, _users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();

    for _u in &_users.0 {
        if _u.age == _age {
            _ok.push(&_u);
        }
    }

    log_users(&_ok);
}

pub fn all(_users: &Users) {
    let mut _ok: Vec<&User> = Vec::new();
    for _u in &_users.0 {
        _ok.push(&_u);
    }
    log_users(&_ok);
}