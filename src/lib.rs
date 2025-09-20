mod auth;
mod query;
pub mod filters;
pub mod models;
pub mod endpoints;
pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::StravaAPI;
    use super::*;
    use crate::query::{
        Page, PerPage, Sendable,
    };

    #[tokio::test]
    async fn get_comments() {
        let api = StravaAPI::new(query::TOKEN);
        api.comments().get().page(1).send().await.unwrap();
    }
}
