use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub login: String
}

pub fn users() -> Vec<User> {
    vec![
        User { login: "foobar".to_string() },
        User { login: "jonh.doe".to_string() },
        User { login: "jane.doe".to_string() }
    ]
}
