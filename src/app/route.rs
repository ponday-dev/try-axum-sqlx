use crate::app::Context;
use crate::controllers::user::list_users;
use std::sync::Arc;

use axum::{routing::get, Router};

pub fn create_router<C: Context>(context: C) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .with_state(Arc::new(context))
}
