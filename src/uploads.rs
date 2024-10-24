use crate::query;

pub async fn get(token: &str, id: &str) {
    query::get(&format!("uploads/{}", id), token)
}

pub async fn create() {
    // TODO query::form function
    // query::form("uploads", token, &activity).await
}
