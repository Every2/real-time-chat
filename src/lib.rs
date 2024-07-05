use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>,) -> (StatusCode, Json<User>) {
    let user = User {
        username: payload.username.to_string(),
        email: payload.email.to_string(),
        password: payload.password.to_string(),
    };

    (StatusCode::CREATED, Json(user))
}