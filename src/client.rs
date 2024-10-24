pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string(),
        }
    }
}
