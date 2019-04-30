use crate::user::{User, log_user};

pub fn by_name(_name: &str, _users: &Vec<User>) {
    for _u in _users {
        if _u.name == _name {
            log_user(_u)
        }
    }
}

pub fn by_username(_username: &str, _users: &Vec<User>) {
    for _u in _users {
        if _u.username == _username {
            log_user(_u)
        }
    }
}

pub fn by_email(_email: &str, _users: &Vec<User>) {
    for _u in _users {
        if _u.email == _email {
            log_user(_u)
        }
    }
}

pub fn by_id(_id: i64, _users: &Vec<User>) {
    for _u in _users {
        if _u.id == _id {
            log_user(_u)
        }
    }
}

pub fn by_age(_age: i64, _users: &Vec<User>) {
    for _u in _users {
        if _u.age == _age {
            log_user(_u)
        }
    }
}