use leptos::config::LeptosOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pg_pool: PgPool,
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions, pg_pool: PgPool) -> Result<Self, sqlx::Error> {
        Ok(Self {
            leptos_options,
            pg_pool,
        })
    }
}
