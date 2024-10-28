use crate::activities::ActivityEndpoints;

pub struct Client {
    token: String,
    pub activities: ActivityEndpoints,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
            activities: ActivityEndpoints::new(token.to_string()),
        }
    }
}
