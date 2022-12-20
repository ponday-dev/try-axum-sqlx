use crate::{
    app::{AppState, Context},
    usecases::UserUseCase,
};
use axum::{extract::State, response::IntoResponse, Json};

pub async fn list_users<T>(State(state): State<AppState<T>>) -> impl IntoResponse
where
    T: Context,
{
    let users = state.user().list_users().await.expect("Query Error");

    Json(users)
}
