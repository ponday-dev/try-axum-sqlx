use crate::{
    app::{AppState, Context},
    models::CreateUserDto,
    usecases::UserUseCase,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn list_users<T>(State(state): State<AppState<T>>) -> impl IntoResponse
where
    T: Context,
{
    let users = state.user().list_users().await.expect("Query Error");

    Json(users)
}

pub async fn create_user<T>(
    State(state): State<AppState<T>>,
    Json(payload): Json<CreateUserDto>,
) -> impl IntoResponse
where
    T: Context,
{
    let user = state
        .user()
        .create_user(payload)
        .await
        .expect("Create Error");

    (StatusCode::CREATED, Json(user))
}
