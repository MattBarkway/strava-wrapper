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

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = auth::get_token(
            123456,
            "",
            "",
        )
        .await
        .unwrap();
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

    // get upload
    // create upload
    // get athlete zones
    // get upload
    // get upload
    // get upload
    // get upload
    // get upload
}
