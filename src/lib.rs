pub mod activities;
pub mod athletes;
mod auth;
mod client;
pub mod clubs;
pub mod efforts;
pub mod gear;
mod query;
pub mod routes;
pub mod segments;
pub mod streams;
mod uploads;
mod models;
mod filters;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateActivity;
    use crate::query::{AfterCursor, Page, Sendable, PageSize, PerPage, Post, TimeFilter, ID, IncludeAllEfforts};

    #[tokio::test]
    async fn get_athlete() {
        let result = athletes::get(query::TOKEN).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_activity() {
        let result = activities::get(query::TOKEN, 12718749861).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_athlete_activities() {
        let result = athletes::activities(query::TOKEN).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_token() {
        let result = auth::get_token(123456, "", "").await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_comments() {
        let result = activities::comments(query::TOKEN, 12718749861)
            .await
            .unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_kudos() {
        let result = activities::kudos(query::TOKEN, 12718749861).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_laps() {
        let result = activities::laps(query::TOKEN, 12718749861).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn get_athlete_activity() {
        activities::get()
            .id(2)
            .include_all_efforts(false)
            .send(query::TOKEN)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_athlete_activities() {
        activities::list()
            .page(2)
            .per_page(5)
            .after(3)
            .before(1)
            .send(query::TOKEN)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_activity_comments() {
        activities::comments()
            .page(2)
            .page_size(5)
            .after_cursor(3)
            .send(query::TOKEN)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_activity() {
        let activity = CreateActivity {
            name: "Foo".to_string(),
            activity_type: None,
            sport_type: "bar".to_string(),
            start_date_local: "1234".to_string(),
            elapsed_time: 0,
            description: None,
            distance: None,
            trainer: None,
            commute: None,
        };
        activities::create(activity)
            .send(query::TOKEN)
            .await
            .unwrap();
    }

    // get upload
    // create upload
    // get athlete zones
    // get upload
    // get upload
    // get upload
    // get upload
    // get upload
}
