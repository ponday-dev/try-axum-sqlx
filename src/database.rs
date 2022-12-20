use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct ConnectionPoolOptions {
    pub max_connections: u32,
    pub user: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: String,
}

pub async fn connect(options: &ConnectionPoolOptions) -> anyhow::Result<PgPool, sqlx::Error> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &options.user, &options.password, &options.host, &options.port, &options.database,
    );
    let pool = PgPoolOptions::new()
        .max_connections(options.max_connections)
        .connect(&database_url)
        .await;

    pool
}
